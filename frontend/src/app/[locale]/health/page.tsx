"use client";

import dynamic from "next/dynamic";
import { ChartSkeleton } from "@/components/charts/ChartSkeleton";

// The health dashboard pulls in recharts; load it on demand so the library
// stays out of the route's initial bundle.
const HealthDashboard = dynamic(() => import("@/components/health/health-dashboard"), {
  ssr: false,
  loading: () => (
    <div className="p-8">
      <ChartSkeleton height={500} />
    </div>
  ),
});

export default function HealthPage() {
  return <HealthDashboard />;
}
