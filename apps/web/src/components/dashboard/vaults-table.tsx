import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { Badge } from "@/components/ui/badge";
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@/components/ui/table";

export type RiskLevel = "positive" | "neutral" | "caution";

export interface RiskSignal {
  label: string;
  level: RiskLevel;
  evidence: string;
}

export interface VaultSource {
  kind: string;
  updated_at: string | null;
}

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
  positive: "bg-primary/20 text-primary border-primary/30",
  neutral: "bg-secondary text-secondary-foreground border-border/50",
  caution: "bg-warning/20 text-warning border-warning/30",
};

function formatUsd(value: string | null): string {
  if (value === null) return "—";
  const n = Number(value);
  if (!Number.isFinite(n)) return "—";
  if (n >= 1_000_000_000) return `$${(n / 1_000_000_000).toFixed(2)}B`;
  if (n >= 1_000_000) return `$${(n / 1_000_000).toFixed(2)}M`;
  if (n >= 1_000) return `$${(n / 1_000).toFixed(1)}K`;
  return `$${n.toFixed(0)}`;
}

function RiskBadge({ signal }: { signal: RiskSignal }) {
  return (
    <span
      title={signal.evidence}
      className={`inline-block rounded border px-2 py-0.5 text-xs font-medium ${RISK_STYLES[signal.level]}`}
    >
      {signal.label}
    </span>
  );
}

export function VaultsTable({ vaults }: { vaults: Vault[] }) {
  return (
    <Card className="border-border/50 bg-card/50 backdrop-blur-sm">
      <CardHeader>
        <CardTitle className="text-foreground">Morpho Vaults · Base</CardTitle>
        <CardDescription>
          Live MetaMorpho vaults on Base, scored by transparent risk signals.
        </CardDescription>
      </CardHeader>
      <CardContent>
        <div className="overflow-x-auto">
          <Table>
            <TableHeader>
              <TableRow className="border-border/50 hover:bg-transparent">
                <TableHead className="text-muted-foreground">Vault</TableHead>
                <TableHead className="text-muted-foreground">Asset</TableHead>
                <TableHead className="text-right text-muted-foreground">TVL</TableHead>
                <TableHead className="text-right text-muted-foreground">Net APY</TableHead>
                <TableHead className="text-right text-muted-foreground">Liquidity</TableHead>
                <TableHead className="text-muted-foreground">Risk signals</TableHead>
              </TableRow>
            </TableHeader>
            <TableBody>
              {vaults.map((vault) => (
                <TableRow
                  key={vault.address}
                  className="border-border/50 hover:bg-secondary/30 transition-colors"
                >
                  <TableCell>
                    <div className="font-medium text-foreground">{vault.name}</div>
                    <div className="text-xs text-muted-foreground">
                      {vault.curator ?? "Unknown curator"}
                    </div>
                  </TableCell>
                  <TableCell>
                    <Badge variant="outline" className="border-border/50 text-muted-foreground">
                      {vault.asset_symbol}
                    </Badge>
                  </TableCell>
                  <TableCell className="text-right font-mono tabular-nums text-foreground">
                    {formatUsd(vault.tvl_usd)}
                  </TableCell>
                  <TableCell className="text-right font-mono tabular-nums text-foreground">
                    {vault.net_apy_percent === null ? "—" : `${vault.net_apy_percent}%`}
                  </TableCell>
                  <TableCell className="text-right font-mono tabular-nums text-foreground">
                    {formatUsd(vault.liquidity_usd)}
                  </TableCell>
                  <TableCell>
                    <div className="flex flex-wrap gap-1">
                      {vault.risk_signals.map((signal) => (
                        <RiskBadge key={signal.label} signal={signal} />
                      ))}
                    </div>
                  </TableCell>
                </TableRow>
              ))}
            </TableBody>
          </Table>
        </div>
      </CardContent>
    </Card>
  );
}
