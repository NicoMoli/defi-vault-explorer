# API Contract

## Status

Live as of iteration 002 (`docs/specs/002-live-morpho-base.md`). Serves real
Morpho vault data on Base. See ADR `004-live-morpho-data.md`.

## Base URL

Local development API default:

```txt
http://127.0.0.1:4000
```

## Health

```http
GET /health
```

Response:

```json
{
  "status": "ok",
  "service": "defi-vault-explorer-api",
  "chain": "base",
  "data_mode": "live"
}
```

## Vault List

```http
GET /vaults
```

Returns whitelisted Base Morpho vaults, ordered by USD TVL, capped at 25.

```json
[
  {
    "address": "0xBEEFE94c8aD530842bfE7d8B397938fFc1cb83b2",
    "name": "Steakhouse Prime USDC",
    "protocol": "morpho",
    "chain": "base",
    "curator": "Steakhouse Financial",
    "asset_symbol": "USDC",
    "tvl_usd": "465285424.49",
    "net_apy_percent": "4.85",
    "liquidity_usd": "136354152.91",
    "risk_signals": [
      {
        "label": "Liquidity",
        "level": "positive",
        "evidence": "29.3% of TVL is withdrawable right now."
      }
    ],
    "source": {
      "kind": "morpho-api",
      "updated_at": "2026-05-19T20:10:50.959076+00:00"
    }
  }
]
```

## Single Vault

```http
GET /vaults/{address}
```

Returns one vault object (same shape as a list element) by address,
case-insensitive, served from the same cached list. An unknown address returns
a `404` with the typed error body below.

## Errors

Error responses use a typed body:

```json
{ "error": "<code>", "message": "<text>" }
```

| Code                   | HTTP | When                                              |
| ---------------------- | ---- | ------------------------------------------------- |
| `upstream_unavailable` | 503  | Morpho API failed and the cache was empty.        |
| `not_found`            | 404  | No vault matches the requested address.           |

## Contract Rules

- Financial quantities cross the boundary as strings; `net_apy_percent` is a
  percentage string.
- Unknown provider values use `null`.
- `risk_signals` is signal-by-signal context (`level` is `positive`,
  `neutral`, or `caution`) — never a composite score, never investment advice.
- `source.updated_at` is the RFC 3339 cache-refresh timestamp; it reflects the
  true age of the data, so stale data is never presented as fresh.
