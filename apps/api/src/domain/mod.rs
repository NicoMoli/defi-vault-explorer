//! Domain models shared across providers, scoring, and routes.
//!
//! All financial quantities cross the API boundary as strings to stay
//! decimal-safe; unknown provider values are represented as `null`.

use serde::{Deserialize, Serialize};

/// A vault as presented by the API: identity, headline numbers, and the
/// transparent risk signals derived from them.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Vault {
    /// Checksummed vault contract address.
    pub address: String,
    /// Human-readable vault name from the provider.
    pub name: String,
    /// Lending protocol; always `"morpho"` for this iteration.
    pub protocol: String,
    /// Chain slug; always `"base"` for this iteration.
    pub chain: String,
    /// Curator display name, or `null` when the provider exposes none.
    pub curator: Option<String>,
    /// Underlying asset symbol (e.g. `"USDC"`).
    pub asset_symbol: String,
    /// Total value locked in USD, as a decimal string.
    pub tvl_usd: Option<String>,
    /// Net APY expressed as a percentage string (e.g. `"4.86"`).
    pub net_apy_percent: Option<String>,
    /// Withdrawable liquidity in USD, as a decimal string.
    pub liquidity_usd: Option<String>,
    /// Signal-by-signal risk context. Never a composite score.
    pub risk_signals: Vec<RiskSignal>,
    /// Where the data came from and when it was last refreshed.
    pub source: Source,
}

/// One transparent risk observation. The `evidence` states the underlying
/// number in plain language; this is context, never investment advice.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RiskSignal {
    pub label: String,
    pub level: RiskLevel,
    pub evidence: String,
}

/// Direction of a risk signal. There is intentionally no numeric score.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum RiskLevel {
    Positive,
    Neutral,
    Caution,
}

/// Provenance and freshness metadata attached to every vault.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Source {
    /// Data origin; `"morpho-api"` for live GraphQL data.
    pub kind: String,
    /// RFC 3339 timestamp of the cache refresh that produced this data.
    pub updated_at: Option<String>,
}

impl Source {
    /// Build a live-data source stamped with the given refresh timestamp.
    pub fn morpho_api(updated_at: impl Into<String>) -> Self {
        Self {
            kind: "morpho-api".to_string(),
            updated_at: Some(updated_at.into()),
        }
    }
}
