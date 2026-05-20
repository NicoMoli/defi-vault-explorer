# ADR 004: Live Morpho Vault Data on Base

## Status

Accepted. Implements spec `docs/specs/002-live-morpho-base.md`. Supersedes the
mocked-data assumptions of ADR `001-initial-skeleton.md`.

## Context

The skeleton (ADR 001) served mocked vault data and left three questions open:
which Morpho data source to trust, whether caching starts on Redis, and whether
risk is a composite score or a breakdown. Iteration 002 needs real Base Morpho
vault data end to end without taking on premature infrastructure.

## Decision

- **Data source.** The Morpho GraphQL API (`https://blue-api.morpho.org/graphql`,
  no API key). One query lists whitelisted Base vaults (`chainId 8453`,
  `listed: true`) ordered by USD TVL, capped at 25.
- **Live only.** There is no mock runtime mode. Tests run offline against
  recorded GraphQL JSON fixtures under `apps/api/tests/fixtures/`.
- **In-memory cache, not Redis.** A single `Arc<RwLock<Option<…>>>` TTL cache
  (~60s) with lazy refresh. Redis stays deferred. On a refresh failure a prior
  snapshot is still served — its `source.updated_at` reflects its true age, so
  stale data is never labeled fresh. Only an empty cache yields a cold-start
  `upstream_unavailable` (HTTP 503).
- **Risk signals, not a score.** Four independent signals — liquidity, TVL
  size, vault age, curator — each `{ label, level, evidence }`. No composite
  score. Thresholds are named constants in `scoring/`.
- **Withdrawable liquidity is derived.** The GraphQL schema exposes no per-vault
  liquidity field, so it is summed per market as
  `min(vault supply in market, market free liquidity)`.
- **Decimal safety.** Money/APY values cross the API boundary as strings; net
  APY is reported as a percentage string. Unknown provider values are `null`.
- **Frontend wiring.** The home page is an async Server Component fetching
  `${API_URL}/vaults` directly on the server. No browser-side calls, no proxy.

## Consequences

- The backend now grows a module layout (`config`, `error`, `state`, `cache`,
  `domain`, `providers`, `scoring`, `routes`) and is split into a lib + bin so
  integration tests can exercise it offline.
- GraphQL schema drift is a live risk: Morpho deprecated `whitelisted` and
  `Market.uniqueKey` in favor of `listed` and `marketId`. The provider uses the
  current field names; fixtures are recorded from real responses.
- Cold-start latency: the first request after expiry blocks on Morpho.
  Acceptable for this iteration; background refresh is a documented later step.
- New dependencies: `reqwest` (rustls) and `chrono`.

## Still Open

- Background/proactive cache refresh to remove cold-start latency.
- On-chain verification of Morpho's reported numbers.
- Filters, a vault detail page, more chains, or a second data provider — each
  gated per the spec's boundaries.
