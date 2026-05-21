import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { TrendingUp, Vault as VaultIcon, Percent } from "lucide-react";
import type { Vault } from "@/components/dashboard/vaults-table";

function formatUsd(value: number): string {
  if (value >= 1_000_000_000) return `$${(value / 1_000_000_000).toFixed(2)}B`;
  if (value >= 1_000_000) return `$${(value / 1_000_000).toFixed(2)}M`;
  if (value >= 1_000) return `$${(value / 1_000).toFixed(1)}K`;
  return `$${value.toFixed(0)}`;
}

function toNumber(value: string | null): number | null {
  if (value === null) return null;
  const n = Number(value);
  return Number.isFinite(n) ? n : null;
}

export function StatsCards({ vaults }: { vaults: Vault[] }) {
  const tvls = vaults.map((v) => toNumber(v.tvl_usd)).filter((n): n is number => n !== null);
  const apys = vaults.map((v) => toNumber(v.net_apy_percent)).filter((n): n is number => n !== null);

  const totalTvl = tvls.reduce((acc, n) => acc + n, 0);
  const avgApy = apys.length === 0 ? null : apys.reduce((acc, n) => acc + n, 0) / apys.length;

  const stats = [
    {
      title: "Total Value Locked",
      value: formatUsd(totalTvl),
      icon: TrendingUp,
      description: `Sum across ${vaults.length} live Base vault${vaults.length === 1 ? "" : "s"}`,
    },
    {
      title: "Active Vaults",
      value: vaults.length.toString(),
      icon: VaultIcon,
      description: "Live Morpho vaults on Base",
    },
    {
      title: "Average APY",
      value: avgApy === null ? "—" : `${avgApy.toFixed(2)}%`,
      icon: Percent,
      description: "Unweighted mean of net APY",
    },
  ];

  return (
    <div className="grid gap-4 md:grid-cols-3">
      {stats.map((stat) => (
        <Card key={stat.title} className="border-border/50 bg-card/50 backdrop-blur-sm">
          <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
            <CardTitle className="text-sm font-medium text-muted-foreground">
              {stat.title}
            </CardTitle>
            <stat.icon className="h-4 w-4 text-primary" />
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold text-foreground">{stat.value}</div>
            <div className="text-xs text-muted-foreground">{stat.description}</div>
          </CardContent>
        </Card>
      ))}
    </div>
  );
}
