# Tasks: Live Morpho Vault Data on Base

## Gate

The spec `002-live-morpho-base.md` and plan `002-live-morpho-base-plan.md` are
approved. All tasks below are implemented and verified.

## Task List

- [x] Task: Backend domain models and error type
  - Acceptance: `domain` models (`Vault`, `RiskSignal`, `Source`) and
    `ApiError` exist; `ApiError` maps to HTTP responses with a typed
    `{error, message}` body, including `upstream_unavailable` -> 503.
  - Verify: `cargo test` compiles; unit test for error-to-response mapping.
  - Files: `apps/api/src/domain/mod.rs`, `apps/api/src/error.rs`

- [x] Task: Morpho GraphQL provider with fixtures
  - Acceptance: a provider issues the Base whitelisted-vaults query and
    normalizes the response to domain models; recorded JSON fixtures cover
    normal, missing-APY, low-liquidity, young-vault, unknown-curator, and
    multi-allocation cases.
  - Verify: provider tests pass offline against fixtures.
  - Files: `apps/api/src/providers/morpho.rs`, `apps/api/src/providers/mod.rs`,
    `apps/api/tests/fixtures/`

- [x] Task: In-memory TTL cache and AppState
  - Acceptance: a TTL cache holds the vault list; lazy refresh on expiry;
    empty-cache upstream failure yields `upstream_unavailable`; `AppState`
    wires provider plus cache.
  - Verify: cache unit tests for expiry and refresh; `cargo test`.
  - Files: `apps/api/src/cache/mod.rs`, `apps/api/src/state.rs`,
    `apps/api/src/config.rs`

- [x] Task: Risk signal scoring
  - Acceptance: pure functions derive the four signals (liquidity, TVL size,
    vault age, curator) with `level` and evidence text; thresholds are named
    constants.
  - Verify: unit tests at threshold boundaries.
  - Files: `apps/api/src/scoring/mod.rs`

- [x] Task: Vault routes and health update
  - Acceptance: `GET /vaults` and `GET /vaults/:address` serve cached, scored
    data; unknown address returns 404 typed error; `/health` reports
    `data_mode: "live"`.
  - Verify: route tests for list, detail found/not-found, 503 cold start;
    update the existing health test.
  - Files: `apps/api/src/routes/vaults.rs`, `apps/api/src/routes/mod.rs`,
    `apps/api/src/main.rs`

- [x] Task: Frontend vault table page
  - Acceptance: `apps/web` home page is an async Server Component fetching
    `${API_URL}/vaults`, rendering a table with risk signals and source
    freshness; handles populated, empty, and error states.
  - Verify: `pnpm lint` and `pnpm build` pass; page renders real vaults.
  - Files: `apps/web/src/app/page.tsx`,
    `apps/web/src/components/vaults/VaultTable.tsx`, `apps/web/.env.example`

- [~] Task: Update architecture and contract docs
  - Acceptance: a new ADR records the GraphQL-API + in-memory-cache + live-only
    decision; `002-api-contract.md` reflects `data_mode: "live"`,
    `source.kind: "morpho-api"`, and the typed error body; README notes
    `API_URL`.
  - Verify: docs match the implemented behavior.
  - Files: `docs/architecture/004-live-morpho-data.md`,
    `docs/architecture/002-api-contract.md`, `README.md`
  - Status: ADR `004` and `002-api-contract.md` done. README deferred — it is
    stale beyond this iteration and needs a deliberate rewrite. Follow-up.
