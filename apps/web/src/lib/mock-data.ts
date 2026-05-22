// MOCK DATA — every export here is a placeholder until a real source is
// wired in. Surfaces that import this file must be visually labeled as
// "Sample data" so users never mistake these for live numbers.

export interface MockTvlPoint {
  date: string;
  tvl: number;
}

export const mockTvlSeries: MockTvlPoint[] = [
  { date: "2025-12", tvl: 820_000_000 },
  { date: "2026-01", tvl: 905_000_000 },
  { date: "2026-02", tvl: 980_000_000 },
  { date: "2026-03", tvl: 1_040_000_000 },
  { date: "2026-04", tvl: 1_120_000_000 },
  { date: "2026-05", tvl: 1_215_000_000 },
];

export interface MockTopVault {
  id: string;
  name: string;
  curator: string;
  chain: string;
  apy: number;
  tvl: number;
}

export const mockTopVaults: MockTopVault[] = [
  { id: "1", name: "Flagship USDC", curator: "Flagship", chain: "Base", apy: 9.4, tvl: 312_000_000 },
  { id: "2", name: "Steakhouse USDC", curator: "Steakhouse", chain: "Base", apy: 8.2, tvl: 280_000_000 },
  { id: "3", name: "Gauntlet WETH", curator: "Gauntlet", chain: "Base", apy: 5.1, tvl: 195_000_000 },
  { id: "4", name: "Re7 cbETH", curator: "Re7 Labs", chain: "Base", apy: 4.6, tvl: 142_000_000 },
  { id: "5", name: "Block Analitica DAI", curator: "Block Analitica", chain: "Base", apy: 7.0, tvl: 128_000_000 },
];

export interface MockChainSlice {
  chain: string;
  pct: number;
}

export const mockChainDistribution: MockChainSlice[] = [
  { chain: "Base", pct: 100 },
];
