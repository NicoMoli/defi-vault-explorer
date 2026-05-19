# AGENTS.md

Operational guide for AI coding agents (Claude Code, Codex, OpenCode, Cursor, etc.)
working in this repository. `CLAUDE.md` is a symlink to this file.

## Project

`defi-vault-explorer` — a learning-first DeFi vault explorer, focused first on
Morpho Vaults on the Base chain.

- **Frontend:** `apps/web` — Next.js App Router, TypeScript, Tailwind.
- **Backend:** `apps/api` — Rust API skeleton targeting Axum.
- **Monorepo:** pnpm workspace (`apps/web`) + Cargo workspace (`apps/api`).

## Workflow — spec-driven, review gate (required)

This project follows spec-driven development. **Do not write implementation
code until the spec, plan, and tasks for the iteration have been reviewed and
approved by the maintainer.**

For each iteration, the order is:

1. Spec — `docs/specs/NNN-*.md`
2. Plan — `docs/specs/NNN-*-plan.md`
3. Tasks — `docs/specs/NNN-*-tasks.md`
4. Implementation — only after 1–3 are approved.

Architecture decisions live in `docs/architecture/`.

## Commands

Install JS dependencies:

```sh
pnpm install --frozen-lockfile --config.confirm-modules-purge=false
```

Run:

```sh
pnpm web:dev   # frontend
pnpm api:dev   # backend
```

Verify — all of these must pass before a change is considered done:

```sh
pnpm web:lint
pnpm web:build
pnpm api:fmt
pnpm api:clippy
pnpm api:test
```

## Constraints

- The Rust toolchain is pinned in `rust-toolchain.toml` — do not change it
  casually.
- Match the existing code style; run the verify commands above rather than
  guessing.
