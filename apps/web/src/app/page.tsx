type VaultSummary = {
  address: string;
  name: string;
  curator: string;
  chain: "Base";
  tvlUsd: string | null;
  netApyPercent: string | null;
  liquidityUsd: string | null;
  riskSignals: string[];
};

const vaults: VaultSummary[] = [
  {
    address: "0x8f5ae9cdd6c523863d1fd6f23ad43d87c3b1b1d0",
    name: "Base USDC Core",
    curator: "Mock Curator Alpha",
    chain: "Base",
    tvlUsd: "12840000",
    netApyPercent: "8.42",
    liquidityUsd: "3950000",
    riskSignals: ["High liquidity", "Established curator"],
  },
  {
    address: "0x3b17dd28f9c54d3c5f4a9ef7011c89d77f6e9124",
    name: "Base WETH Growth",
    curator: "Mock Curator Beta",
    chain: "Base",
    tvlUsd: "6420000",
    netApyPercent: "6.18",
    liquidityUsd: "1180000",
    riskSignals: ["Moderate liquidity", "Multi-market exposure"],
  },
  {
    address: "0xa2901f8d88f5a7baf4c5315da19ad9f973cbb742",
    name: "Base cbETH Emerging",
    curator: "Unknown curator",
    chain: "Base",
    tvlUsd: "940000",
    netApyPercent: null,
    liquidityUsd: "210000",
    riskSignals: ["Missing APY", "Young vault", "Unknown curator"],
  },
];

function formatUsd(value: string | null) {
  if (value === null) {
    return "Unavailable";
  }

  return new Intl.NumberFormat("en-US", {
    style: "currency",
    currency: "USD",
    maximumFractionDigits: 0,
  }).format(Number(value));
}

function formatApy(value: string | null) {
  return value === null ? "Unavailable" : `${value}%`;
}

export default function Home() {
  return (
    <main className="min-h-screen bg-stone-50 text-slate-950">
      <section className="border-b border-slate-200 bg-white">
        <div className="mx-auto flex w-full max-w-7xl flex-col gap-8 px-5 py-8 sm:px-8 lg:px-10">
          <div className="flex flex-col gap-5 lg:flex-row lg:items-end lg:justify-between">
            <div>
              <p className="text-sm font-medium uppercase tracking-normal text-emerald-700">
                Morpho Vaults on Base
              </p>
              <h1 className="mt-3 max-w-3xl text-4xl font-semibold tracking-normal text-slate-950 sm:text-5xl">
                defi-vault-explorer
              </h1>
            </div>
            <div className="grid grid-cols-3 gap-3 text-sm">
              <div className="border border-slate-200 bg-slate-50 px-4 py-3">
                <p className="text-slate-500">Vaults</p>
                <p className="mt-1 text-2xl font-semibold">{vaults.length}</p>
              </div>
              <div className="border border-slate-200 bg-slate-50 px-4 py-3">
                <p className="text-slate-500">Chain</p>
                <p className="mt-1 text-2xl font-semibold">Base</p>
              </div>
              <div className="border border-slate-200 bg-slate-50 px-4 py-3">
                <p className="text-slate-500">Mode</p>
                <p className="mt-1 text-2xl font-semibold">Mock</p>
              </div>
            </div>
          </div>
        </div>
      </section>

      <section className="mx-auto w-full max-w-7xl px-5 py-8 sm:px-8 lg:px-10">
        <div className="overflow-hidden border border-slate-200 bg-white">
          <div className="grid grid-cols-1 border-b border-slate-200 bg-slate-100 px-4 py-3 text-sm font-medium text-slate-600 md:grid-cols-[1.6fr_1fr_0.8fr_0.8fr_1.4fr]">
            <span>Vault</span>
            <span>TVL</span>
            <span>APY</span>
            <span>Liquidity</span>
            <span>Signals</span>
          </div>
          <div className="divide-y divide-slate-200">
            {vaults.map((vault) => (
              <article
                key={vault.address}
                className="grid grid-cols-1 gap-3 px-4 py-5 text-sm md:grid-cols-[1.6fr_1fr_0.8fr_0.8fr_1.4fr] md:items-center"
              >
                <div>
                  <h2 className="text-base font-semibold text-slate-950">
                    {vault.name}
                  </h2>
                  <p className="mt-1 truncate font-mono text-xs text-slate-500">
                    {vault.address}
                  </p>
                  <p className="mt-2 text-slate-600">{vault.curator}</p>
                </div>
                <p className="font-medium">{formatUsd(vault.tvlUsd)}</p>
                <p className="font-medium text-emerald-700">
                  {formatApy(vault.netApyPercent)}
                </p>
                <p className="font-medium">{formatUsd(vault.liquidityUsd)}</p>
                <div className="flex flex-wrap gap-2">
                  {vault.riskSignals.map((signal) => (
                    <span
                      key={signal}
                      className="border border-amber-200 bg-amber-50 px-2.5 py-1 text-xs font-medium text-amber-900"
                    >
                      {signal}
                    </span>
                  ))}
                </div>
              </article>
            ))}
          </div>
        </div>
      </section>
    </main>
  );
}
