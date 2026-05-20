// Presentational table for the live Morpho vault list. Server-rendered; no
// client interactivity in this iteration.

/** Direction of a single risk signal. Mirrors the API's lowercase enum. */
export type RiskLevel = "positive" | "neutral" | "caution";

/** One transparent risk observation, with the number behind it in `evidence`. */
export interface RiskSignal {
  label: string;
  level: RiskLevel;
  evidence: string;
}

/** Provenance and freshness metadata attached to every vault. */
export interface VaultSource {
  kind: string;
  updated_at: string | null;
}

/** A vault as returned by `GET /vaults`. Money fields are decimal strings. */
export interface Vault {
  address: string;
  name: string;
  protocol: string;
  chain: string;
  curator: string | null;
  asset_symbol: string;
  tvl_usd: string | null;
  net_apy_percent: string | null;
  liquidity_usd: string | null;
  risk_signals: RiskSignal[];
  source: VaultSource;
}

const RISK_STYLES: Record<RiskLevel, string> = {
  positive: "bg-emerald-100 text-emerald-800",
  neutral: "bg-slate-100 text-slate-700",
  caution: "bg-amber-100 text-amber-900",
};

/** Format a USD decimal string as a compact `$1.2M` / `$3.4K` label. */
function formatUsd(value: string | null): string {
  if (value === null) return "—";
  const amount = Number(value);
  if (!Number.isFinite(amount)) return "—";
  if (amount >= 1_000_000) return `$${(amount / 1_000_000).toFixed(1)}M`;
  if (amount >= 1_000) return `$${(amount / 1_000).toFixed(1)}K`;
  return `$${amount.toFixed(0)}`;
}

function RiskBadge({ signal }: { signal: RiskSignal }) {
  return (
    <span
      title={signal.evidence}
      className={`inline-block rounded px-2 py-0.5 text-xs font-medium ${RISK_STYLES[signal.level]}`}
    >
      {signal.label}
    </span>
  );
}

export function VaultTable({ vaults }: { vaults: Vault[] }) {
  return (
    <div className="overflow-x-auto rounded-lg border border-slate-200">
      <table className="w-full border-collapse text-sm">
        <thead className="bg-slate-50 text-left text-slate-600">
          <tr>
            <th className="px-4 py-3 font-semibold">Vault</th>
            <th className="px-4 py-3 font-semibold">Asset</th>
            <th className="px-4 py-3 text-right font-semibold">TVL</th>
            <th className="px-4 py-3 text-right font-semibold">Net APY</th>
            <th className="px-4 py-3 text-right font-semibold">Liquidity</th>
            <th className="px-4 py-3 font-semibold">Risk signals</th>
          </tr>
        </thead>
        <tbody>
          {vaults.map((vault) => (
            <tr key={vault.address} className="border-t border-slate-100">
              <td className="px-4 py-3">
                <div className="font-medium text-slate-900">{vault.name}</div>
                <div className="text-xs text-slate-500">
                  {vault.curator ?? "Unknown curator"}
                </div>
              </td>
              <td className="px-4 py-3 text-slate-700">{vault.asset_symbol}</td>
              <td className="px-4 py-3 text-right tabular-nums text-slate-700">
                {formatUsd(vault.tvl_usd)}
              </td>
              <td className="px-4 py-3 text-right tabular-nums text-slate-700">
                {vault.net_apy_percent === null
                  ? "—"
                  : `${vault.net_apy_percent}%`}
              </td>
              <td className="px-4 py-3 text-right tabular-nums text-slate-700">
                {formatUsd(vault.liquidity_usd)}
              </td>
              <td className="px-4 py-3">
                <div className="flex flex-wrap gap-1">
                  {vault.risk_signals.map((signal) => (
                    <RiskBadge key={signal.label} signal={signal} />
                  ))}
                </div>
              </td>
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  );
}
