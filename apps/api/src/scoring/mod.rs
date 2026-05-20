//! Risk signal derivation.
//!
//! Pure functions: vault metrics in, a `Vec<RiskSignal>` out. Each signal is
//! independent — there is intentionally no composite score. Thresholds are
//! named constants so they are easy to find and tune.

use crate::domain::{RiskLevel, RiskSignal};

// --- Liquidity thresholds (withdrawable liquidity as a share of TVL) -------
/// At or above this ratio, liquidity is a positive signal.
const LIQUIDITY_RATIO_HEALTHY: f64 = 0.20;
/// Below this ratio, liquidity is a caution signal.
const LIQUIDITY_RATIO_THIN: f64 = 0.05;

// --- TVL size bands (absolute USD) -----------------------------------------
/// At or above this TVL, the vault counts as large.
const TVL_LARGE_USD: f64 = 100_000_000.0;
/// At or above this TVL, the vault counts as mid-sized; below it, small.
const TVL_MID_USD: f64 = 10_000_000.0;

// --- Vault age cutoffs (days since creation) -------------------------------
/// At or above this age, the vault counts as established.
const AGE_ESTABLISHED_DAYS: f64 = 180.0;
/// At or above this age, the vault is past its earliest days; below it, young.
const AGE_YOUNG_DAYS: f64 = 30.0;

/// Numeric inputs to scoring, normalized from a provider response.
#[derive(Debug, Clone, Default)]
pub struct VaultMetrics {
    /// Total value locked, in USD.
    pub tvl_usd: Option<f64>,
    /// Withdrawable liquidity, in USD.
    pub liquidity_usd: Option<f64>,
    /// Age of the vault in days since its creation timestamp.
    pub age_days: Option<f64>,
    /// Curator display name, if the provider exposes one.
    pub curator_name: Option<String>,
}

/// Derive every risk signal for a vault, in a stable order.
pub fn risk_signals(metrics: &VaultMetrics) -> Vec<RiskSignal> {
    vec![
        liquidity_signal(metrics),
        tvl_size_signal(metrics),
        vault_age_signal(metrics),
        curator_signal(metrics),
    ]
}

/// Withdrawable liquidity relative to TVL.
fn liquidity_signal(metrics: &VaultMetrics) -> RiskSignal {
    let signal = |level, evidence: String| RiskSignal {
        label: "Liquidity".to_string(),
        level,
        evidence,
    };

    match (metrics.liquidity_usd, metrics.tvl_usd) {
        (Some(liquidity), Some(tvl)) if tvl > 0.0 => {
            let ratio = liquidity / tvl;
            let pct = ratio * 100.0;
            if ratio >= LIQUIDITY_RATIO_HEALTHY {
                signal(
                    RiskLevel::Positive,
                    format!("{pct:.1}% of TVL is withdrawable right now."),
                )
            } else if ratio >= LIQUIDITY_RATIO_THIN {
                signal(
                    RiskLevel::Neutral,
                    format!("{pct:.1}% of TVL is withdrawable right now."),
                )
            } else {
                signal(
                    RiskLevel::Caution,
                    format!("Only {pct:.1}% of TVL is withdrawable right now."),
                )
            }
        }
        _ => signal(
            RiskLevel::Neutral,
            "Liquidity data is unavailable for this vault.".to_string(),
        ),
    }
}

/// Absolute TVL bucketed into size bands.
fn tvl_size_signal(metrics: &VaultMetrics) -> RiskSignal {
    let signal = |level, evidence: String| RiskSignal {
        label: "TVL size".to_string(),
        level,
        evidence,
    };

    match metrics.tvl_usd {
        Some(tvl) if tvl >= TVL_LARGE_USD => signal(
            RiskLevel::Positive,
            format!("Large vault: ${} TVL.", millions(tvl)),
        ),
        Some(tvl) if tvl >= TVL_MID_USD => signal(
            RiskLevel::Neutral,
            format!("Mid-sized vault: ${} TVL.", millions(tvl)),
        ),
        Some(tvl) => signal(
            RiskLevel::Caution,
            format!("Small vault: ${} TVL.", millions(tvl)),
        ),
        None => signal(
            RiskLevel::Neutral,
            "TVL data is unavailable for this vault.".to_string(),
        ),
    }
}

/// Time since the vault was created.
fn vault_age_signal(metrics: &VaultMetrics) -> RiskSignal {
    let signal = |level, evidence: String| RiskSignal {
        label: "Vault age".to_string(),
        level,
        evidence,
    };

    match metrics.age_days {
        Some(days) if days >= AGE_ESTABLISHED_DAYS => signal(
            RiskLevel::Positive,
            format!("Established: created {} days ago.", days as i64),
        ),
        Some(days) if days >= AGE_YOUNG_DAYS => signal(
            RiskLevel::Neutral,
            format!("Created {} days ago.", days as i64),
        ),
        Some(days) => signal(
            RiskLevel::Caution,
            format!("Young vault: created only {} days ago.", days as i64),
        ),
        None => signal(
            RiskLevel::Neutral,
            "Creation date is unavailable for this vault.".to_string(),
        ),
    }
}

/// Whether the curator is a known, named entity.
fn curator_signal(metrics: &VaultMetrics) -> RiskSignal {
    let signal = |level, evidence: String| RiskSignal {
        label: "Curator".to_string(),
        level,
        evidence,
    };

    match metrics.curator_name.as_deref() {
        Some(name) if !name.trim().is_empty() => signal(
            RiskLevel::Positive,
            format!("Curated by a named entity: {name}."),
        ),
        _ => signal(
            RiskLevel::Caution,
            "Curator is not identified by the provider.".to_string(),
        ),
    }
}

/// Format a USD amount as a compact millions string (e.g. `465.3M`).
fn millions(usd: f64) -> String {
    format!("{:.1}M", usd / 1_000_000.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn metrics() -> VaultMetrics {
        VaultMetrics {
            tvl_usd: Some(50_000_000.0),
            liquidity_usd: Some(10_000_000.0),
            age_days: Some(90.0),
            curator_name: Some("Steakhouse Financial".to_string()),
        }
    }

    fn find<'a>(signals: &'a [RiskSignal], label: &str) -> &'a RiskSignal {
        signals
            .iter()
            .find(|s| s.label == label)
            .unwrap_or_else(|| panic!("missing signal {label}"))
    }

    #[test]
    fn liquidity_ratio_at_healthy_boundary_is_positive() {
        let m = VaultMetrics {
            tvl_usd: Some(100.0),
            liquidity_usd: Some(20.0),
            ..metrics()
        };
        assert_eq!(
            find(&risk_signals(&m), "Liquidity").level,
            RiskLevel::Positive
        );
    }

    #[test]
    fn liquidity_ratio_just_below_thin_boundary_is_caution() {
        let m = VaultMetrics {
            tvl_usd: Some(100.0),
            liquidity_usd: Some(4.99),
            ..metrics()
        };
        assert_eq!(
            find(&risk_signals(&m), "Liquidity").level,
            RiskLevel::Caution
        );
    }

    #[test]
    fn liquidity_in_mid_band_is_neutral() {
        let m = VaultMetrics {
            tvl_usd: Some(100.0),
            liquidity_usd: Some(10.0),
            ..metrics()
        };
        assert_eq!(
            find(&risk_signals(&m), "Liquidity").level,
            RiskLevel::Neutral
        );
    }

    #[test]
    fn missing_liquidity_is_neutral_and_unavailable() {
        let m = VaultMetrics {
            liquidity_usd: None,
            ..metrics()
        };
        let signal = find(&risk_signals(&m), "Liquidity").clone();
        assert_eq!(signal.level, RiskLevel::Neutral);
        assert!(signal.evidence.contains("unavailable"));
    }

    #[test]
    fn tvl_size_bands_at_boundaries() {
        let large = VaultMetrics {
            tvl_usd: Some(TVL_LARGE_USD),
            ..metrics()
        };
        let mid = VaultMetrics {
            tvl_usd: Some(TVL_MID_USD),
            ..metrics()
        };
        let small = VaultMetrics {
            tvl_usd: Some(TVL_MID_USD - 1.0),
            ..metrics()
        };
        assert_eq!(
            find(&risk_signals(&large), "TVL size").level,
            RiskLevel::Positive
        );
        assert_eq!(
            find(&risk_signals(&mid), "TVL size").level,
            RiskLevel::Neutral
        );
        assert_eq!(
            find(&risk_signals(&small), "TVL size").level,
            RiskLevel::Caution
        );
    }

    #[test]
    fn vault_age_bands_at_boundaries() {
        let old = VaultMetrics {
            age_days: Some(AGE_ESTABLISHED_DAYS),
            ..metrics()
        };
        let mid = VaultMetrics {
            age_days: Some(AGE_YOUNG_DAYS),
            ..metrics()
        };
        let young = VaultMetrics {
            age_days: Some(AGE_YOUNG_DAYS - 0.1),
            ..metrics()
        };
        assert_eq!(
            find(&risk_signals(&old), "Vault age").level,
            RiskLevel::Positive
        );
        assert_eq!(
            find(&risk_signals(&mid), "Vault age").level,
            RiskLevel::Neutral
        );
        assert_eq!(
            find(&risk_signals(&young), "Vault age").level,
            RiskLevel::Caution
        );
    }

    #[test]
    fn unknown_curator_is_caution() {
        let blank = VaultMetrics {
            curator_name: Some("   ".to_string()),
            ..metrics()
        };
        let absent = VaultMetrics {
            curator_name: None,
            ..metrics()
        };
        assert_eq!(
            find(&risk_signals(&blank), "Curator").level,
            RiskLevel::Caution
        );
        assert_eq!(
            find(&risk_signals(&absent), "Curator").level,
            RiskLevel::Caution
        );
    }

    #[test]
    fn known_curator_is_positive() {
        assert_eq!(
            find(&risk_signals(&metrics()), "Curator").level,
            RiskLevel::Positive
        );
    }

    #[test]
    fn every_vault_gets_four_signals() {
        assert_eq!(risk_signals(&metrics()).len(), 4);
    }
}
