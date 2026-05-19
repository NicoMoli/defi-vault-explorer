# Spec: Live Morpho Vault Data on Base

## Status

Draft. Review gate before any implementation.

## Relationship to Prior Docs

This spec is the second iteration. It builds on `001-initial-skeleton.md` and
answers several of its open questions. It supersedes parts of ADR
`docs/architecture/001-initial-skeleton.md` that assume mocked data.

## Objective

Replace mocked vault data with real Morpho Vault data on Base, and prove the
core product idea end to end: a user can open the app and compare real Base
Morpho vaults by transparent safety signals.

Validation question for this iteration: *does a signal-by-signal vault
comparison, backed by real data, look useful enough to keep building?*

## Decisions (Confirmed)

These were confirmed in a review session and are not open for this iteration.

1. **Chain:** Base only (`chainId: 8453`). Unchanged from iteration 001.
2. **Data source:** Morpho GraphQL API (`https://blue-api.morpho.org/graphql`).
   No API key required. Morpho's reported numbers are trusted for now; on-chain
   verification is a later iteration.
3. **Runtime data mode:** Live only. There is no mock runtime mode. Tests use
   recorded GraphQL JSON fixtures, never live network calls.
4. **Caching:** In-memory TTL cache (`Arc<RwLock<...>>`), TTL approximately
   60 seconds, lazy refresh on first request after expiry. No Redis, no
   database, no Docker in this iteration.
5. **Endpoints:** Implement `GET /vaults` and `GET /vaults/:address` only.
   Metrics and risk signals are embedded in the vault object, not separate
   routes.
6. **Vault set:** Whitelisted Base vaults, sorted by TVL descending, capped at
   a constant (~25).
7. **Risk signals:** Signal-by-signal only. No composite score. Approximately
   four independent signals (see below).
8. **Frontend scope:** Backend plus one minimal Next.js page rendering `/vaults`
   as a plain table. No filters, no detail page in this iteration. The
   `/vaults/:address` endpoint is built but not yet consumed by a page.
9. **Frontend/API wiring:** The page is an async Server Component that fetches
   the API directly on the server using an `API_URL` env var. No browser-side
   API calls, no Next route-handler proxy.
10. **Cold-start failure:** When the Morpho API fails and the cache is empty,
    `/vaults` returns HTTP 503 with a typed error body. The page renders an
    explicit error state. Stale data is never presented as fresh.

## Answers to 001 Open Questions

- *Which Morpho data source:* Morpho GraphQL API (decision 2).
- *Minimum freshness:* ~60s via cache TTL (decision 4).
- *Composite score vs breakdown:* Signal-by-signal breakdown only (decision 7).
- *Redis local vs hosted:* Neither yet; Redis deferred past this iteration.

## Data Source Details

Endpoint: `https://blue-api.morpho.org/graphql`.

The backend issues one GraphQL query to list whitelisted Base vaults ordered by
USD TVL, requesting at least: vault address, name, chain id, curator, asset
symbol and decimals, total assets, total assets in USD, net APY, fee,
liquidity, creation timestamp, and market allocations.

Note: exact GraphQL field and type names must be confirmed against the live
schema during implementation. The spec fixes intent, not field spelling.

`/vaults/:address` is served from the same cached list (lookup by address); it
does not issue a separate upstream query in this iteration.

## API Contract Changes

Relative to `docs/architecture/002-api-contract.md`:

- `/health`: `data_mode` becomes the constant `"live"` (was `"mock"`). The
  existing test asserting `"mock"` must be updated.
- Vault `source` object: `kind` becomes `"morpho-api"` (was `"mock"`).
  `updated_at` carries the cache refresh timestamp (RFC 3339 string), not null.
- All financial quantities still cross the boundary as strings. Unknown
  provider values are `null`.
- Error responses use a typed body: `{ "error": "<code>", "message": "<text>" }`.
  Cold-start upstream failure uses code `upstream_unavailable` with HTTP 503.

`002-api-contract.md` should be updated to reflect these once this spec is
approved.

## Risk Signals

Each signal is `{ label, level, evidence }` where `level` is one of
`positive`, `neutral`, `caution`. No composite score. Initial signals, all
derivable from the GraphQL response:

1. **Liquidity** — withdrawable liquidity as a share of TVL.
2. **TVL size** — absolute USD TVL bucketed into size bands.
3. **Vault age** — time since creation timestamp.
4. **Curator** — whether the curator is present/known vs unknown.

Each signal's `evidence` states the underlying number in plain language. Signal
thresholds are constants documented in the plan and easy to tune.

## Boundaries

- Always: present risk output as signals with evidence, never as advice.
- Always: surface source and freshness (`source.updated_at`) in API and UI.
- Always: keep money, share, and APY values decimal-safe (strings across the
  boundary; no `f64` round-tripping into responses).
- Always: keep the chain fixed to Base.
- Never: claim a vault is safe, recommended, or investment-grade.
- Never: serve stale data labeled as fresh; never serve fake/mock data at
  runtime.
- Ask first: add Redis, a database, a composite score, more chains, more
  endpoints, or a second data provider.

## Testing Strategy

Backend:

- Provider tests run against recorded GraphQL JSON fixtures, offline.
- Normalization tests: GraphQL shape to `VaultSummary`, including missing APY,
  low liquidity, young vault, unknown curator, multiple market exposures.
- Risk signal tests: each signal's level and evidence at threshold boundaries.
- Route tests: `/health`, `/vaults`, `/vaults/:address` (found and not found),
  and the 503 cold-start path.
- Cache tests: TTL expiry triggers refresh; lookup-by-address hits cache.

Frontend:

- The table page renders populated, empty, and error states.
- Basic accessibility check on the table.

## Success Criteria

- `/vaults` and `/vaults/:address` return real Base Morpho data via the
  GraphQL API, cached in memory with a ~60s TTL.
- Cold-start upstream failure yields a 503 typed error, surfaced as an explicit
  UI error state.
- The Next.js page lists real vaults with their risk signals in a table.
- Backend tests pass offline using fixtures.
- The iteration is reviewable: backend and frontend changes are coherent and
  the affected 001 docs are updated.

## Open Questions

1. Does the Morpho GraphQL API expose per-vault withdrawable liquidity
   directly, or must it be derived from market allocations? Resolved during
   implementation against the live schema.
2. Net APY units from the API: fraction (`0.0842`) or percent (`8.42`)?
   Normalization must pin this down.
3. Final value of the vault cap constant (~25) and the signal thresholds.

## Next Gate

Approve this spec and its plan, then convert the plan into tasks and implement.
