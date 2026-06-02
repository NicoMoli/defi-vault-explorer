# AGENTS.md

Operational guide for AI coding agents (Claude Code, Codex, OpenCode, Cursor, etc.)
working in this repository. `CLAUDE.md` is a symlink to this file.

## Project

`defi-vault-explorer` — a learning-first DeFi vault explorer, focused first on
Morpho Vaults on the Base chain.

- **Frontend:** `apps/web` — Next.js App Router, TypeScript, Tailwind.
- **Backend:** `apps/api` — Rust API skeleton targeting Axum.
- **Monorepo:** pnpm workspace (`apps/web`) + Cargo workspace (`apps/api`).

## Workflow

There is no mandatory spec gate — for small or exploratory changes, going
straight to code is fine.

When stress-testing a plan (e.g. via `grill-me` / `grill-with-docs`), capture
decisions that are **hard to reverse and the result of a real trade-off** as an
ADR (Architecture Decision Record) in `docs/architecture/`. Don't write specs
for this — ADRs record *why* a decision was made, after the fact, not a plan of
what to build.

Specs are optional. The existing `docs/specs/` files are kept as historical
record; the convention there is a numbered spec plus matching `-plan.md` and
`-tasks.md`, used only when the maintainer explicitly asks.

Before coding or proposing a design, read the domain guide:
[`docs/agents/domain.md`](docs/agents/domain.md). It points at `CONTEXT.md` (the
glossary of the project's language) and the ADRs, and explains how to use and
maintain them.

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
