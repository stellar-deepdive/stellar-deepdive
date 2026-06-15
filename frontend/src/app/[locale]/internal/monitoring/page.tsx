"use client";

import React, { useState, useEffect } from "react";
import { MainLayout } from "@/components/layout";
import { Activity, Clock, Globe, RefreshCw, Zap } from "lucide-react";
import dynamic from "next/dynamic";
import { getMockMonitoringStats, MonitoringStats } from "@/lib/mock-monitoring";

// Code-split recharts: the chart section loads in its own chunk on demand.
const MonitoringCharts = dynamic(() => import("./MonitoringCharts"), {
  ssr: false,
  loading: () => (
    <div className="h-64 animate-pulse rounded-xl border border-gray-200 dark:border-slate-700 bg-gray-100 dark:bg-slate-800" />
  ),
});

export default function MonitoringDashboard() {
  const [stats, setStats] = useState<MonitoringStats | null>(null);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    const fetchData = () => {
      setLoading(true);
      // Simulate API latency
      setTimeout(() => {
        setStats(getMockMonitoringStats());
        setLoading(false);
      }, 800);
    };

    fetchData();
  }, []);

  if (loading || !stats) {
    return (
      <MainLayout>
        <div className="flex items-center justify-center h-screen">
          <RefreshCw className="w-8 h-8 text-blue-500 animate-spin" />
        </div>
      </MainLayout>
    );
  }

  return (
    <MainLayout>
      <div className="p-6 max-w-7xl mx-auto">
        <header className="mb-8">
          <h1 className="text-3xl font-bold text-gray-900 dark:text-white mb-2 flex items-center gap-2">
            <Activity className="text-blue-500" />
            Internal Performance Dashboard
          </h1>
          <p className="text-gray-600 dark:text-gray-400">
            Real-time monitoring of frontend vitals, API health, and system
            errors.
          </p>
        </header>

        {/* KPI Cards */}
        <div className="grid grid-cols-1 md:grid-cols-3 gap-6 mb-8">
          <div className="bg-white dark:bg-slate-800 p-6 rounded-xl border border-gray-200 dark:border-slate-700 shadow-sm">
            <div className="flex items-center justify-between mb-4">
              <h3 className="text-sm font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider">
                Total Sessions
              </h3>
              <Globe className="text-blue-500 w-5 h-5" />
            </div>
            <p className="text-3xl font-bold dark:text-white">
              {stats.sessionStats.totalSessions.toLocaleString()}
            </p>
            <p className="text-sm text-green-500 mt-2 flex items-center gap-1">
              <span>↑ 12%</span>
              <span className="text-gray-400 font-normal">vs last 24h</span>
            </p>
          </div>

          <div className="bg-white dark:bg-slate-800 p-6 rounded-xl border border-gray-200 dark:border-slate-700 shadow-sm">
            <div className="flex items-center justify-between mb-4">
              <h3 className="text-sm font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider">
                Avg. Session Duration
              </h3>
              <Clock className="text-purple-500 w-5 h-5" />
            </div>
            <p className="text-3xl font-bold dark:text-white">
              {Math.floor(stats.sessionStats.avgDuration / 60)}m{" "}
              {stats.sessionStats.avgDuration % 60}s
            </p>
            <p className="text-sm text-gray-400 mt-2">Target: 5m 00s</p>
          </div>

          <div className="bg-white dark:bg-slate-800 p-6 rounded-xl border border-gray-200 dark:border-slate-700 shadow-sm">
            <div className="flex items-center justify-between mb-4">
              <h3 className="text-sm font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider">
                Bounce Rate
              </h3>
              <Zap className="text-yellow-500 w-5 h-5" />
            </div>
            <p className="text-3xl font-bold dark:text-white">
              {stats.sessionStats.bounceRate}%
            </p>
            <p className="text-sm text-red-500 mt-2 flex items-center gap-1">
              <span>↑ 2.1%</span>
              <span className="text-gray-400 font-normal">vs last 24h</span>
            </p>
          </div>
        </div>

        {/* Charts Section (recharts loaded on demand) */}
        <MonitoringCharts stats={stats} />
      </div>
    </MainLayout>
  );
}
