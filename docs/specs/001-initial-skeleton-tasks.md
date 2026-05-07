# Tasks: Initial Skeleton

## Gate

The initial spec, plan, and scaffold decisions are confirmed. The next implementation tasks are the frontend and backend skeletons.

## Task List

- [x] Task: Create initial specification
  - Acceptance: The spec defines objective, tech stack, commands, structure, code style, testing strategy, boundaries, success criteria, and open questions.
  - Verify: Read `docs/specs/001-initial-skeleton.md`.
  - Files: `docs/specs/001-initial-skeleton.md`

- [x] Task: Create implementation plan
  - Acceptance: The plan identifies components, order, risks, parallel work, checkpoints, and open decisions.
  - Verify: Read `docs/specs/001-initial-skeleton-plan.md`.
  - Files: `docs/specs/001-initial-skeleton-plan.md`

- [x] Task: Create ordered task list
  - Acceptance: Tasks are small enough for focused sessions and include acceptance criteria, verification, and expected files.
  - Verify: Read `docs/specs/001-initial-skeleton-tasks.md`.
  - Files: `docs/specs/001-initial-skeleton-tasks.md`

- [x] Task: Confirm scaffold decisions
  - Acceptance: Package manager, styling approach, first chain, first data source research path, and initial cache approach are selected or explicitly deferred.
  - Verify: Update the open decisions section in the spec or plan.
  - Files: `docs/specs/001-initial-skeleton.md`, `docs/specs/001-initial-skeleton-plan.md`

- [ ] Task: Initialize frontend app
  - Acceptance: `apps/web` contains a Next.js App Router TypeScript scaffold with lint and build scripts.
  - Verify: `cd apps/web && pnpm lint && pnpm build`
  - Files: `apps/web`

- [ ] Task: Initialize backend app
  - Acceptance: `apps/api` contains a Rust Axum binary crate with a `/health` route, and the repository pins the Rust toolchain.
  - Verify: `cd apps/api && cargo fmt --check && cargo clippy --all-targets --all-features -- -D warnings && cargo test`
  - Files: `apps/api`, `rust-toolchain.toml`

- [x] Task: Document local development
  - Acceptance: README explains how to run the frontend, backend, tests, and where specs live.
  - Verify: Follow README commands from a clean shell.
  - Files: `README.md`

- [x] Task: Add first architecture decision record
  - Acceptance: `docs/architecture` records pnpm, Tailwind, Base, mocked initial data, and Redis as the intended cache approach.
  - Verify: Read the ADR and confirm it matches the chosen scaffold.
  - Files: `docs/architecture/001-initial-skeleton.md`

- [x] Task: Add API contract placeholder
  - Acceptance: The expected health route and future vault summary response shape are documented without committing to a live data provider.
  - Verify: Compare the contract doc against backend route names.
  - Files: `docs/architecture/002-api-contract.md`

- [x] Task: Add fixture strategy
  - Acceptance: Fixture expectations cover Base, mocked Morpho vaults, missing APY, low liquidity, young vault, unknown curator, and multiple market exposures.
  - Verify: Confirm future backend and frontend tests can share the fixture shape.
  - Files: `docs/architecture/003-fixtures.md`
