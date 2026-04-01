# Claude Code Instructions

## Environment

- **Claude VM**: `gh` CLI unavailable. Git remote is `http://local_proxy@127.0.0.1:*/git/...`
- **Local/Desktop**: `gh` CLI may be available.

## Project Layout

- `packages/ui/` — React/Vite SPA (TypeScript)
- `rust/harnspec-core/` — Core Rust library
- `rust/harnspec-http/` — Axum HTTP server
- `rust/harnspec-cli/` — CLI tool
- `deploy/` — Cloud deployment configs (Railway, Fly.io, Render)
- `docker/` — Dockerfile for production

## Build Commands

```bash
# TypeScript
pnpm install --frozen-lockfile
pnpm build
pnpm typecheck
pnpm test

# Rust (Cargo.toml is in rust/)
cargo check -p harnspec-http --manifest-path rust/Cargo.toml
cargo test --manifest-path rust/Cargo.toml -- --test-threads=1
cargo fmt --manifest-path rust/Cargo.toml -- --check
cargo clippy --manifest-path rust/Cargo.toml -- -D warnings
```

## After Pushing

Use `/watch-ci` to monitor the GitHub Actions pipeline.
