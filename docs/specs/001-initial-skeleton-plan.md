# Plan: Initial Skeleton

## Spec

This plan implements `docs/specs/001-initial-skeleton.md`.

## Current Gate

Status: planning.

Do not scaffold `apps/web` or `apps/api` until the initial spec and this plan are reviewed.

## Assumptions

1. The repository will use a monorepo layout with `apps/web`, `apps/api`, and `docs`.
2. The first implementation pass should create only project skeleton files, not product features.
3. Next.js dependencies should be locked by pnpm, and the Rust compiler should be pinned by `rust-toolchain.toml`.
4. Until package management is chosen, whole-project commands stay documented instead of automated.
5. We should use pnpm for manage the monorepo, pnpm also for NextJS part and cargo for Rust side.

## Implementation Order

1. Confirm the initial skeleton spec.
2. Confirm this implementation plan.
3. Convert the plan into small implementation tasks.
4. Initialize the frontend skeleton in `apps/web`.
5. Initialize the backend skeleton in `apps/api`.
6. Add shared documentation for local development and architecture decisions.
7. Run scaffold verification commands and record any follow-up decisions.

## Components

### Frontend Skeleton

Responsible for the initial browser entry point and future vault explorer UI.

Initial work:

- Scaffold a Next.js App Router project in `apps/web`.
- Keep the first page simple, only showing some message, no feature.
- Add placeholder API contract types only after the backend contract is defined. Here, we can just create some /health API, no more is needed.

Dependencies:

- Package manager decision: npm, pnpm, or bun.
- Styling decision: Tailwind, CSS modules, or another explicit choice.

### Backend Skeleton

Responsible for normalized vault data and API routing.

Initial work:

- Scaffold a Rust binary crate in `apps/api`.
- Add Axum server wiring with `/health`.
- Keep Morpho provider code behind traits or modules so live data-source research can happen separately.

Dependencies:

- Rust `1.94.0` from `rust-toolchain.toml`.
- API route contract decisions.
- Data-source research before real vault ingestion.

### Documentation

Responsible for preserving decisions before implementation.

Initial work:

- Keep specs in `docs/specs`.
- Record architecture decisions in `docs/architecture`.
- Keep exploratory product notes in `docs/ideas`.

Dependencies:

- Human review for unresolved product and architecture questions.

## Risks

- Data-source uncertainty could leak into the UI too early.
  Mitigation: expose normalized backend routes first and use fixtures until provider behavior is known.

- Financial numeric precision could be lost if values are treated as floating-point numbers.
  Mitigation: keep money, share, and APY quantities as strings or decimal-safe types across API boundaries.

- Safety language could sound like investment advice.
  Mitigation: use signal-level copy and avoid recommendations or guarantees.

- Tooling churn could slow the first milestone.
  Mitigation: delay monorepo task runners until the app and API scaffolds are stable.

## Parallel Work

Can proceed in parallel after plan approval:

- Frontend scaffold and visual shell.
- Backend scaffold and health route.
- Documentation for architecture decisions.

Must be sequential:

- Data-source integration after route contract and provider choice.
- Composite safety score only after signal-by-signal explanation is defined.
- Persistent storage only after cache and ingestion requirements are clearer.

## Verification Checkpoints

1. Docs checkpoint: spec, plan, and tasks exist and are internally consistent.
2. Frontend checkpoint: `npm run lint` and `npm run build` pass in `apps/web`.
3. Backend checkpoint: `cargo fmt --check`, `cargo clippy --all-targets --all-features -- -D warnings`, and `cargo test` pass in `apps/api`.
4. Integration checkpoint: frontend can consume a documented backend response shape, even if backed by fixture data.

## Open Decisions Before Scaffold

1. Package manager: npm, pnpm, or bun.
2. Frontend styling: Tailwind or CSS modules.
3. First chain: Ethereum mainnet or another Morpho-supported chain.
4. First data source: Morpho API, subgraph, direct on-chain calls, or hybrid.
5. Initial cache: in-memory or file-backed.
