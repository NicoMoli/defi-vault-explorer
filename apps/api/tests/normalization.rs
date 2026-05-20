//! Offline normalization tests: recorded GraphQL JSON in, domain vaults out.

use defi_vault_explorer_api::domain::{RiskLevel, Vault};
use defi_vault_explorer_api::providers::morpho::{GraphqlResponse, normalize};

/// Fixed reference time for deterministic vault-age math (2025-06-15).
const NOW_UNIX: i64 = 1_750_000_000;
const UPDATED_AT: &str = "2025-06-15T00:00:00+00:00";

fn cases() -> Vec<Vault> {
    let raw = include_str!("fixtures/morpho-vaults-cases.json");
    let response: GraphqlResponse =
        serde_json::from_str(raw).expect("cases fixture should deserialize");
    normalize(response, UPDATED_AT, NOW_UNIX).expect("cases fixture should normalize")
}

fn vault<'a>(vaults: &'a [Vault], name: &str) -> &'a Vault {
    vaults
        .iter()
        .find(|v| v.name == name)
        .unwrap_or_else(|| panic!("missing vault {name}"))
}

fn signal_level(vault: &Vault, label: &str) -> RiskLevel {
    vault
        .risk_signals
        .iter()
        .find(|s| s.label == label)
        .unwrap_or_else(|| panic!("missing signal {label}"))
        .level
}

#[test]
fn normalizes_every_vault_in_the_fixture() {
    let vaults = cases();
    assert_eq!(vaults.len(), 6);
    for v in &vaults {
        assert_eq!(v.protocol, "morpho");
        assert_eq!(v.chain, "base");
        assert_eq!(v.source.kind, "morpho-api");
        assert_eq!(v.source.updated_at.as_deref(), Some(UPDATED_AT));
        assert_eq!(v.risk_signals.len(), 4);
    }
}

#[test]
fn normal_vault_has_string_money_and_percent_apy() {
    let vaults = cases();
    let v = vault(&vaults, "Normal Vault");
    assert_eq!(v.tvl_usd.as_deref(), Some("50000000.00"));
    assert_eq!(v.net_apy_percent.as_deref(), Some("5.12"));
    assert_eq!(v.liquidity_usd.as_deref(), Some("50000000.00"));
    assert_eq!(v.curator.as_deref(), Some("Steakhouse Financial"));
    assert_eq!(signal_level(v, "Liquidity"), RiskLevel::Positive);
    assert_eq!(signal_level(v, "TVL size"), RiskLevel::Neutral);
    assert_eq!(signal_level(v, "Vault age"), RiskLevel::Positive);
    assert_eq!(signal_level(v, "Curator"), RiskLevel::Positive);
}

#[test]
fn missing_apy_becomes_null() {
    let vaults = cases();
    assert_eq!(vault(&vaults, "Missing APY Vault").net_apy_percent, None);
}

#[test]
fn low_liquidity_vault_flags_caution() {
    let vaults = cases();
    let v = vault(&vaults, "Low Liquidity Vault");
    assert_eq!(v.liquidity_usd.as_deref(), Some("1000000.00"));
    assert_eq!(signal_level(v, "Liquidity"), RiskLevel::Caution);
}

#[test]
fn young_vault_flags_caution() {
    let vaults = cases();
    assert_eq!(
        signal_level(vault(&vaults, "Young Vault"), "Vault age"),
        RiskLevel::Caution
    );
}

#[test]
fn unknown_curator_is_null_and_caution() {
    let vaults = cases();
    let v = vault(&vaults, "Unknown Curator Vault");
    assert_eq!(v.curator, None);
    assert_eq!(signal_level(v, "Curator"), RiskLevel::Caution);
}

#[test]
fn multi_allocation_liquidity_sums_per_market_minimums() {
    let vaults = cases();
    let v = vault(&vaults, "Multi Allocation Vault");
    // min(10M,8M) + min(20M,50M) + min(5M,0) = 8M + 20M + 0 = 28M.
    assert_eq!(v.liquidity_usd.as_deref(), Some("28000000.00"));
    assert_eq!(signal_level(v, "Liquidity"), RiskLevel::Positive);
}

#[test]
fn recorded_live_response_normalizes_without_panic() {
    let raw = include_str!("fixtures/morpho-vaults-base.json");
    let response: GraphqlResponse =
        serde_json::from_str(raw).expect("recorded fixture should deserialize");
    let vaults = normalize(response, UPDATED_AT, NOW_UNIX).expect("recorded fixture normalizes");
    assert!(!vaults.is_empty());
    assert!(vaults.iter().all(|v| v.protocol == "morpho"));
}

#[test]
fn graphql_errors_surface_as_upstream_unavailable() {
    let raw = r#"{ "errors": [{ "message": "schema exploded" }] }"#;
    let response: GraphqlResponse = serde_json::from_str(raw).expect("error envelope parses");
    assert!(normalize(response, UPDATED_AT, NOW_UNIX).is_err());
}
