# Spec: Dashboard Design Migration

## Status

Approved — implementation in progress (2026-05-21).

## Relationship to Prior Docs

Builds on `001-initial-skeleton.md` and `002-live-morpho-base.md`. This
iteration is purely a frontend change: it does not touch the API contract,
the Morpho provider, scoring, or the cache.

The current page (`apps/web/src/app/page.tsx` + `VaultTable.tsx`) is the live
surface delivered by iteration 002. This iteration replaces the surrounding
shell with a polished dashboard layout while keeping that live surface intact
underneath.

## Objective

Replace the bare table layout with the dashboard design provided as a v0/lovable
export, and graft the existing live Morpho/Base vault data into the new
`vaults-table` slot. Every other dashboard surface (stats cards, TVL chart,
chain distribution, top vaults) is mocked for this iteration; mocks are clearly
labeled and centralized.

Validation question for this iteration: *does the design hold up as a real
dashboard once a single live data island sits inside it, and where do the
remaining mocks start to feel dishonest?*

## Decisions (Confirmed)

These were confirmed in a grilling session and are not open for this iteration.

1. **Scope:** Adopt the full design composition (header, stats cards, TVL
   chart, chain distribution, top vaults, vaults table). Mock anywhere live
   data is missing.
2. **Approach:** Piece-by-piece port into `apps/web/src`. Do not bulk-copy
   the design folder; do not replace `apps/web` wholesale.
3. **shadcn/ui:** Run `pnpm dlx shadcn@latest init` in `apps/web`, then add
   only the components the kept dashboard surfaces actually import. Do not
   copy the design's `components/ui/*` verbatim.
4. **Folder layout:** Keep the existing `src/` layout. Alias `@/*` -> `src/*`.
5. **Theming:** Adopt `next-themes`, default to dark, expose a toggle in the
   header.
6. **Live data plug-in point:** The design's `vaults-table` reads the live
   Base vault list (current `Vault` shape from `VaultTable.tsx`). Every other
   surface reads from mocks. The existing live wiring in `page.tsx` is
   preserved end-to-end (fetch, populated/empty/error states).
7. **Single-chain placeholder:** Chain distribution renders as a "100% Base"
   single segment, not a multi-chain mock and not dropped.
8. **Stats cards:** Three cards only — TVL total, vault count, average APY,
   all derived from the live vault list. The design's 24h-change card is
   dropped; this iteration has no historical data and a mocked delta would
   read as dishonest.
9. **TVL chart:** Renders a mock time-series, labeled "Sample data" in the
   card chrome.
10. **Mocks:** All mocked values live in `src/lib/mock-data.ts` as named
    exports with a file-level `MOCK` comment. No inline mocks in components.
11. **Loading and error states:** Re-graft iteration 002's behavior using
    shadcn `Skeleton` and `Alert`. The "live data unavailable" and empty
    states must not regress.
12. **Dependencies:** Add only what the kept surfaces need. Use `shadcn add`
    to resolve Radix transitives; install `recharts`, `next-themes`, and
    `lucide-react` manually. Do not copy the design's full `dependencies`.
13. **Fonts:** Inherit the design's font choice (verified during port).

## Out of Scope

- Multi-chain support (chain distribution is a placeholder, not a real chart).
- Historical TVL ingestion or any backend changes to provide a real time
  series.
- Filters, sort controls, vault detail page, search.
- Mobile-first redesign — the design is taken as-is on responsive behavior.
- Replacing the live data fetch with React Query, SWR, or client-side
  fetching. The page stays a Server Component.

## Boundaries

- Always: keep `vaults-table` rendering live Morpho/Base data, with the same
  populated/empty/error states as iteration 002.
- Always: label mocked surfaces visibly (e.g. "Sample data" on the TVL chart).
- Always: centralize mocks in `src/lib/mock-data.ts`. One grep for `mock-data`
  must find every fake surface.
- Never: claim mocked numbers are live; never wire mock data through what
  looks like a real fetch.
- Never: introduce a second data path (browser-side fetch, API route proxy)
  for live vault data.
- Ask first: add a real historical time series, multi-chain support, a vault
  detail page, or replace the Server Component fetch pattern.

## Mock Inventory

After this iteration, the page contains exactly one live surface and the
following mocks (sourced from `src/lib/mock-data.ts`):

- `mockTvlSeries` — time series fed to the TVL chart.
- `mockTopVaults` — rows fed to the "top vaults" widget.
- `mockChainDistribution` — single entry: `[{ chain: "Base", pct: 100 }]`.

Stats card derivations from live data are *not* in `mock-data.ts`; they live
next to the stats-cards component.

## Testing Strategy

- `pnpm web:lint` and `pnpm web:build` pass.
- Manual visual check in dev: dashboard renders end-to-end against the live
  API; force an API failure and confirm the error state still surfaces (now
  via shadcn `Alert`).
- Manual visual check: dark/light toggle works; layout does not break in
  light mode.
- No new automated frontend tests are required for this iteration. (Frontend
  tests remain a follow-up tracked by 001.)

## Success Criteria

- The home page renders the new dashboard composition.
- `vaults-table` shows live Base Morpho vaults; loading/empty/error states
  behave as in iteration 002.
- Stats cards show live TVL, count, and average APY derived from the vault
  list; 24h-change is mocked-and-labeled or dropped.
- TVL chart, chain distribution, top vaults render from `src/lib/mock-data.ts`
  with visible "sample data" cues.
- Dark mode is the default; a toggle in the header switches to light.
- `pnpm web:lint` and `pnpm web:build` pass.
- The iteration is reviewable: the diff is a focused frontend change, no API
  changes, no regression of the iteration 002 live data path.

## Open Questions

1. Does the design's header include a theme toggle component already, or do
   we add one alongside the existing header content? Resolved during the
   port.
2. Exact font choice carried over from the design. Resolved during the port.

## Next Gate

Approve this spec and its plan, then convert the plan into tasks and
implement.
