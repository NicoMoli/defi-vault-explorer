import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { Badge } from "@/components/ui/badge";
import { mockChainDistribution } from "@/lib/mock-data";

export function ChainDistribution() {
  return (
    <Card className="border-border/50 bg-card/50 backdrop-blur-sm">
      <CardHeader>
        <div className="flex items-center justify-between gap-2">
          <div>
            <CardTitle className="text-foreground">Chain Distribution</CardTitle>
            <CardDescription>TVL share by chain</CardDescription>
          </div>
          <Badge variant="outline" className="border-warning/40 text-warning">
            Single-chain
          </Badge>
        </div>
      </CardHeader>
      <CardContent className="space-y-4">
        {mockChainDistribution.map((slice) => (
          <div key={slice.chain} className="space-y-2">
            <div className="flex items-center justify-between text-sm">
              <span className="font-medium text-foreground">{slice.chain}</span>
              <span className="font-mono text-muted-foreground">{slice.pct}%</span>
            </div>
            <div className="h-2 w-full overflow-hidden rounded-full bg-secondary">
              <div
                className="h-full rounded-full bg-primary"
                style={{ width: `${slice.pct}%` }}
              />
            </div>
          </div>
        ))}
        <p className="pt-2 text-xs text-muted-foreground">
          Base is the only chain wired in this iteration. Multi-chain support is
          out of scope.
        </p>
      </CardContent>
    </Card>
  );
}
