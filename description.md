# Stellar Deepdive — Project Description

## Overview

Stellar Deepdive is a full-stack analytics platform that provides real-time payment reliability metrics and liquidity health monitoring for the [Stellar network](https://stellar.org). It gives wallets, apps, anchors, and developers the data they need to make payments with confidence — before and after they're sent.

## What Problem It Solves

The Stellar network processes cross-border payments across many currency corridors, but there's no single place to answer questions like:

- Which corridors have the highest payment success rates right now?
- How deep is the liquidity for a given asset pair?
- How long does settlement actually take?
- Which anchors are performing reliably?

Stellar Deepdive collects, aggregates, and surfaces that data through a clean API and dashboard.

## Core Features

- **Payment success rate tracking** — per-corridor success/failure metrics derived from on-chain payment history
- **Liquidity depth analysis** — real-time order book analysis to quantify available capital
- **Anchor reliability scoring** — composite scores for asset issuers based on performance data
- **Corridor health metrics** — a 0–100 health score for each currency corridor
- **Settlement time monitoring** — median and average payment confirmation latency
- **Cost calculator** — route-by-route cross-border payment cost estimation
- **Price feed integration** — real-time USD pricing via CoinGecko with 15-minute caching
- **On-chain verification** — analytics snapshots anchored to the Stellar blockchain via Soroban smart contracts
- **Custom alerts** — configurable alert rules with notification delivery

## Architecture

```
Frontend (Next.js)  ──►  Backend (Rust/Axum)  ──►  Stellar Horizon / RPC
                               │
                         PostgreSQL DB
                               │
                      Soroban Smart Contracts
                      (on-chain verification)
```

The backend ingests payment data from Stellar's RPC, runs analytics pipelines, stores aggregated results in PostgreSQL, and exposes a REST + GraphQL API. The frontend renders the metrics in a real-time dashboard. Smart contracts on Soroban provide tamper-proof anchoring of analytics snapshots for trustless data integrity.

## Tech Stack

| Layer | Technology |
|---|---|
| Frontend | Next.js 16, React 19, TypeScript, Tailwind CSS |
| Backend | Rust, Axum, SQLx |
| Database | PostgreSQL 14+ |
| Caching | Redis |
| Smart Contracts | Soroban (Rust → WASM) |
| Observability | OpenTelemetry, Prometheus, Logstash |
| Auth | JWT, SEP-10 (Stellar auth standard) |
| API | REST + GraphQL (async-graphql) |

## Project Structure

```
stellar-deepdive/
├── frontend/       # Next.js dashboard and UI
├── backend/        # Rust analytics engine and API server
│   ├── src/        # Application source (API handlers, jobs, auth, caching)
│   └── migrations/ # PostgreSQL schema migrations
├── contracts/      # Soroban smart contracts
│   ├── snapshot-contract/   # On-chain analytics snapshot anchoring
│   ├── analytics/           # Analytics verification contracts
│   ├── governance/          # Governance contracts
│   └── access-control/      # Access control contracts
├── docs/           # Extended API and integration documentation
├── k8s/            # Kubernetes deployment manifests
└── scripts/        # Utilities and backup scripts
```

## Key Integrations

- **Stellar Horizon / RPC** — source of truth for payment, trade, and order book data
- **CoinGecko API** — real-time USD price feeds for all major Stellar assets
- **SEP-10** — Stellar's standard for wallet authentication
- **SEP-24 / SEP-31** — hosted deposit/withdrawal and cross-border payment standards

## Use Cases

**Wallets & apps** — predict payment success, suggest optimal routing, display corridor health scores to end users before sending.

**Anchors & issuers** — monitor asset performance, identify liquidity gaps, and track reliability over time.

**Developers** — consume payment analytics via REST or GraphQL API, verify data integrity on-chain, or build new products on top of the metrics infrastructure.

## Security

- API key authentication with per-key rate limiting
- SEP-10 challenge/response authentication for Stellar wallets
- Encrypted secrets via AES-GCM
- Analytics snapshots cryptographically anchored on-chain for immutable audit trails
- Full threat model documented in [`THREAT_MODEL.md`](./THREAT_MODEL.md)

## License

MIT
