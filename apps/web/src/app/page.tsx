// Home page: an async Server Component that fetches the live vault list from
// the API and renders it as a table. Handles populated, empty, and error
// states. No browser-side API calls.

import { VaultTable, type Vault } from "@/components/vaults/VaultTable";

const API_URL = process.env.API_URL ?? "http://127.0.0.1:4000";

/** Fetch the live vault list. Throws on any non-OK response. */
async function getVaults(): Promise<Vault[]> {
  const response = await fetch(`${API_URL}/vaults`, { cache: "no-store" });
  if (!response.ok) {
    throw new Error(`API responded with ${response.status}`);
  }
  return response.json();
}

function Shell({ children }: { children: React.ReactNode }) {
  return (
    <main className="mx-auto w-full max-w-5xl px-6 py-12">
      <header className="mb-8">
        <h1 className="text-2xl font-semibold text-slate-900">
          DeFi Vault Explorer
        </h1>
        <p className="mt-1 text-sm text-slate-600">
          Live Morpho vaults on Base, compared by transparent safety signals.
          This is risk context, not investment advice.
        </p>
      </header>
      {children}
    </main>
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
        <div className="rounded-lg border border-amber-200 bg-amber-50 p-6 text-sm text-amber-900">
          <p className="font-medium">Live vault data is unavailable.</p>
          <p className="mt-1 text-amber-800">
            The API could not be reached ({detail}). Stale data is never shown
            as fresh — try again shortly.
          </p>
        </div>
      </Shell>
    );
  }

  if (vaults.length === 0) {
    return (
      <Shell>
        <div className="rounded-lg border border-slate-200 bg-slate-50 p-6 text-sm text-slate-600">
          No vaults are available right now.
        </div>
      </Shell>
    );
  }

  const updatedAt = vaults[0]?.source.updated_at;

  return (
    <Shell>
      <VaultTable vaults={vaults} />
      {updatedAt ? (
        <p className="mt-3 text-xs text-slate-500">
          Source: Morpho API · refreshed {updatedAt}
        </p>
      ) : null}
    </Shell>
  );
}
