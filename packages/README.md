# HarnSpec Packages

This directory contains the HarnSpec monorepo packages.

## Structure

```
packages/
├── cli/               - harnspec: CLI wrapper for Rust binary
├── mcp/               - @harnspec/mcp: MCP server wrapper
└── ui/                - @harnspec/ui: Primary Vite SPA (web + desktop + shared UI library)
```

## Architecture (Vite + Rust)

```
┌─────────────────┐              ┌────────────────────────┐
│   Web App       │──────► HTTP ►│ Rust HTTP server       │
│  @harnspec/ui   │              │ @harnspec/http-server  │
└─────────────────┘              └────────────────────────┘

┌─────────────────┐
│   CLI / MCP     │──────► Rust binaries (harnspec-cli/mcp)
└─────────────────┘
```

- Rust provides backend for both HTTP server and CLI/MCP commands

## harnspec (CLI)

**JavaScript wrapper for Rust CLI binary.**

Provides platform detection, binary resolution, and templates for `harnspec init`.

### Usage

```bash
npm install -g harnspec
npx harnspec list
npx harnspec create my-feature
```

### Development

```bash
cd rust && cargo build --release
node scripts/copy-rust-binaries.mjs
node bin/harnspec.mjs --version
```

## @harnspec/mcp

**MCP server integration wrapper.**

Delegates to the Rust MCP binary and makes MCP setup discoverable.

```bash
npx -y @harnspec/mcp
```

See [MCP Integration docs](https://harnspec.github.io/docs/guide/usage/ai-assisted/mcp-integration).

## @harnspec/ui (Vite SPA)

Primary web UI package:

- Vite 7 + React 19 + TypeScript 5
- Shared components exported from `@harnspec/ui`
- Served by Rust HTTP server or bundled in Tauri

### Development

```bash
pnpm --filter @harnspec/ui dev       # Vite dev server
pnpm --filter @harnspec/ui build     # build SPA assets
pnpm --filter @harnspec/ui preview   # preview production build
```

## Desktop Repository

The desktop application now lives in a dedicated repository:

- <https://github.com/harnspec/harnspec-desktop>

## Building

```bash
pnpm build
```

Build specific package:

```bash
pnpm --filter @harnspec/ui build
```

## Testing

```bash
pnpm test
```

Run tests for a package:

```bash
pnpm --filter @harnspec/ui test
```

## Publishing

Published packages:

- `harnspec` - CLI (wrapper + Rust binary via optional dependencies)
- `@harnspec/mcp` - MCP server wrapper
- `@harnspec/ui` - Vite SPA bundle

Platform-specific binary packages (published separately):

- `harnspec-darwin-arm64`
- `harnspec-darwin-x64`
- `harnspec-linux-x64`
- `harnspec-windows-x64`

## Migration Notes

- Vite SPA is the primary UI implementation
- Rust remains the single source of truth for backend logic
