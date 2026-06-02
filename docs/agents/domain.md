# Domain guide for agents

How to read, use, and maintain this project's domain documentation before you
write code or propose a change. This is the **consume** side of the loop whose
**produce** side is the `grill-with-docs` workflow.

## Read first

Before coding or proposing a design, read:

1. **`CONTEXT.md`** (repo root) — the glossary of the project's ubiquitous
   language. It defines what each term *means*, nothing more.
2. **`docs/architecture/`** — the ADRs. They record *why* hard-to-reverse
   decisions were made. Skim the titles; read any that touch your area.

This repo is **single-context**: one `CONTEXT.md` at the root. (If a
`CONTEXT-MAP.md` ever appears at the root, the repo has gone multi-context and
each area has its own `CONTEXT.md` — read the map first.)

If `CONTEXT.md` or `docs/architecture/` does not exist, carry on silently — do
not fabricate one to satisfy this guide.

## Use the vocabulary

- Use the terms from `CONTEXT.md` exactly, in code and in prose. Prefer the
  canonical term over its listed `_Avoid_` aliases (e.g. **Provider** for an
  upstream data source, never "backend").
- If you need a concept that isn't in the glossary, that's a signal — flag it and
  offer to add the term, rather than inventing a synonym.
- If your work uses a term in a way that conflicts with its definition, surface
  the conflict instead of quietly redefining it. The glossary wins until it is
  deliberately changed.

## Respect the decisions

- If a change contradicts an existing ADR, say so explicitly and point at it —
  e.g. "This contradicts ADR 005 (read path fetches only on an empty cache) —
  but here's why it might be worth revisiting…". Don't silently work around a
  decision.
- ADRs are append-only history. To reverse one, write a new ADR that supersedes
  it; don't edit the old one's decision out.

## Keep it healthy

- The glossary is the one artifact that must stay accurate over time. When a term
  is resolved, sharpened, or contradicted, update `CONTEXT.md` in the same change
  — don't let it drift.
- Keep `CONTEXT.md` a glossary: no architecture prose, no implementation detail,
  no task lists. Those belong in ADRs (`docs/architecture/`) or in
  `README.md` / `AGENTS.md`.
