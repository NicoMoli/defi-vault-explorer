# Plan: Dashboard Design Migration

## Spec

This plan implements `docs/specs/003-design-migration.md`.

## Current Gate

Status: approved (2026-05-21) — implementation in progress.

## Assumptions

1. The design source lives at `~/Downloads/de-fi-dashboard` and contains an
   `app/`, `components/`, `hooks/`, `lib/`, `styles/` layout from a v0 export.
   That folder is *not* checked into this repo and remains read-only during
   the port.
2. The current `apps/web` is the iteration 002 result: a Server Component
   home page fetching `${API_URL}/vaults` and rendering `VaultTable`, with
   populated/empty/error states.
3. The Rust API and its contract do not change in this iteration.
4. Tailwind v4 is already in place on both sides, so no Tailwind config
   migration is required beyond the shadcn-generated `globals.css`.

## Components

### shadcn baseline

- Run `pnpm dlx shadcn@latest init` inside `apps/web`. Accept defaults that
  match `src/` layout and `@/*` aliases. Confirm the generated
  `components.json`, `src/lib/utils.ts`, and `src/app/globals.css` look sane.
- Add `tw-animate-css` if shadcn init does not pull it in.
- Add `next-themes`, `lucide-react`, `recharts` manually.
- Use `pnpm dlx shadcn@latest add <name>` for each component the dashboard
  surfaces import. Track the set in the tasks file.

### Theme provider

- New `src/components/theme-provider.tsx` wrapping `next-themes`'
  `ThemeProvider` with `attribute="class"`, `defaultTheme="dark"`,
  `enableSystem`.
- Wired into `src/app/layout.tsx` so the whole tree opts in.
- A theme toggle component sits in the header (either ported from the design
  or built minimally with `lucide-react` icons).

### Dashboard surfaces

Port each of the design's dashboard components into `src/components/dashboard/`:

- `header.tsx` — branding, theme toggle, any nav. Trim links that do not
  resolve to real pages in this iteration.
- `stats-cards.tsx` — accepts a `Vault[]` prop, renders three cards
  (TVL/count/avg APY) derived from it. The design's 24h-change card is
  dropped.
- `tvl-chart.tsx` — reads `mockTvlSeries`; card chrome shows "Sample data".
- `chain-distribution.tsx` — reads `mockChainDistribution` (single entry).
- `top-vaults.tsx` — reads `mockTopVaults`.
- `vaults-table.tsx` — accepts `Vault[]`, mirrors the design's columns,
  re-uses iteration 002's `RISK_STYLES` and signal rendering, surfaces
  loading via `Skeleton` and errors via `Alert`.

`src/components/vaults/VaultTable.tsx` is replaced by `vaults-table.tsx` (the
design's name and location). The `Vault`, `RiskSignal`, and `VaultSource`
type exports move with it; existing imports update accordingly.

### Mocks

- New `src/lib/mock-data.ts` with named exports for `mockTvlSeries`,
  `mockTopVaults`, `mockChainDistribution`.
- File-level header comment: `// MOCK DATA — every export here is a
  placeholder until a real source is wired in.`

### Page composition

- `src/app/layout.tsx` is rewritten to mount `ThemeProvider`, set the font
  carried over from the design, and apply the shadcn-generated globals.
- `src/app/page.tsx` becomes the dashboard composition: header, stats grid,
  TVL chart row, chain distribution + top vaults row, vaults table at the
  bottom. The existing `getVaults()` fetch and try/catch wrapper for the
  error state are preserved; only the *rendering* changes. Empty and error
  states render via shadcn `Alert`.

## Implementation Order

1. Confirm this plan; produce the task list.
2. `shadcn init`, add `next-themes`, `recharts`, `lucide-react`,
   `tw-animate-css`. Commit the baseline.
3. Port `theme-provider.tsx` and wire it into a new `layout.tsx`.
4. Add `shadcn add` components needed by the design surfaces, recording each
   in the tasks file as it is added.
5. Add `src/lib/mock-data.ts` with the four named exports.
6. Port `header.tsx`, `stats-cards.tsx`, `tvl-chart.tsx`,
   `chain-distribution.tsx`, `top-vaults.tsx`, in that order — each one
   isolated, each verified to render against its data source.
7. Port `vaults-table.tsx`, re-graft live data and iteration 002's
   loading/empty/error states using shadcn `Skeleton` and `Alert`.
8. Replace `src/app/page.tsx` with the full dashboard composition. Delete
   the now-unused `src/components/vaults/VaultTable.tsx`.
9. Run verification (`pnpm web:lint`, `pnpm web:build`), spot-check in dev
   against the live API, toggle dark/light, force an API failure to confirm
   the error state still surfaces.
10. Update affected docs: a brief ADR-style note if a meaningful frontend
    decision warrants one (theming choice, mock-vs-live policy), and the
    README if needed.

## Risks

- **Design components depend on shadcn pieces we have not added yet.** Each
  port step starts by reading the design component, listing its `@/components/ui/*`
  imports, and `shadcn add`-ing them before copying.
- **Tailwind v4 + shadcn variable drift.** The design's `globals.css` may
  predate the version of shadcn the CLI installs. Mitigation: trust the CLI
  output, copy only the design's *additions* (e.g. animation utilities) on
  top.
- **Live data shape mismatch.** The design's `vaults-table` will assume
  fields the live `Vault` does not have (and vice versa). Mitigation: map
  during the port, drop columns we cannot fill from live data, do not
  invent.
- **Regression of iteration 002's error UX.** Mitigation: verify the error
  state manually by forcing an API failure (kill the backend, reload).
- **Scope creep into filters, detail pages, or charts that need real data.**
  Mitigation: out-of-scope list in the spec is authoritative.

## Parallel Work

- Mock data file and shadcn baseline can land before any dashboard component
  is ported.
- Each non-table dashboard surface can be ported independently once its
  shadcn deps are present.

Sequential:

- `vaults-table` depends on the live `Vault` type being importable from its
  new location.
- The final `page.tsx` composition depends on every surface existing.

## Verification Checkpoints

1. Baseline: after `shadcn init`, `pnpm web:lint` and `pnpm web:build` still
   pass and the existing page still renders.
2. After each dashboard surface lands: that component renders in isolation
   (a temporary import on the home page is fine during the port).
3. Final: full dashboard renders against the live API; dark/light toggle
   works; forced API failure surfaces the shadcn `Alert` error state;
   `pnpm web:lint` and `pnpm web:build` pass.

## Open Decisions Before Implementation

1. Whether the header toggle uses the design's component verbatim or a
   minimal shadcn-based version. Resolved on inspection of the design's
   `header.tsx`.
2. Whether a frontend ADR is warranted. Decided once the port is done.
