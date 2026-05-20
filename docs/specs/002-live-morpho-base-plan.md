# Plan: Live Morpho Vault Data on Base

## Spec

This plan implements `docs/specs/002-live-morpho-base.md`.

## Current Gate

Status: approved and implemented. The three open decisions below were resolved
with the documented defaults — vault cap 25, the risk thresholds in
`scoring/mod.rs`, and a separate ADR `004`.

## Assumptions

1. The Morpho GraphQL API is reachable without authentication.
2. The current backend is the `001` skeleton: `apps/api/src/main.rs` with only
   a `/health` route and no module structure yet.
3. `apps/web` is still the default Next.js scaffold.
4. The exact Morpho GraphQL schema is verified during implementation; the plan
   names intent, not final field strings.

## Components

### Backend module structure

Grow `apps/api/src` from a single `main.rs` into the structure the `001` spec
anticipated, adding only what this iteration needs:

```txt
apps/api/src/
  main.rs        Server startup, state wiring.
  config.rs      Env-backed config (bind address, API_URL, cache TTL).
  error.rs       ApiError type and HTTP response mapping.
  state.rs       AppState: provider handle + cache.
  routes/        health, vaults handlers.
  domain/        Vault, RiskSignal, Source models.
  providers/     Morpho GraphQL client + VaultProvider boundary.
  scoring/       Risk signal derivation.
  cache/         In-memory TTL cache.
```

### Morpho provider

- One module owning the GraphQL request and response types.
- A normalization step mapping GraphQL responses to `domain` models.
- Recorded JSON fixtures committed for offline tests.

### Cache

- `Arc<RwLock<Option<Cached<Vec<Vault>>>>>` style store with a TTL.
- Lazy refresh: a request past TTL triggers a fresh fetch; failure with an
  empty cache surfaces as `ApiError::upstream_unavailable`.

### Scoring

- Pure functions: vault data in, `Vec<RiskSignal>` out.
- Thresholds as named constants.

### Frontend

- Replace the default `apps/web` page with an async Server Component that
  fetches `${API_URL}/vaults` and renders a table.
- Handle populated, empty, and error (caught fetch failure) states.
- Add `API_URL` to frontend env config.

## Implementation Order

1. Confirm this plan; produce the task list.
2. Backend: domain models and `ApiError`.
3. Backend: Morpho GraphQL provider plus recorded fixtures.
4. Backend: in-memory TTL cache and `AppState`.
5. Backend: scoring (risk signals).
6. Backend: `/vaults` and `/vaults/:address` routes; update `/health`.
7. Frontend: Server Component table page and `API_URL` config.
8. Update affected docs: ADR (new record), `002-api-contract.md`, README.
9. Run verification commands; record follow-ups.

## Risks

- **GraphQL schema drift / wrong field assumptions.** Mitigation: verify
  against the live schema early; fixtures are recorded from real responses.
- **APY/decimal unit mistakes.** Mitigation: pin units in normalization tests;
  keep values as strings across the boundary.
- **Cold-start latency** (first request blocks on Morpho). Mitigation:
  acceptable for this iteration; background refresh is a documented later step.
- **Scope creep into filters/detail page.** Mitigation: those are explicitly
  out of scope per the spec.

## Parallel Work

- Frontend table page can be built against the documented contract while the
  backend provider is in progress.
- Doc updates can proceed alongside code.

Sequential:

- Routes depend on provider, cache, and scoring.
- Frontend integration test depends on a running backend.

## Verification Checkpoints

1. Backend: `cargo fmt --check`, `cargo clippy --all-targets --all-features
   -- -D warnings`, `cargo test` pass; provider tests run offline.
2. Manual: `curl /vaults` and `curl /vaults/:address` return real Base data;
   a forced upstream failure returns 503 with a typed body.
3. Frontend: `pnpm lint` and `pnpm build` pass; the page renders real vaults.
4. Docs: ADR, API contract, and README reflect the live-data decisions.

## Open Decisions Before Implementation

1. Vault cap constant value (spec suggests ~25).
2. Risk signal threshold values (liquidity ratio bands, TVL size bands, vault
   age cutoff).
3. Whether the new ADR amends `001` or is a separate `004` architecture record.
