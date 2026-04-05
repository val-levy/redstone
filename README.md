# Gotham

Gotham is a full-stack platform monorepo scaffold for a modular entity and analytics system. It includes a Next.js frontend, a Rust gateway/BFF, multiple backend services, shared packages, infrastructure definitions, operational tooling, and documentation.

## Quickstart

1. Copy `.env.example` to `.env` and update environment variables.
2. Start the local stack:

   ```sh
   make dev
   ```

3. Open the web app at `http://localhost:3000`.
4. Access the gateway at `http://localhost:8080`.

## Repository layout

- `apps/`
  - `web/` — Next.js frontend application
  - `gateway/` — Rust API gateway / BFF

- `services/`
  - `entity-service/` — CRUD + canonical entity management
  - `graph-service/` — relationship graph queries
  - `search-service/` — search orchestration and indexing
  - `timeline-service/` — temporal / event querying
  - `geo-service/` — geospatial and tiling service
  - `ingestion-service/` — stream consumers and ETL
  - `resolution-service/` — entity resolution and deduplication
  - `analytics-service/` — correlation, anomaly detection, analytics
  - `notification-service/` — alerts, subscriptions, push events

- `packages/`
  - `proto/` — protobuf contracts
  - `schemas/` — JSON schemas, OpenAPI fragments, validation artifacts
  - `shared-ts/` — frontend shared types and utilities
  - `shared-rust/` — shared Rust library code
  - `ui/` — shared React component system

- `infra/`
  - `docker/` — image and Dockerfile scaffolding
  - `compose/` — Docker Compose variants
  - `k8s/` — Kubernetes manifests and overlays
  - `terraform/` — infrastructure as code
  - `helm/` — Helm charts for deployment

- `ops/`
  - `scripts/` — development and operational helper scripts
  - `monitoring/` — observability config stubs
  - `runbooks/` — operational runbook guidance

- `docs/`
  - architecture, API documentation, design decisions, and module docs

- `tests/`
  - integration, contract, end-to-end, and load test scaffolding

## Tools

- `Makefile` — basic development task runner
- `docker-compose.yml` — root compose entrypoint for local services
- `pnpm-workspace.yaml` — shared frontend workspace config
- `Cargo.toml` — Rust workspace definition
- `pyproject.toml` — Python workspace metadata

## Notes
