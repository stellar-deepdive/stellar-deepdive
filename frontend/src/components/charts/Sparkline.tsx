"use client";

import { ResponsiveContainer, LineChart, Line } from "recharts";

interface SparklineProps {
  data: { score: number }[];
  color: string;
}

/**
 * Tiny inline trend line. Extracted into its own module so the recharts
 * dependency can be code-split out of the pages that render sparklines.
 */
export function Sparkline({ data, color }: SparklineProps) {
  return (
    <ResponsiveContainer width="100%" height="100%">
      <LineChart data={data}>
        <Line type="monotone" dataKey="score" stroke={color} strokeWidth={2} dot={false} />
      </LineChart>
    </ResponsiveContainer>
  );
}
