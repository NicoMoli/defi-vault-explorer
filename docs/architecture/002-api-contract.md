# API Contract Placeholder

## Status

Draft.

## Base URL

Local development API default:

```txt
http://127.0.0.1:4000
```

## Health

```http
GET /health
```

Expected response:

```json
{
  "status": "ok",
  "service": "defi-vault-explorer-api",
  "chain": "base",
  "data_mode": "mock"
}
```

## Future Vault Summary

```http
GET /vaults
```

Draft response shape:

```json
[
  {
    "address": "0x...",
    "name": "Base USDC Core",
    "protocol": "morpho",
    "chain": "base",
    "curator": "Mock Curator Alpha",
    "tvl_usd": "12840000",
    "net_apy_percent": "8.42",
    "liquidity_usd": "3950000",
    "risk_signals": [
      {
        "label": "High liquidity",
        "level": "positive",
        "evidence": "Mock fixture data"
      }
    ],
    "source": {
      "kind": "mock",
      "updated_at": null
    }
  }
]
```

## Contract Rules

- Financial quantities cross the API boundary as strings.
- Unknown provider values use `null`.
- Risk output is evidence-based signal context, not investment advice.
- Mock data should keep the same field names expected from future live data.
