# Tasks: Dashboard Design Migration

## Gate

Spec and plan approved 2026-05-21. Implementation underway.

## Task List

- [ ] Task: shadcn baseline in `apps/web`
  - Acceptance: `pnpm dlx shadcn@latest init` has been run inside
    `apps/web`; `components.json`, `src/lib/utils.ts`, and the updated
    `src/app/globals.css` exist with the shadcn CSS variables; the existing
    page still renders.
  - Verify: `pnpm web:lint` and `pnpm web:build` pass; the iteration 002
    page is visually unchanged at this step.
  - Files: `apps/web/components.json`, `apps/web/src/lib/utils.ts`,
    `apps/web/src/app/globals.css`, `apps/web/package.json`

- [ ] Task: Non-shadcn dependencies
  - Acceptance: `next-themes`, `lucide-react`, `recharts`, and
    `tw-animate-css` are installed in `apps/web`. No other design-folder
    dependencies are added at this step.
  - Verify: `pnpm web:build` passes.
  - Files: `apps/web/package.json`, `pnpm-lock.yaml`

- [ ] Task: Theme provider and layout
  - Acceptance: `src/components/theme-provider.tsx` wraps `next-themes`'
    `ThemeProvider` with `attribute="class"`, `defaultTheme="dark"`,
    `enableSystem`; `src/app/layout.tsx` mounts it and applies the design's
    font choice.
  - Verify: dark mode is the default in dev; toggling `class="dark"` on
    `<html>` flips theme; no hydration warnings in console.
  - Files: `apps/web/src/components/theme-provider.tsx`,
    `apps/web/src/app/layout.tsx`

- [ ] Task: Mock data module
  - Acceptance: `src/lib/mock-data.ts` exists with a `MOCK DATA` file-level
    comment and named exports `mockTvlSeries`, `mockTopVaults`,
    `mockChainDistribution` (single `Base/100%` entry).
  - Verify: `pnpm web:lint` passes; a grep for `mock-data` finds only this
    file and the surfaces that import it.
  - Files: `apps/web/src/lib/mock-data.ts`

- [ ] Task: shadcn components for dashboard surfaces
  - Acceptance: each shadcn component the kept surfaces import has been
    added via `pnpm dlx shadcn@latest add <name>`. The exact set is
    determined by reading the design's components before porting; expected
    candidates: `card`, `badge`, `button`, `table`, `skeleton`, `alert`,
    `dropdown-menu`, `chart`, `tooltip`. The final list is recorded inline
    here as each one is added.
  - Verify: `pnpm web:build` passes after each addition.
  - Files: `apps/web/src/components/ui/*`

- [ ] Task: Port header with theme toggle
  - Acceptance: `src/components/dashboard/header.tsx` renders the design's
    header with a working theme toggle. Links to pages that do not exist in
    this iteration are removed.
  - Verify: header renders in isolation on the home page; toggle flips
    theme.
  - Files: `apps/web/src/components/dashboard/header.tsx`

- [ ] Task: Port stats cards with live derivations
  - Acceptance: `src/components/dashboard/stats-cards.tsx` accepts a
    `Vault[]` prop and renders three cards: total TVL, vault count, and
    average APY, all derived from the vault list. The design's 24h-change
    card is dropped.
  - Verify: card values match `vaults.length`, sum of `tvl_usd`, and mean
    of `net_apy_percent` in dev.
  - Files: `apps/web/src/components/dashboard/stats-cards.tsx`

- [ ] Task: Port TVL chart (mock series, labeled)
  - Acceptance: `src/components/dashboard/tvl-chart.tsx` renders the
    design's chart against `mockTvlSeries`; the card header includes a
    visible "Sample data" label.
  - Verify: chart renders without console errors; recharts respects dark
    mode.
  - Files: `apps/web/src/components/dashboard/tvl-chart.tsx`

- [ ] Task: Port chain distribution (Base placeholder)
  - Acceptance: `src/components/dashboard/chain-distribution.tsx` renders
    the design's component against `mockChainDistribution` showing a single
    "Base 100%" segment.
  - Verify: component renders in isolation; no other chains appear.
  - Files: `apps/web/src/components/dashboard/chain-distribution.tsx`

- [ ] Task: Port top vaults (mocked)
  - Acceptance: `src/components/dashboard/top-vaults.tsx` renders the
    design's component against `mockTopVaults`; the widget has a visible
    "Sample data" cue.
  - Verify: renders without errors; styling matches the design.
  - Files: `apps/web/src/components/dashboard/top-vaults.tsx`

- [ ] Task: Port vaults table with live data and re-grafted states
  - Acceptance: `src/components/dashboard/vaults-table.tsx` accepts the
    live `Vault[]` shape, mirrors the design's columns, and reuses the
    iteration 002 risk-signal rendering. Loading uses shadcn `Skeleton`;
    errors and empty state use shadcn `Alert`. The old
    `src/components/vaults/VaultTable.tsx` is deleted.
  - Verify: against the live API, rows populate correctly; killing the API
    and reloading shows the `Alert` error state; an empty vault list shows
    the empty `Alert`.
  - Files: `apps/web/src/components/dashboard/vaults-table.tsx`,
    `apps/web/src/components/vaults/VaultTable.tsx` (deleted)

- [ ] Task: Replace home page with full dashboard composition
  - Acceptance: `src/app/page.tsx` is the dashboard composition (header,
    stats grid, TVL chart, chain distribution + top vaults, vaults table)
    wired to the existing `getVaults()` fetch. Populated, empty, and error
    states all render through the new layout.
  - Verify: the full dashboard renders in dev; `pnpm web:lint` and
    `pnpm web:build` pass; manual visual check of all three states.
  - Files: `apps/web/src/app/page.tsx`

- [ ] Task: Update docs
  - Acceptance: README mentions the dashboard layout and `next-themes`
    default. A frontend ADR is added only if a non-trivial decision deserves
    it (e.g. mock-vs-live policy worth a long-term record).
  - Verify: docs match the implemented behavior; no stale mock references.
  - Files: `README.md`, optionally `docs/architecture/NNN-*.md`
