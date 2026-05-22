import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { Badge } from "@/components/ui/badge";
import { mockTopVaults } from "@/lib/mock-data";

function formatUsd(value: number): string {
  if (value >= 1_000_000_000) return `$${(value / 1_000_000_000).toFixed(2)}B`;
  if (value >= 1_000_000) return `$${(value / 1_000_000).toFixed(2)}M`;
  return `$${value.toLocaleString()}`;
}

export function TopVaults() {
  return (
    <Card className="border-border/50 bg-card/50 backdrop-blur-sm">
      <CardHeader>
        <div className="flex items-center justify-between gap-2">
          <div>
            <CardTitle className="text-foreground">Top Performing Vaults</CardTitle>
            <CardDescription>Ranked by current APY</CardDescription>
          </div>
          <Badge variant="outline" className="border-warning/40 text-warning">
            Sample data
          </Badge>
        </div>
      </CardHeader>
      <CardContent>
        <div className="space-y-3">
          {mockTopVaults.map((vault, index) => (
            <div
              key={vault.id}
              className="flex items-center justify-between rounded-lg bg-secondary/30 p-3 transition-colors hover:bg-secondary/50"
            >
              <div className="flex items-center gap-3">
                <div className="flex h-8 w-8 items-center justify-center rounded-full bg-primary/20 text-sm font-bold text-primary">
                  {index + 1}
                </div>
                <div>
                  <div className="font-medium text-foreground">{vault.name}</div>
                  <div className="text-xs text-muted-foreground">
                    {vault.curator} · {vault.chain}
                  </div>
                </div>
              </div>
              <div className="text-right">
                <div className="font-mono text-foreground">
                  {vault.apy.toFixed(2)}%
                </div>
                <div className="text-xs text-muted-foreground">
                  {formatUsd(vault.tvl)} TVL
                </div>
              </div>
            </div>
          ))}
        </div>
      </CardContent>
    </Card>
  );
}
