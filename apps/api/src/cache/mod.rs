//! A minimal in-memory snapshot store.
//!
//! One value, guarded by a `std::sync::RwLock`. Many readers can hold the
//! snapshot concurrently; a write briefly takes the exclusive guard to swap the
//! value. There is no freshness gate here anymore (ADR 005): the store hands
//! back whatever is present, and the value's own `source.updated_at` states its
//! age.
//!
//! ## Why `std::sync::RwLock` and not `tokio::sync::RwLock`?
//!
//! The Tokio docs are explicit: prefer the std lock in async code, and reach
//! for the async lock *only* when a guard must be held across an `.await`.
//! Here the critical sections are tiny and fully synchronous — `snapshot`
//! clones the value and drops the guard in one expression; `store` swaps and
//! drops. No guard ever crosses an `.await`, so the std lock is the lighter,
//! preferred choice and these methods need not be `async`.
//!
//! (The single-flight `Mutex` in `state.rs` is the opposite case — it *is* held
//! across the upstream fetch's `.await`, so that one must be `tokio::sync`.)

use std::sync::{Arc, RwLock};

/// A clonable handle to a single shared value behind an `RwLock`.
///
/// Cloning shares the same underlying slot (`Arc`), so every handle sees the
/// same writes.
#[derive(Debug, Clone, Default)]
pub struct SnapshotStore<T> {
    slot: Arc<RwLock<Option<T>>>,
}

impl<T: Clone> SnapshotStore<T> {
    /// Create an empty store.
    pub fn new() -> Self {
        Self {
            slot: Arc::new(RwLock::new(None)),
        }
    }

    /// Return whatever is stored, if anything. Clones the value under a
    /// short-lived read guard that is dropped before returning.
    pub fn snapshot(&self) -> Option<T> {
        self.slot.read().expect("snapshot lock poisoned").clone()
    }

    /// Replace the stored value. The write guard is held only for the swap,
    /// never across an upstream fetch.
    pub fn store(&self, value: T) {
        *self.slot.write().expect("snapshot lock poisoned") = Some(value);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_store_has_no_snapshot() {
        let store: SnapshotStore<Vec<i32>> = SnapshotStore::new();
        assert_eq!(store.snapshot(), None);
    }

    #[test]
    fn store_then_snapshot_returns_the_value() {
        let store = SnapshotStore::new();
        store.store(vec![1, 2, 3]);
        assert_eq!(store.snapshot(), Some(vec![1, 2, 3]));
    }

    #[test]
    fn store_overwrites_the_previous_value() {
        let store = SnapshotStore::new();
        store.store(vec![1]);
        store.store(vec![2]);
        assert_eq!(store.snapshot(), Some(vec![2]));
    }

    #[test]
    fn clones_share_one_slot() {
        let a = SnapshotStore::new();
        let b = a.clone();
        a.store(vec![42]);
        assert_eq!(b.snapshot(), Some(vec![42]));
    }
}
