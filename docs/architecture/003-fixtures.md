# Fixture Strategy

## Status

Draft.

## Purpose

Fixtures let the frontend and backend build against stable Base/Morpho-shaped data before a live provider is selected.

## Required Cases

Fixture data should include:

- A Base Morpho vault with normal APY, TVL, and liquidity.
- A vault with missing APY.
- A vault with low liquidity.
- A young vault.
- A vault with an unknown curator.
- A vault with multiple market exposures.

## Fixture Rules

- Use realistic shapes, but do not imply real performance or recommendations.
- Keep financial values as strings.
- Include source metadata with `kind: "mock"`.
- Include risk signals as separate explanations instead of a single score.
- Keep addresses syntactically address-like, even when mocked.

## Future Location

When implementation needs shared fixtures, prefer one backend-owned fixture source first. The frontend can consume it through the API or copy a generated JSON snapshot if component tests need static data.
