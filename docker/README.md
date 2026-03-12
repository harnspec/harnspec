# LeanSpec Docker

Run the LeanSpec UI in a Docker container — useful for CI/CD, team self-hosting, and cloud deployment.

## Quick Start

### Using Docker Compose (recommended)

```sh
docker compose up
```

Open http://localhost:3000 in your browser. Projects are managed through the UI — use the project discovery or add projects manually.

### Using Docker directly

```sh
docker pull ghcr.io/codervisor/leanspec:latest

docker run -p 3000:3000 \
  -v leanspec-data:/root/.lean-spec \
  ghcr.io/codervisor/leanspec:latest
```

## Data Persistence

LeanSpec stores its data in `~/.lean-spec/` inside the container:

| File | Description |
|------|-------------|
| `config.json` | Server and UI configuration |
| `projects.json` | Registered project registry |
| `leanspec.db` | SQLite database (sessions, chat) |

Mount a volume at `/root/.lean-spec` to persist data across container restarts.

## Configuration

| Option | Description |
|--------|-------------|
| `--project <path>` | Auto-register a mounted directory as a project on startup |
| `--host 0.0.0.0` | Bind all network interfaces (included by default) |
| `--no-open` | Skip browser launch (included by default) |
| `PORT` env var | Override the port (default: `3000`) |

### Mounting a project directory

To auto-register a specific project on startup:

```sh
docker run -p 3000:3000 \
  -v leanspec-data:/root/.lean-spec \
  -v /path/to/your/project:/project \
  ghcr.io/codervisor/leanspec:latest \
  --project /project
```

### Custom port example

```sh
docker run -p 8080:8080 \
  -e PORT=8080 \
  -v leanspec-data:/root/.lean-spec \
  ghcr.io/codervisor/leanspec:latest
```

## Building Locally

```sh
docker build -t leanspec docker/
docker run -p 3000:3000 -v leanspec-data:/root/.lean-spec leanspec
```

## Image

The image is published to GitHub Container Registry:

```
ghcr.io/codervisor/leanspec:latest
ghcr.io/codervisor/leanspec:<version>   # e.g. 0.2.27
```

The image uses a two-stage build:
- **Builder stage** (`node:20-slim`): installs `@leanspec/http-linux-x64` and `@leanspec/ui` from npm
- **Runtime stage** (`debian:12-slim`): copies only the Rust binary and pre-built UI static files — no Node at runtime

No Rust compilation happens at build time.
