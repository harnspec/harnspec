---
status: planned
created: 2026-03-05
priority: high
tags:
- cloud
- deployment
- production
- infrastructure
- security
depends_on:
- 329-database-consolidation-multi-backend
created_at: 2026-03-05T05:23:18.233610402Z
updated_at: 2026-03-05T05:31:22.658851268Z
---
# Cloud Deployment Readiness

## Overview

LeanSpec's HTTP server (`leanspec-http`) needs hardening to support deployment on cloud platforms like Railway, Fly.io, Render, and similar PaaS/container hosts. While spec 317 covers Docker image creation, this spec addresses the **application-level gaps** that block production-ready cloud deployments.

## Current State

The Axum-based HTTP server has basic capabilities:
- Configurable `LEANSPEC_HOST` / `PORT` env vars
- Basic `/health` endpoint (returns OK without checking DB)
- CORS middleware, request tracing
- SQLite with WAL mode at `~/.lean-spec/leanspec.db`
- Structured logging to stdout via `tracing`

## Design

### 1. Configurable Data Directory

The hardcoded `~/.lean-spec/` path is the biggest blocker — cloud containers have ephemeral filesystems.

- Add `LEANSPEC_DATA_DIR` env var (overrides `~/.lean-spec/`)
- All paths (DB, config, sync state) resolve relative to this
- Document persistent volume mount strategy for each platform

### 2. Graceful Shutdown

Cloud platforms send SIGTERM before killing containers (typically 10-30s grace).

- Handle SIGTERM/SIGINT with `tokio::signal`
- Drain in-flight HTTP requests before exiting
- Close SQLite connections cleanly
- Log shutdown sequence for observability

### 3. Enhanced Health Checks

Cloud orchestrators need readiness probes to route traffic correctly.

- `GET /health/live` — simple liveness (always 200 if process is up)
- `GET /health/ready` — checks DB connectivity, returns 503 if not ready
- Keep existing `/health` as-is for backward compatibility

### 4. API Authentication

All `/api/*` endpoints are currently public — anyone with the URL can read/write specs.

- Add `LEANSPEC_API_KEY` env var for bearer token auth
- Middleware checks `Authorization: Bearer <key>` header
- Skip auth for health endpoints
- When no key is set, server runs unauthenticated (local dev mode)

### 5. Resource Limits

Prevent abuse and OOM in constrained cloud environments.

- `LEANSPEC_REQUEST_TIMEOUT` — per-request timeout (default: 30s)
- `LEANSPEC_MAX_REQUEST_SIZE` — body size limit (default: 5MB)
- Connection limit via tower middleware

### 6. Structured JSON Logging

Cloud log aggregators (Datadog, CloudWatch, etc.) need JSON.

- Add `LEANSPEC_LOG_FORMAT` env var (`text` | `json`, default: `text`)
- JSON format includes timestamp, level, message, span context
- Configurable log level via `LEANSPEC_LOG_LEVEL` (default: `info`)

### 7. Cloud Platform Configs

Provide ready-to-use deploy configs for popular platforms.

- `railway.json` — Railway deployment config
- `fly.toml` — Fly.io config
- `render.yaml` — Render blueprint
- `.env.example` — documented env var template

## Database Strategy

This spec uses **SQLite with persistent volumes** for cloud deployment — sufficient for single-instance deploys on Railway, Render, and Fly.io (all support volumes).

For PostgreSQL/MySQL multi-backend support, see **spec 329** (Database Consolidation and Multi-Backend Support), which handles the full migration from rusqlite → sqlx with connection pooling and `database_url` configuration. Spec 329 Phase 3 enables the PG path; this spec's `LEANSPEC_DATA_DIR` work aligns with 329's Phase 1 consolidation.

## Plan

- [ ] Add `LEANSPEC_DATA_DIR` env var and resolve all paths relative to it
- [ ] Implement graceful shutdown with SIGTERM/SIGINT handling
- [ ] Add `/health/live` and `/health/ready` endpoints with DB check
- [ ] Add API key authentication middleware (`LEANSPEC_API_KEY`)
- [ ] Add request timeout and body size limit middleware
- [ ] Add JSON log format option (`LEANSPEC_LOG_FORMAT`)
- [ ] Add `LEANSPEC_LOG_LEVEL` env var support
- [ ] Create `.env.example` with all env vars documented
- [ ] Create `railway.json` deploy config
- [ ] Create `fly.toml` deploy config
- [ ] Create `render.yaml` blueprint
- [ ] Document cloud deployment in docs-site

## Test

- [ ] Server starts with `LEANSPEC_DATA_DIR=/tmp/test` and creates DB there
- [ ] SIGTERM causes clean shutdown with no dropped requests
- [ ] `/health/ready` returns 503 when DB is inaccessible
- [ ] Requests without valid API key return 401 when `LEANSPEC_API_KEY` is set
- [ ] Requests succeed without auth when `LEANSPEC_API_KEY` is unset
- [ ] Requests exceeding body size limit return 413
- [ ] Requests exceeding timeout return 408
- [ ] `LEANSPEC_LOG_FORMAT=json` produces valid JSON log lines
- [ ] Railway/Fly.io/Render configs deploy successfully

## Notes

- This spec covers application-level changes. Docker image creation is in spec 317.
- SQLite is sufficient for single-instance cloud deploys with a persistent volume. PostgreSQL migration is handled by spec 329.
- The sync bridge (spec 213) is complementary — it syncs local specs to a hosted instance.
- Railway and Render support persistent volumes for SQLite; Fly.io uses Volumes.