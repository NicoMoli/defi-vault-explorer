//! Environment-backed runtime configuration.

use std::time::Duration;

/// Default address the HTTP server binds to.
const DEFAULT_BIND_ADDRESS: &str = "127.0.0.1:4000";
/// Default Morpho GraphQL endpoint.
const DEFAULT_MORPHO_API_URL: &str = "https://blue-api.morpho.org/graphql";
/// Default cache time-to-live, in seconds (~1 minute, per spec).
const DEFAULT_CACHE_TTL_SECS: u64 = 60;
/// Default cap on the number of vaults fetched and served.
const DEFAULT_VAULT_CAP: usize = 25;

/// Resolved configuration for one process run.
#[derive(Debug, Clone)]
pub struct Config {
    /// `host:port` the server binds to.
    pub bind_address: String,
    /// Morpho GraphQL endpoint the provider queries.
    pub morpho_api_url: String,
    /// How long a cached vault list is considered fresh.
    pub cache_ttl: Duration,
    /// Maximum number of vaults to request and serve.
    pub vault_cap: usize,
}

impl Config {
    /// Build config from environment variables, falling back to defaults.
    pub fn from_env() -> Self {
        let bind_address =
            std::env::var("API_BIND_ADDRESS").unwrap_or_else(|_| DEFAULT_BIND_ADDRESS.to_string());
        let morpho_api_url =
            std::env::var("MORPHO_API_URL").unwrap_or_else(|_| DEFAULT_MORPHO_API_URL.to_string());
        let cache_ttl = std::env::var("CACHE_TTL_SECS")
            .ok()
            .and_then(|raw| raw.parse().ok())
            .map(Duration::from_secs)
            .unwrap_or_else(|| Duration::from_secs(DEFAULT_CACHE_TTL_SECS));
        let vault_cap = std::env::var("VAULT_CAP")
            .ok()
            .and_then(|raw| raw.parse().ok())
            .unwrap_or(DEFAULT_VAULT_CAP);

        Self {
            bind_address,
            morpho_api_url,
            cache_ttl,
            vault_cap,
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            bind_address: DEFAULT_BIND_ADDRESS.to_string(),
            morpho_api_url: DEFAULT_MORPHO_API_URL.to_string(),
            cache_ttl: Duration::from_secs(DEFAULT_CACHE_TTL_SECS),
            vault_cap: DEFAULT_VAULT_CAP,
        }
    }
}
