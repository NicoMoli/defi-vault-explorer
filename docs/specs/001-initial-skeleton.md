# Spec: defi-vault-explorer Initial Skeleton

## Assumptions

1. This is a web application with a Next.js frontend and a Rust API backend.
2. The first supported protocol is Morpho Vaults only.
3. The first supported chain is Base.
4. The backend should expose normalized vault data through REST before any direct frontend-to-Morpho integration.
5. The intended cache layer is Redis, but initial vault data can be mocked until the API contract and UI shell are in place.
6. The product must describe transparent safety and risk signals without claiming that any vault is safe, recommended, or investment-grade.

Correct these before implementation if any are wrong.

## Objective

Build the first skeleton of `defi-vault-explorer`, a learning-first DeFi application that helps users compare Morpho Vaults by measurable signals such as liquidity, APY, TVL, curator, vault age, market exposure, and share-related data.

Primary users:

- Retail DeFi users comparing vaults.
- Vault curators inspecting vault behavior.
- Researchers evaluating DeFi yield opportunities.

The first milestone is not a finished product. It should establish a clear monorepo shape, a frontend/backend contract, and enough documented direction that future implementation can proceed safely.

Acceptance criteria for this skeleton:

- Repository has `apps/web`, `apps/api`, and `docs` folders.
- A reviewable spec exists before application code is generated.
- Initial frontend and backend responsibilities are separated.
- Open product and architecture questions are recorded.
- Safety language boundaries are explicit.

## Tech Stack

Frontend target:

- Next.js, latest stable at scaffold time, using App Router and TypeScript.
- React, latest stable required by the selected Next.js version.
- Tailwind CSS for styling.
- pnpm for JavaScript package management.
- Testing: Vitest or Jest for unit tests, Playwright for browser-level checks.

Backend target:

- Rust `1.94.0`, pinned by the repository `rust-toolchain.toml`.
- Axum `0.8.x` for HTTP routing.
- Tokio `1.x` for async runtime.
- Reqwest `0.13.x` for outbound HTTP.
- Serde and serde_json `1.x` for JSON.
- tower-http `0.6.x` for CORS and HTTP middleware.
- tracing and tracing-subscriber `0.1.x` / `0.3.x` for structured logs.
- thiserror `2.x` for typed application errors.
- anyhow `1.x` for setup and integration errors.
- dotenvy `0.15.x` for local env loading.

Data target:

- Start with normalized API models and simple caching.
- Use mocked vault data for the first skeleton.
- Redis is the intended cache approach once caching is implemented.
- Ask before adding SQLite, Postgres, an external hosted Redis provider, or a custom indexer.

Version note:

- As of May 7, 2026, Next.js docs describe `next@latest` upgrade flow and current Rust ecosystem references show Axum `0.8.9`, Reqwest `0.13.2`, and tower-http `0.6.8` as current docs.rs releases. Exact package versions should be locked by the package managers during scaffold.

## Commands

Initial setup commands, once this spec is approved:

```sh
cd defi-vault-explorer
pnpm create next-app apps/web --ts --app --eslint --tailwind --src-dir --import-alias "@/*"
cargo new apps/api --bin
```

Frontend commands:

```sh
cd defi-vault-explorer/apps/web
pnpm dev
pnpm lint
pnpm build
pnpm test
```

Backend commands:

```sh
cd defi-vault-explorer/apps/api
cargo run
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
```

Whole-project commands should be added after deciding whether to use pnpm workspace scripts, justfiles, Make, or plain documented commands.

## Project Structure

```txt
defi-vault-explorer/
  apps/
    web/                 Next.js frontend.
    api/                 Rust Axum API.
  docs/
    specs/               Living specs and implementation plans.
    architecture/        Architecture notes and data-source decisions.
    ideas/               Product notes, learning notes, and future concepts.
  README.md
  rust-toolchain.toml    Pinned Rust toolchain for API development.
```

Expected future structure:

```txt
apps/api/src/
  main.rs                Server startup and runtime wiring.
  config.rs              Environment-backed configuration.
  error.rs               Shared API error type and response mapping.
  routes/                HTTP route handlers.
  domain/                Vault, metric, risk signal models.
  providers/             Morpho and third-party data clients.
  scoring/               Safety/risk signal aggregation.
  cache/                 Snapshot or cache adapters.

apps/web/src/app/
  page.tsx               Vault explorer table.
  vaults/[address]/      Vault detail route.
  api/                   Optional frontend API client helpers only.

apps/web/src/components/
  vaults/                Vault table, filters, detail panels.
  risk/                  Risk signal UI components.
```

## Code Style

Rust style:

```rust
#[derive(Debug, serde::Serialize)]
pub struct VaultSummary {
    pub address: String,
    pub name: String,
    pub protocol: Protocol,
    pub tvl_usd: Option<String>,
    pub net_apy_percent: Option<String>,
    pub risk_signals: Vec<RiskSignal>,
}

pub async fn list_vaults(state: AppState) -> Result<Vec<VaultSummary>, ApiError> {
    let vaults = state.vault_provider.fetch_vaults().await?;
    Ok(vaults.into_iter().map(VaultSummary::from).collect())
}
```

TypeScript style:

```tsx
type VaultSummary = {
  address: string;
  name: string;
  protocol: "morpho";
  tvlUsd: string | null;
  netApyPercent: string | null;
  riskSignals: RiskSignal[];
};

export function VaultTable({ vaults }: { vaults: VaultSummary[] }) {
  return <section aria-label="Vault comparison">{/* table implementation */}</section>;
}
```

Conventions:

- Prefer explicit domain names: `VaultSummary`, `RiskSignal`, `VaultProvider`.
- Keep financial quantities as strings or decimal-safe types across API boundaries.
- Use `Option<T>` in Rust and `null` in JSON where provider data can be unavailable.
- Keep risk copy explanatory and evidence-based.
- Avoid abbreviations unless they are common DeFi terms such as APY, TVL, or ERC-4626.

## Testing Strategy

Backend:

- Unit tests for normalization, risk signal aggregation, and error mapping.
- Route tests for `/health`, `/vaults`, `/vaults/:address`, `/vaults/:address/metrics`, and `/vaults/:address/risk-signals`.
- Provider tests should use fixtures or recorded responses, not live network calls by default.

Frontend:

- Component tests for vault table states, filters, and risk signal rendering.
- Browser checks for loading, empty, error, and populated states.
- Accessibility checks for tables, filters, and detail pages.

Data correctness:

- Snapshot fixtures should include known vaults and edge cases: missing APY, low liquidity, young vault, unknown curator, and multiple market exposures.
- Any displayed safety score must include the underlying signals used to produce it.

## Boundaries

- Always: Label safety output as signals, indicators, or risk context.
- Always: Show source or freshness metadata for fetched vault data.
- Always: Preserve backend/frontend separation for normalized data.
- Always: Treat on-chain numeric values and shares with decimal-safe handling.
- Always: Keep the first chain focused on Base unless the scope changes.
- Ask first: Add a database, external hosted Redis provider, external hosted service, API key requirement, or analytics.
- Ask first: Expand beyond Morpho or beyond the first selected chain.
- Ask first: Add a single composite safety score if the explanation model is not ready.
- Never: Claim that a vault is safe or guaranteed.
- Never: Provide personalized investment advice.
- Never: Add authentication, billing, workspaces, or monetization in the MVP skeleton.
- Never: Build a full custom event indexer unless simpler data sources are insufficient.

## Success Criteria

- The project has a clear monorepo skeleton committed to files.
- The initial spec can be reviewed and corrected before code scaffold begins.
- The first implementation phase can be broken into tasks of five files or fewer.
- The MVP direction remains focused on Morpho Vault comparison.
- The open questions are visible enough to drive the next planning session.

## Open Questions

1. Which Morpho data source should we trust after the mock phase: Morpho API, subgraph, direct on-chain calls, or a hybrid?
2. What is the minimum acceptable freshness for vault metrics once live data is added: live, hourly, daily, or manual refresh?
3. Should the first version use a composite safety score, or only a signal-by-signal breakdown?
4. Should Redis run only locally for development at first, or should a hosted Redis provider be selected later?

## Next Gate

The initial spec, plan, and task list have been reviewed enough to confirm scaffold decisions. Next gate: scaffold the frontend and backend skeletons.
