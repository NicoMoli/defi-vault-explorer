//! Provider boundary: anything that can supply a list of vaults.
//!
//! Routes and state depend on the `VaultProvider` trait, never on a concrete
//! HTTP client, so tests can substitute an offline fake.

pub mod morpho;

use std::future::Future;
use std::pin::Pin;

use crate::domain::Vault;
use crate::error::ApiError;

/// The future returned by [`VaultProvider::fetch_vaults`].
///
/// Boxed so the trait stays object-safe and can live behind `Arc<dyn _>`.
pub type VaultFetch<'a> = Pin<Box<dyn Future<Output = Result<Vec<Vault>, ApiError>> + Send + 'a>>;

/// A source of Base Morpho vault data.
pub trait VaultProvider: Send + Sync {
    /// Fetch the current whitelisted vault list, already normalized and scored.
    fn fetch_vaults(&self) -> VaultFetch<'_>;
}
