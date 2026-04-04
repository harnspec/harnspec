# Cloud Deployment Guide

HarnSpec can be deployed to any cloud platform that supports Docker containers.

## Quick Start

### Docker (self-hosted)

```bash
docker run -d \
  -p 3000:3000 \
  -v harnspec-data:/home/harnspec/.harnspec \
  -e HARNSPEC_API_KEY=your-secret-key \
  ghcr.io/harnspec/harnspec:latest
```

Or use the example docker-compose file:

```bash
cd deploy/examples && docker compose up -d
```

### Fly.io

```bash
cp deploy/fly.toml .
fly launch --copy-config
fly secrets set HARNSPEC_API_KEY=your-secret-key
fly deploy
```

### Railway

1. Connect your GitHub repo
2. Railway auto-detects `railway.json`
3. Set `HARNSPEC_API_KEY` in the Railway dashboard
4. Add a volume mounted at `/data` and set `HARNSPEC_DATA_DIR=/data`

### Render

1. Create a new Blueprint from `deploy/render.yaml`
2. Render auto-generates `HARNSPEC_API_KEY`
3. Persistent disk is configured automatically

## Environment Variables

| Variable | Default | Description |
|----------|---------|-------------|
| `PORT` | `3000` | Server port |
| `HARNSPEC_HOST` | `127.0.0.1` | Bind address (use `0.0.0.0` in containers) |
| `HARNSPEC_DATA_DIR` | `~/.harnspec` | Persistent data directory |
| `HARNSPEC_API_KEY` | _(none)_ | Bearer token for API authentication |
| `HARNSPEC_LOG_FORMAT` | `text` | `text` or `json` for structured logging |
| `HARNSPEC_LOG_LEVEL` | `info` | Log verbosity |
| `HARNSPEC_CORS_ORIGINS` | _(allow all)_ | Comma-separated allowed origins |
| `HARNSPEC_UI_DIST` | _(auto)_ | Path to UI static files |
| `HARNSPEC_PROJECT_SOURCES` | `local,github` | Enabled project sources (comma-separated: `local`, `github`) |

## Health Checks

| Endpoint | Purpose | Auth |
|----------|---------|------|
| `GET /health` | Basic health + version | No |
| `GET /health/live` | Liveness probe | No |
| `GET /health/ready` | Readiness (checks DB) | No |

## Architecture

The Docker image runs a single Rust binary serving both the API and static UI files.
No Node.js runtime is required. The image is ~30MB compressed.

```
┌─────────────────────────────┐
│  Cloud Platform (Fly/Railway/Render)  │
│  ┌───────────────────────┐  │
│  │  harnspec-http binary │  │
│  │  ├── REST API         │  │
│  │  ├── Static UI files  │  │
│  │  └── SQLite DB        │  │
│  └───────────────────────┘  │
│  ┌───────────────────────┐  │
│  │  Persistent Volume    │  │
│  │  └── /data/           │  │
│  │      ├── harnspec.db  │  │
│  │      ├── config.json  │  │
│  │      └── projects.json│  │
│  └───────────────────────┘  │
└─────────────────────────────┘
```

## Platform Comparison

| Feature | Fly.io | Railway | Render |
|---------|--------|---------|--------|
| Free tier | Yes (limited) | Yes (limited) | Yes (limited) |
| Persistent volumes | Yes | Yes | Yes (disk) |
| Auto-sleep | Yes | Yes | No |
| Custom domains | Yes | Yes | Yes |
| Health checks | Yes | Yes | Yes |
| Auto-deploy from GitHub | Yes | Yes | Yes |
