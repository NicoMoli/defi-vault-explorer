# defi-vault-explorer

A learning-first explorer for DeFi vaults (Morpho on Base, for now). Its stance
is transparency: it reports observed numbers and their age, and never presents
old data as current.

This file is a **glossary** of the project's ubiquitous language — what each term
*means*. It is not an architecture overview, a spec, or a record of decisions.
For those: *why* lives in `docs/architecture/` (ADRs); *how to run / what the
stack is* lives in `README.md` and `AGENTS.md`.

## Language

### The vault and its parts

**Vault**:
A curated allocation of a single deposited Asset across one or more lending
Markets, identified by its on-chain address.
_Avoid_: pool, fund.

**Curator**:
The named entity responsible for a Vault's allocation strategy and risk
parameters.
_Avoid_: manager, owner, operator.

**Asset**:
The single underlying token a Vault accepts and denominates positions in (e.g.
USDC).
_Avoid_: token, collateral.

**Market**:
A single lending venue a Vault can allocate into, with its own free liquidity.

**Allocation**:
A Vault's supplied position in one Market.

**Protocol**:
The lending system a Vault belongs to; `"morpho"` for now.
_Avoid_: platform.

**Chain**:
The blockchain network a Vault lives on; `"base"` for now.
_Avoid_: network (in user-facing copy).

### Headline numbers

**TVL**:
Total value locked — the total Assets held in a Vault, in USD.

**Net APY**:
A Vault's annualized yield after fees, expressed as a percentage.

**Withdrawable Liquidity**:
The USD a Vault could withdraw right now, *derived* (not reported by the
Provider) as the per-Market sum of `min(vault supply, market free liquidity)`.
_Note_: an approximation with known caveats — see ADR 004 and the provider's
`withdrawable_liquidity` doc comment.

### Risk

**Risk Signal**:
One transparent, independent observation about a Vault — a `{ label, level,
evidence }` triple. Context, never investment advice.
_Avoid_: rating, grade.

**Risk Level**:
The direction of a Risk Signal: Positive, Neutral, or Caution. There is
intentionally no composite numeric score.
_Avoid_: score, rank.

**Evidence**:
The plain-language statement of the underlying number behind a Risk Signal.

### Provenance and freshness

**Provider**:
A source of Vault data the explorer reads from; the Morpho GraphQL API is the
only one today.
_Avoid_: backend, API (those mean the explorer's own service).

**Snapshot**:
The most recent set of Vaults successfully obtained from a Provider, served
as-is whatever its age.
_Avoid_: cache contents, the data.

**Freshness**:
How recently a Snapshot was obtained, carried by its Source and always reflecting
the Snapshot's true age.
_Avoid_: up-to-date-ness.

**Stale**:
A Snapshot older than the intended refresh window. Still served — never relabeled
as current.
_Avoid_: expired, invalid.

**Source**:
The provenance and Freshness stamp attached to every Vault: which Provider the
data came from and when that Snapshot was obtained.

## Relationships

- A **Vault** belongs to one **Protocol** on one **Chain**, denominated in one
  **Asset**.
- A **Vault** holds one **Allocation** per **Market** it supplies.
- A **Vault** is overseen by a **Curator** (or none the Provider names).
- A **Vault** carries many **Risk Signals**, each with one **Risk Level** and its
  **Evidence**. **TVL**, **Net APY**, and **Withdrawable Liquidity** feed them.
- A **Snapshot** is a set of **Vaults** from one **Provider**, sharing one
  **Source**.
- A **Source** carries the **Freshness** of its **Snapshot**; a **Stale**
  Snapshot is still served, its Freshness still stating its real age.

## Example dialogue

> **Dev:** "Should a **Stale** **Snapshot** trigger a re-fetch when someone reads
> a **Vault**?"
> **Maintainer:** "No. We serve it as-is — the **Source** already states its true
> **Freshness**. Only an *empty* cache fetches. The background task is what cures
> staleness."
> **Dev:** "And **Withdrawable Liquidity** — that's a number from the
> **Provider**?"
> **Maintainer:** "No, the Provider doesn't expose it per Vault. We *derive* it
> across the Vault's **Allocations**, so treat it as an approximation."

## Flagged ambiguities

- "fresh" was overloaded: it once meant "within the read-side cache window" (a
  gate that decided whether to serve data). Resolved — the read path no longer
  gates on age; it serves any present **Snapshot** and only fetches when the
  cache is *empty*. **Freshness** is a reported signal, never a gate.
- "Provider" vs the explorer's own service: **Provider** is always an *upstream*
  data source (e.g. Morpho). The explorer's own service is the "API" or
  "backend" — never called a Provider.
