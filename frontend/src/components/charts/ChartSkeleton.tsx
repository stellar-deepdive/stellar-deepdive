interface ChartSkeletonProps {
  /** Pixel height of the placeholder; should roughly match the chart it stands in for. */
  height?: number;
  className?: string;
}

/**
 * Lightweight loading placeholder shown while a code-split chart chunk
 * (and the recharts library) is being fetched. Kept dependency-free so it
 * never pulls recharts into the initial bundle.
 */
export function ChartSkeleton({ height = 350, className = "" }: ChartSkeletonProps) {
  return (
    <div
      className={`w-full animate-pulse rounded-2xl border border-border/50 bg-muted/20 ${className}`}
      style={{ height }}
      aria-hidden="true"
    />
  );
}
