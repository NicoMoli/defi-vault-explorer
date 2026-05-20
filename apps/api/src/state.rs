//! Application state: the provider handle plus the vault cache, and the
//! lazy-refresh logic that ties them together.

use std::sync::Arc;
use std::time::Duration;

use crate::cache::TtlCache;
use crate::domain::Vault;
use crate::error::ApiError;
use crate::providers::VaultProvider;

/// Shared, clonable state handed to every route handler.
#[derive(Clone)]
pub struct AppState {
    provider: Arc<dyn VaultProvider>,
    cache: TtlCache<Vec<Vault>>,
}

impl AppState {
    /// Wire a provider to a fresh cache with the given TTL.
    pub fn new(provider: Arc<dyn VaultProvider>, cache_ttl: Duration) -> Self {
        Self {
            provider,
            cache: TtlCache::new(cache_ttl),
        }
    }

    /// Return the vault list, refreshing lazily when the cache has expired.
    ///
    /// On a refresh failure the last successful snapshot is served if one
    /// exists — its `source.updated_at` still reflects its true age, so stale
    /// data is never presented as fresh. Only an empty cache surfaces the
    /// cold-start `upstream_unavailable` error.
    pub async fn vaults(&self) -> Result<Vec<Vault>, ApiError> {
        if let Some(vaults) = self.cache.fresh().await {
            return Ok(vaults);
        }

        match self.provider.fetch_vaults().await {
            Ok(vaults) => {
                self.cache.store(vaults.clone()).await;
                Ok(vaults)
            }
            Err(err) => match self.cache.snapshot().await {
                Some(stale) => {
                    tracing::warn!(%err, "serving stale vault snapshot after refresh failure");
                    Ok(stale)
                }
                None => Err(err),
            },
        }
    }

    /// Look up a single vault by address (case-insensitive) from the cache.
    pub async fn vault_by_address(&self, address: &str) -> Result<Vault, ApiError> {
        let vaults = self.vaults().await?;
        vaults
            .into_iter()
            .find(|vault| vault.address.eq_ignore_ascii_case(address))
            .ok_or_else(|| ApiError::NotFound(format!("vault {address}")))
    }
}
