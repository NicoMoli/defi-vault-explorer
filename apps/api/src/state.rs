//! Application state: the provider handle, the vault snapshot store, and the
//! single-flight gate that coordinates the two fetch trigger sources.
//!
//! Two locks, two jobs (ADR 005):
//! - the `RwLock` inside [`SnapshotStore`] guards the *data* — many readers,
//!   one brief writer per swap;
//! - the `fetch_gate` `Mutex` guards the *act of fetching* — at most one
//!   upstream call in flight across the whole process, so concurrent misses and
//!   the background timer never stampede Morpho.

use std::sync::Arc;

use tokio::sync::Mutex;

use crate::cache::SnapshotStore;
use crate::domain::Vault;
use crate::error::ApiError;
use crate::providers::VaultProvider;

/// Shared, clonable state handed to every route handler and to the background
/// refresh task.
#[derive(Clone)]
pub struct AppState {
    provider: Arc<dyn VaultProvider>,
    store: SnapshotStore<Vec<Vault>>,
    /// Single-flight gate: held across an upstream fetch so only one runs at a
    /// time. Guards an action, not data — hence `Mutex<()>`.
    fetch_gate: Arc<Mutex<()>>,
}

impl AppState {
    /// Wire a provider to a fresh, empty snapshot store.
    pub fn new(provider: Arc<dyn VaultProvider>) -> Self {
        Self {
            provider,
            store: SnapshotStore::new(),
            fetch_gate: Arc::new(Mutex::new(())),
        }
    }

    /// Return the vault list for a request.
    ///
    /// Serve whatever Snapshot is present, fresh *or* stale (ADR 005): staleness
    /// alone never triggers a fetch. Only an **empty** cache fills on demand,
    /// behind the single-flight gate with a double-checked re-read so concurrent
    /// cold misses collapse into one upstream call. A `503 upstream_unavailable`
    /// survives only when the cache is empty *and* the fetch fails.
    pub async fn vaults(&self) -> Result<Vec<Vault>, ApiError> {
        // Fast path: serve the present snapshot, holding no gate.
        if let Some(vaults) = self.store.snapshot() {
            return Ok(vaults);
        }

        // Empty cache: take the gate so at most one of us fetches.
        let _gate = self.fetch_gate.lock().await;

        // Double-checked read: someone may have filled the cache while we waited
        // for the gate. If so, serve it and skip the fetch.
        if let Some(vaults) = self.store.snapshot() {
            return Ok(vaults);
        }

        // Still empty and we hold the gate: this fetch is the single flight.
        self.fetch_and_store().await
    }

    /// Refresh the cache through the single-flight gate, replacing the snapshot
    /// on success. Used by the background task; shares the *same* gate as the
    /// on-demand path so the timer and a cold request can never double-fetch.
    pub async fn refresh(&self) -> Result<(), ApiError> {
        let _gate = self.fetch_gate.lock().await;
        self.fetch_and_store().await.map(|_| ())
    }

    /// Fetch from the provider and store the result. The caller must hold the
    /// `fetch_gate`. The gate (not the store's write lock) is what is held
    /// across the slow upstream call, so readers keep serving the old snapshot.
    async fn fetch_and_store(&self) -> Result<Vec<Vault>, ApiError> {
        let vaults = self.provider.fetch_vaults().await?;
        self.store.store(vaults.clone());
        Ok(vaults)
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
