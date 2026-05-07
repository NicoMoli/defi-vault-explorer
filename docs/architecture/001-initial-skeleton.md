# ADR 001: Initial Skeleton

## Status

Accepted.

## Context

`defi-vault-explorer` needs a small, reviewable monorepo skeleton before product features are built. The first milestone should establish frontend/backend separation, keep data mocked, and preserve the ability to change the live data source later.

## Decision

- Use `apps/web` for the frontend.
- Use `apps/api` for the Rust API.
- Use pnpm for JavaScript package management.
- Use Next.js App Router, TypeScript, and Tailwind CSS for the frontend.
- Use Rust `1.94.0`, pinned by `rust-toolchain.toml`.
- Use Axum as the backend HTTP framework.
- Target Base as the first chain.
- Use mocked Morpho vault data during the skeleton phase.
- Treat Redis as the intended cache layer, but defer implementation until the API contract is stable.

## Consequences

- Frontend development can start without a live data-source decision.
- Backend routing and normalized response shapes can evolve independently from the UI.
- Redis remains a documented direction, not an immediate dependency.
- Live Morpho data integration remains a later architecture decision.

## Still Open

- Which Morpho data source to trust after the mock phase.
- Whether Redis starts local-only or uses a hosted provider later.
- Whether the first product version uses only signal-by-signal risk context or includes a composite score.
