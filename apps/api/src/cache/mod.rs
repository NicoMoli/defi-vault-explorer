//! A minimal in-memory TTL cache.
//!
//! One value, guarded by an async `RwLock`. Entries past their TTL are still
//! readable as a stale snapshot — callers decide whether to use them — but a
//! `fresh` read only returns a value within the TTL window.

use std::sync::Arc;
use std::time::{Duration, Instant};

use tokio::sync::RwLock;

/// A cached value together with the instant it was stored.
#[derive(Debug, Clone)]
struct Cached<T> {
    value: T,
    fetched_at: Instant,
}

/// A clonable handle to a single TTL-guarded value.
#[derive(Debug, Clone)]
pub struct TtlCache<T> {
    ttl: Duration,
    slot: Arc<RwLock<Option<Cached<T>>>>,
}

impl<T: Clone> TtlCache<T> {
    /// Create an empty cache with the given time-to-live.
    pub fn new(ttl: Duration) -> Self {
        Self {
            ttl,
            slot: Arc::new(RwLock::new(None)),
        }
    }

    /// Return the cached value only if it is still within its TTL.
    pub async fn fresh(&self) -> Option<T> {
        let guard = self.slot.read().await;
        guard
            .as_ref()
            .filter(|cached| cached.fetched_at.elapsed() < self.ttl)
            .map(|cached| cached.value.clone())
    }

    /// Return whatever is cached, fresh or stale, if anything.
    pub async fn snapshot(&self) -> Option<T> {
        let guard = self.slot.read().await;
        guard.as_ref().map(|cached| cached.value.clone())
    }

    /// Replace the cached value and reset its TTL window.
    pub async fn store(&self, value: T) {
        let mut guard = self.slot.write().await;
        *guard = Some(Cached {
            value,
            fetched_at: Instant::now(),
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn fresh_returns_value_within_ttl() {
        let cache = TtlCache::new(Duration::from_secs(60));
        assert_eq!(cache.fresh().await, None);

        cache.store(vec![1, 2, 3]).await;
        assert_eq!(cache.fresh().await, Some(vec![1, 2, 3]));
    }

    #[tokio::test]
    async fn expired_entry_is_not_fresh_but_is_a_snapshot() {
        let cache = TtlCache::new(Duration::ZERO);
        cache.store(vec![1, 2, 3]).await;

        // A zero TTL means the entry is stale the instant it is stored.
        assert_eq!(cache.fresh().await, None);
        assert_eq!(cache.snapshot().await, Some(vec![1, 2, 3]));
    }

    #[tokio::test]
    async fn store_refreshes_the_ttl_window() {
        let cache = TtlCache::new(Duration::from_secs(60));
        cache.store(vec![1]).await;
        cache.store(vec![2]).await;
        assert_eq!(cache.fresh().await, Some(vec![2]));
    }
}
