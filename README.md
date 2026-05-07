# defi-vault-explorer

Learning-first DeFi vault explorer focused first on Morpho Vaults.

## Status

Initial skeleton setup is in progress.

- Frontend: Next.js App Router, TypeScript, Tailwind, pnpm.
- Backend: Rust API skeleton with Axum target.
- Chain: Base.
- Data mode: mocked vault data first.
- Cache direction: Redis later, after the API contract is stable.

## Spec-Driven Development

The project follows spec-driven development. Start with the active skeleton docs:

- [Initial project skeleton spec](docs/specs/001-initial-skeleton.md)
- [Initial project skeleton plan](docs/specs/001-initial-skeleton-plan.md)
- [Initial project skeleton tasks](docs/specs/001-initial-skeleton-tasks.md)

Architecture notes live in:

- [Initial skeleton ADR](docs/architecture/001-initial-skeleton.md)
- [API contract placeholder](docs/architecture/002-api-contract.md)
- [Fixture strategy](docs/architecture/003-fixtures.md)

## Local Development

Install JavaScript dependencies:

```sh
pnpm install --frozen-lockfile --config.confirm-modules-purge=false
```

Run the frontend:

```sh
pnpm web:dev
```

Run the API:

```sh
pnpm api:dev
```

Verify the frontend:

```sh
pnpm web:lint
pnpm web:build
```

Verify the API:

```sh
pnpm api:fmt
pnpm api:clippy
pnpm api:test
```

The Rust toolchain is pinned in `rust-toolchain.toml`.
