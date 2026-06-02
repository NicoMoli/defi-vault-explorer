# ADR 005: Background Refresh with On-Demand Fill

## Status

Accepted (design). Builds on ADR `004-live-morpho-data.md`, which introduced the
in-memory TTL cache with lazy, request-driven refresh.

## Context

ADR 004 served vaults from an `Arc<RwLock<Option<…>>>` TTL cache refreshed
*lazily*: on a cache miss, the handling request fetched from Morpho inline. That
left two problems the iteration explicitly parked as "still open":

1. **Cold-start latency.** The first request after expiry blocks on Morpho.
2. **Stampede (dogpile).** Nothing coordinates concurrent misses. When the TTL
   lapses, every simultaneous request independently calls `fetch_vaults()`, so N
   concurrent requests can fan out into N upstream calls.

This iteration is deliberately a Rust async-concurrency exercise. The goal is to
keep the cache warm proactively and make refresh behaviour correct under
concurrency, without taking on new infrastructure (still no Redis).

## Decision

A single **background refresh task** keeps the cache warm, with an **on-demand
fill** path for the empty-cache case, and **single-flight coalescing** so at most
one upstream fetch is ever in flight.

- **Read path — serve present, fill on empty.** A request serves whatever
  Snapshot is present, fresh *or* stale. Only an **empty** cache triggers a
  fetch-and-wait; the request then serves the result. A `503 upstream_unavailable`
  survives only as the honest case: the cache is empty *and* the fetch failed.
  Staleness alone never triggers a fetch — stale data is served as-is, its
  `source.updated_at` stating its true age (the transparency stance of ADR 004).

- **Background task is the steady-state writer.** A spawned task loops on a
  computed delay, refreshing the cache so that in normal operation the cache is
  never empty and the on-demand path effectively never fires.

- **Single-flight gate across both trigger sources.** One shared
  `tokio::sync::Mutex` enforces "at most one fetch in flight in the whole
  process." Concurrent cache-miss requests: the first acquires the gate and
  fetches; the rest block, then re-check the cache (double-checked locking) and
  find it filled, so they skip the fetch and serve it. The background task takes
  the *same* gate, so the timer and a cold request can never double-fetch.

- **`RwLock` store, not `watch`.** The Snapshot lives behind an `RwLock`; the
  fetch gate is a separate `Mutex`. Single-writer is not enforced by types
  because there are deliberately two trigger sources coordinated by the gate.

- **TTL collapses into a refresh interval.** There is no read-side freshness
  gate anymore, so the old `cache_ttl` (a read gate) is renamed to
  `refresh_interval` (`REFRESH_INTERVAL_SECS`), owned by the background task.

- **Failure handling: keep last, log, back off.** A failed refresh keeps the
  last good Snapshot and logs a warning. Repeated failures back off
  exponentially — base = `refresh_interval` (~60s), ×2 per failure, capped at
  ~10 min, reset to base on the first success. No jitter (single instance). This
  uses `tokio::time::sleep(delay)` with `delay` recomputed from the outcome,
  rather than a fixed-period `tokio::time::interval`.

- **Graceful lifecycle.** The task is spawned in `main` with its `JoinHandle`
  retained. A shared `tokio_util::sync::CancellationToken` signals shutdown; the
  task loop is `select!` over `token.cancelled()` and the refresh `sleep`, so it
  stops promptly on the same signal that drives `axum::serve(...)
  .with_graceful_shutdown(...)`. On shutdown: cancel the token, then `await` the
  handle so an in-flight refresh finishes rather than being killed mid-fetch.

## Alternatives considered

- **Pure read-only read path (503 on cold start).** Initially favoured: the
  background task as the *only* writer, requests never fetching, so the stampede
  is impossible by construction. Rejected because it makes the API return 503 on
  cold start instead of just fetching — un-API-like for no real gain, since the
  stampede is equally solved by coalescing while *keeping* on-demand fill.

- **`tokio::sync::watch` for storage.** Attractive as the canonical
  publish-to-many-readers primitive and would make single-writer structural. The
  tokio docs show `watch` reads take a read lock on an internal `RwLock`
  ("Outstanding borrows hold a read lock … long-lived borrows could cause the
  producer half to block"), so its read mechanics are identical to a plain
  `RwLock`; its only distinguishing feature is change-notification
  (`changed().await`), which nothing in this iteration consumes. Rejected as a
  primitive chosen for an unused feature. Reserved for when something genuinely
  waits on changes (readiness gate, live push to the UI).

- **`arc-swap::ArcSwap`.** Truly lock-free reads, but the performance is
  irrelevant at ~25 vaults and one refresh/minute, and it adds a dependency for
  no functional gain. Rejected as learning-for-its-own-sake.

- **Block startup until the first refresh.** Gives a clean "ready ⇒ warm"
  guarantee, but couples process boot to Morpho being reachable, turning an
  upstream outage into a boot failure. Rejected; on-demand fill makes it
  unnecessary.

- **Faster-retry backoff (shrink the staleness window).** Rejected: staleness is
  explicitly acceptable because Freshness is reported honestly, so shaving the
  window adds complexity for little value. Backoff instead grows to spare a
  struggling upstream and cut log noise.

## Consequences

- Two fetch trigger sources (timer + on-demand) now exist, coordinated by one
  gate. The lock story is more involved than a single-writer model — the
  double-checked re-read on the miss path must be correct — but the system
  behaves like a normal API while remaining stampede-safe.
- The read path no longer depends on a TTL; `TtlCache::fresh()` and its `ttl`
  field are removed or repurposed, and the `cache_ttl` config / `CACHE_TTL_SECS`
  env var are renamed to `refresh_interval` / `REFRESH_INTERVAL_SECS`.
  `.env.example` updates accordingly.
- New dependency: `tokio_util` (for `CancellationToken`).
- Shutdown is now explicit: the background task and the HTTP server stop on the
  same signal, and an in-flight refresh is allowed to complete.

## Still Open

- On-chain verification of Morpho's reported numbers (carried over from ADR 004).
- Whether to expose readiness ("cache warm") distinctly from liveness — would
  reintroduce a use for `watch`-style change notification.
- Filters, a vault detail page, more chains, or a second data provider — each
  still gated to a later iteration.
