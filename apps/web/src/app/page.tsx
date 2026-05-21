import { Alert, AlertDescription, AlertTitle } from "@/components/ui/alert";
import { DashboardHeader } from "@/components/dashboard/header";
import { StatsCards } from "@/components/dashboard/stats-cards";
import { TvlChart } from "@/components/dashboard/tvl-chart";
import { ChainDistribution } from "@/components/dashboard/chain-distribution";
import { TopVaults } from "@/components/dashboard/top-vaults";
import { VaultsTable, type Vault } from "@/components/dashboard/vaults-table";

const API_URL = process.env.API_URL ?? "http://127.0.0.1:4000";

async function getVaults(): Promise<Vault[]> {
  const response = await fetch(`${API_URL}/vaults`, { cache: "no-store" });
  if (!response.ok) {
    throw new Error(`API responded with ${response.status}`);
  }
  return response.json();
}

function Shell({ children }: { children: React.ReactNode }) {
  return (
    <div className="min-h-screen bg-background">
      <DashboardHeader />
      <main className="container mx-auto px-4 py-8">
        <div className="mb-8">
          <h1 className="text-3xl font-bold text-foreground text-balance">
            Morpho Vaults Dashboard
          </h1>
          <p className="mt-2 text-muted-foreground text-pretty">
            Live Morpho vaults on Base, compared by transparent safety signals.
            This is risk context, not investment advice.
          </p>
        </div>
        {children}
      </main>
    </div>
  );
}

export default async function Home() {
  let vaults: Vault[];
  try {
    vaults = await getVaults();
  } catch (error) {
    const detail = error instanceof Error ? error.message : "unknown error";
    return (
      <Shell>
        <Alert variant="destructive">
          <AlertTitle>Live vault data is unavailable</AlertTitle>
          <AlertDescription>
            The API could not be reached ({detail}). Stale data is never shown
            as fresh — try again shortly.
          </AlertDescription>
        </Alert>
      </Shell>
    );
  }

  if (vaults.length === 0) {
    return (
      <Shell>
        <Alert>
          <AlertTitle>No vaults right now</AlertTitle>
          <AlertDescription>
            The API returned an empty list. This usually means the upstream
            provider has no Base vaults to report at the moment.
          </AlertDescription>
        </Alert>
      </Shell>
    );
  }

  const updatedAt = vaults[0]?.source.updated_at;

  return (
    <Shell>
      <div className="space-y-8">
        <StatsCards vaults={vaults} />

        <div className="grid gap-6 lg:grid-cols-3">
          <TvlChart />
          <ChainDistribution />
        </div>

        <VaultsTable vaults={vaults} />

        <TopVaults />

        {updatedAt ? (
          <p className="text-xs text-muted-foreground">
            Source: Morpho API · refreshed {updatedAt}
          </p>
        ) : null}
      </div>
    </Shell>
  );
}
