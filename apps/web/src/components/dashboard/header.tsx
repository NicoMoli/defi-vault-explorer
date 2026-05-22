import Link from "next/link";
import { ThemeToggle } from "./theme-toggle";

export function DashboardHeader() {
  return (
    <header className="sticky top-0 z-50 border-b border-border/50 bg-background/80 backdrop-blur-sm">
      <div className="container mx-auto flex h-16 items-center justify-between px-4">
        <Link href="/" className="flex items-center gap-2">
          <div className="flex h-8 w-8 items-center justify-center rounded-lg bg-primary">
            <span className="text-sm font-bold text-primary-foreground">M</span>
          </div>
          <span className="text-lg font-semibold text-foreground">
            DeFi Vault Explorer
          </span>
        </Link>
        <ThemeToggle />
      </div>
    </header>
  );
}
