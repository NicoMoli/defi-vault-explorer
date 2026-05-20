//! Morpho GraphQL provider: one query, plus normalization to domain models.
//!
//! The GraphQL schema is verified against the live API; field spelling here
//! matches `https://blue-api.morpho.org/graphql` as of 2026-05. Recorded JSON
//! fixtures under `tests/fixtures/` exercise normalization offline.

use chrono::Utc;
use serde::Deserialize;

use super::{VaultFetch, VaultProvider};
use crate::domain::{Source, Vault};
use crate::error::ApiError;
use crate::scoring::{self, VaultMetrics};

/// Base chain id; the only chain this iteration queries.
const BASE_CHAIN_ID: i64 = 8453;
/// Seconds in a day, for age math.
const SECONDS_PER_DAY: f64 = 86_400.0;

/// Live provider backed by the Morpho GraphQL API.
pub struct MorphoProvider {
    client: reqwest::Client,
    api_url: String,
    vault_cap: usize,
}

impl MorphoProvider {
    /// Build a provider targeting `api_url`, fetching at most `vault_cap` vaults.
    pub fn new(api_url: impl Into<String>, vault_cap: usize) -> Self {
        Self {
            client: reqwest::Client::new(),
            api_url: api_url.into(),
            vault_cap,
        }
    }

    /// The GraphQL query for whitelisted Base vaults, ordered by USD TVL.
    fn query(&self) -> String {
        format!(
            r#"{{
  vaults(first: {cap}, where: {{ chainId_in: [{chain}], listed: true }}, orderBy: TotalAssetsUsd, orderDirection: Desc) {{
    items {{
      address
      name
      creationTimestamp
      asset {{ symbol decimals }}
      chain {{ id network }}
      state {{
        totalAssetsUsd
        netApy
        fee
        curator
        curators {{ name }}
        allocation {{
          supplyAssetsUsd
          market {{ marketId state {{ liquidityAssetsUsd }} }}
        }}
      }}
    }}
  }}
}}"#,
            cap = self.vault_cap,
            chain = BASE_CHAIN_ID,
        )
    }
}

impl VaultProvider for MorphoProvider {
    fn fetch_vaults(&self) -> VaultFetch<'_> {
        Box::pin(async move {
            let request = serde_json::json!({ "query": self.query() });
            let response = self
                .client
                .post(&self.api_url)
                .json(&request)
                .send()
                .await
                .map_err(|err| ApiError::upstream_unavailable(err.to_string()))?;

            let parsed: GraphqlResponse = response
                .json()
                .await
                .map_err(|err| ApiError::upstream_unavailable(err.to_string()))?;

            let now = Utc::now();
            normalize(parsed, &now.to_rfc3339(), now.timestamp())
        })
    }
}

// --- GraphQL response shapes ------------------------------------------------

/// Top-level GraphQL envelope.
#[derive(Debug, Deserialize)]
pub struct GraphqlResponse {
    data: Option<ResponseData>,
    errors: Option<Vec<GraphqlError>>,
}

#[derive(Debug, Deserialize)]
struct GraphqlError {
    message: String,
}

#[derive(Debug, Deserialize)]
struct ResponseData {
    vaults: VaultList,
}

#[derive(Debug, Deserialize)]
struct VaultList {
    items: Vec<RawVault>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct RawVault {
    address: String,
    name: String,
    creation_timestamp: Option<i64>,
    asset: RawAsset,
    state: Option<RawState>,
}

#[derive(Debug, Deserialize)]
struct RawAsset {
    symbol: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct RawState {
    total_assets_usd: Option<f64>,
    net_apy: Option<f64>,
    curators: Option<Vec<RawCurator>>,
    allocation: Option<Vec<RawAllocation>>,
}

#[derive(Debug, Deserialize)]
struct RawCurator {
    name: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct RawAllocation {
    supply_assets_usd: Option<f64>,
    market: Option<RawMarket>,
}

#[derive(Debug, Deserialize)]
struct RawMarket {
    state: Option<RawMarketState>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct RawMarketState {
    liquidity_assets_usd: Option<f64>,
}

// --- Normalization ----------------------------------------------------------

/// Map a parsed GraphQL response to scored domain vaults.
///
/// `updated_at` is the RFC 3339 refresh timestamp stamped onto each vault's
/// source; `now_unix` is the reference point for vault-age math.
pub fn normalize(
    response: GraphqlResponse,
    updated_at: &str,
    now_unix: i64,
) -> Result<Vec<Vault>, ApiError> {
    if let Some(errors) = response.errors.filter(|errors| !errors.is_empty()) {
        let detail = errors
            .into_iter()
            .map(|err| err.message)
            .collect::<Vec<_>>()
            .join("; ");
        return Err(ApiError::upstream_unavailable(format!(
            "GraphQL errors: {detail}"
        )));
    }

    let data = response
        .data
        .ok_or_else(|| ApiError::upstream_unavailable("GraphQL response had no data"))?;

    Ok(data
        .items()
        .map(|raw| normalize_vault(raw, updated_at, now_unix))
        .collect())
}

impl ResponseData {
    fn items(self) -> impl Iterator<Item = RawVault> {
        self.vaults.items.into_iter()
    }
}

/// Normalize a single raw vault, deriving its metrics and risk signals.
fn normalize_vault(raw: RawVault, updated_at: &str, now_unix: i64) -> Vault {
    let state = raw.state;

    let tvl_usd = state.as_ref().and_then(|s| s.total_assets_usd);
    let net_apy = state.as_ref().and_then(|s| s.net_apy);
    let liquidity_usd = state.as_ref().and_then(withdrawable_liquidity);
    let curator_name = state.as_ref().and_then(curator_name);
    let age_days = raw
        .creation_timestamp
        .map(|created| (now_unix - created) as f64 / SECONDS_PER_DAY);

    let metrics = VaultMetrics {
        tvl_usd,
        liquidity_usd,
        age_days,
        curator_name: curator_name.clone(),
    };

    Vault {
        address: raw.address,
        name: raw.name,
        protocol: "morpho".to_string(),
        chain: "base".to_string(),
        curator: curator_name,
        asset_symbol: raw.asset.symbol,
        tvl_usd: tvl_usd.map(usd_string),
        net_apy_percent: net_apy.map(|apy| format!("{:.2}", apy * 100.0)),
        liquidity_usd: liquidity_usd.map(usd_string),
        risk_signals: scoring::risk_signals(&metrics),
        source: Source::morpho_api(updated_at),
    }
}

/// Withdrawable liquidity: per market, the vault can pull out at most what it
/// has supplied and at most what the market currently holds free.
///
/// TODO(understand): this value is *derived*, not reported by Morpho. The
/// GraphQL schema exposes no per-vault `liquidityUsd`, so we approximate it as
/// `Σ min(vault supply in market, market free liquidity)`. Open questions to
/// resolve before trusting it as a headline metric:
///   - A market's free liquidity is shared across every vault supplying it;
///     summing per-vault minimums can overstate what one vault could actually
///     withdraw if others race to exit the same market.
///   - It ignores Morpho's public allocator / reallocatable liquidity, which
///     can move liquidity between markets on withdrawal.
///   - It is not the same as instant on-chain withdrawable assets.
///
/// See spec `002-live-morpho-base.md` open question #1 and ADR 004.
fn withdrawable_liquidity(state: &RawState) -> Option<f64> {
    let allocations = state.allocation.as_ref()?;
    if allocations.is_empty() {
        return None;
    }
    let total = allocations
        .iter()
        .filter_map(|alloc| {
            let supplied = alloc.supply_assets_usd?;
            let market_free = alloc
                .market
                .as_ref()?
                .state
                .as_ref()?
                .liquidity_assets_usd?;
            Some(supplied.min(market_free))
        })
        .sum();
    Some(total)
}

/// First non-empty curator name, if any.
fn curator_name(state: &RawState) -> Option<String> {
    state
        .curators
        .as_ref()?
        .iter()
        .filter_map(|curator| curator.name.as_ref())
        .find(|name| !name.trim().is_empty())
        .cloned()
}

/// Render a USD amount as a fixed two-decimal string for the API boundary.
fn usd_string(amount: f64) -> String {
    format!("{amount:.2}")
}
