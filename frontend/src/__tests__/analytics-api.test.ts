import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';
import {
  fetchAnalyticsMetrics,
  fetchApiUsageOverview,
  getMockAnalyticsData,
  getMockApiUsageOverview,
} from '@/lib/analytics-api';

describe('analytics-api', () => {
  const originalFetch = global.fetch;

  afterEach(() => {
    global.fetch = originalFetch;
    vi.restoreAllMocks();
  });

  describe('fetchAnalyticsMetrics', () => {
    it('returns parsed JSON when the request succeeds', async () => {
      const payload = getMockAnalyticsData();
      global.fetch = vi.fn().mockResolvedValue({
        ok: true,
        json: () => Promise.resolve(payload),
      }) as unknown as typeof fetch;

      const result = await fetchAnalyticsMetrics();

      expect(global.fetch).toHaveBeenCalledWith(
        expect.stringContaining('/api/analytics'),
        expect.objectContaining({ method: 'GET' }),
      );
      expect(result).toEqual(payload);
    });

    it('falls back to mock data when the response is not ok', async () => {
      const consoleError = vi.spyOn(console, 'error').mockImplementation(() => {});
      global.fetch = vi.fn().mockResolvedValue({
        ok: false,
        status: 500,
      }) as unknown as typeof fetch;

      const result = await fetchAnalyticsMetrics();

      expect(result.top_corridors.length).toBeGreaterThan(0);
      expect(result.active_corridors).toBe(result.top_corridors.length);
      expect(consoleError).toHaveBeenCalled();
    });

    it('falls back to mock data without logging on network errors', async () => {
      const consoleError = vi.spyOn(console, 'error').mockImplementation(() => {});
      global.fetch = vi.fn().mockRejectedValue(new TypeError('Failed to fetch')) as unknown as typeof fetch;

      const result = await fetchAnalyticsMetrics();

      expect(result.total_volume_usd).toBeGreaterThan(0);
      expect(consoleError).not.toHaveBeenCalled();
    });
  });

  describe('fetchApiUsageOverview', () => {
    it('returns parsed JSON when the request succeeds', async () => {
      const payload = getMockApiUsageOverview();
      global.fetch = vi.fn().mockResolvedValue({
        ok: true,
        json: () => Promise.resolve(payload),
      }) as unknown as typeof fetch;

      const result = await fetchApiUsageOverview();

      expect(global.fetch).toHaveBeenCalledWith(
        expect.stringContaining('/api/admin/analytics/overview'),
        expect.objectContaining({ method: 'GET' }),
      );
      expect(result).toEqual(payload);
    });

    it('falls back to mock data when the request throws', async () => {
      const consoleError = vi.spyOn(console, 'error').mockImplementation(() => {});
      global.fetch = vi.fn().mockRejectedValue(new Error('boom')) as unknown as typeof fetch;

      const result = await fetchApiUsageOverview();

      expect(result.top_endpoints.length).toBeGreaterThan(0);
      expect(result.status_distribution.length).toBeGreaterThan(0);
      expect(consoleError).toHaveBeenCalled();
    });
  });
});
