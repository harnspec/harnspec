# npm Distribution for Rust Binaries

This document describes how HarnSpec distributes Rust CLI and MCP binaries via npm.

## Overview

HarnSpec uses the **optional dependencies pattern** (used by `esbuild`, `swc`, `@tauri-apps/cli`) to distribute platform-specific Rust binaries:

```
Main Package (harnspec)
├── bin/harnspec.mjs (wrapper script)
└── optionalDependencies:
    ├── harnspec-darwin-x64
    ├── harnspec-darwin-arm64
    ├── harnspec-linux-x64
    ├── harnspec-linux-arm64
    └── harnspec-windows-x64
```

## How It Works

1. User runs `npm install -g harnspec`
2. npm detects platform and installs matching optional dependency
3. Wrapper script (`bin/harnspec.mjs`) detects platform and spawns Rust binary
4. Other platform packages are ignored (saves bandwidth)

## Package Structure

### Main Packages

- **`harnspec`** - CLI main package with wrapper script
- **`@harnspec/mcp`** - MCP server main package with wrapper script

### Platform Packages (CLI)

| Package                      | Platform            |
| ---------------------------- | ------------------- |
| `@harnspec/cli-darwin-x64`   | macOS Intel         |
| `@harnspec/cli-darwin-arm64` | macOS Apple Silicon |
| `@harnspec/cli-linux-x64`    | Linux x86_64        |
| `@harnspec/cli-linux-arm64`  | Linux ARM64         |
| `@harnspec/cli-windows-x64`  | Windows x64         |

### Platform Packages (MCP)

| Package                      | Platform            |
| ---------------------------- | ------------------- |
| `@harnspec/mcp-darwin-x64`   | macOS Intel         |
| `@harnspec/mcp-darwin-arm64` | macOS Apple Silicon |
| `@harnspec/mcp-linux-x64`    | Linux x86_64        |
| `@harnspec/mcp-linux-arm64`  | Linux ARM64         |
| `@harnspec/mcp-windows-x64`  | Windows x64         |

## Directory Structure

```
packages/
├── cli/
│   ├── package.json          # Main package
│   ├── bin/
│   │   ├── harnspec.js      # Current TypeScript wrapper
│   │   └── harnspec-rust.js # Rust binary wrapper
│   └── binaries/
│       ├── darwin-x64/
│       │   ├── package.json
│       │   └── harnspec     # Rust binary
│       ├── darwin-arm64/
│       ├── linux-x64/
│       ├── linux-arm64/
│       └── windows-x64/
└── mcp/
    ├── package.json          # Main MCP package
    ├── bin/
    │   ├── harnspec-mcp.js       # Current wrapper
    │   └── harnspec-mcp-rust.js  # Rust binary wrapper
    └── binaries/
        └── (same structure)
```

## Publishing Workflow

### Prerequisites

1. Rust binaries built for all platforms (via CI)
2. Versions synced across all packages
3. Logged in to npm (`npm login`)

### Step 1: Sync Versions

```bash
pnpm sync-versions
```

This updates:

- All workspace package versions
- All platform package versions

### Step 2: Generate Platform Manifests

```bash
pnpm tsx scripts/generate-platform-manifests.ts
```

This creates `package.json` and `postinstall.js` for all platform packages. The postinstall script sets executable permissions on Unix binaries (npm strips file permissions during packaging).

### Step 3: Publish Platform Packages

```bash
pnpm publish:platforms [--dry-run]
```

This publishes all platform-specific binary packages. Platform packages **must** be published before main packages.

### Step 4: Publish Main Packages

```bash
pnpm publish:main [--dry-run]
```

This publishes `harnspec` and `@harnspec/mcp` main packages.

## Version Synchronization

All packages use the same version (from root `package.json`):

```json
{
  "harnspec": "0.3.0",
  "@harnspec/cli-darwin-x64": "0.3.0",
  "@harnspec/cli-darwin-arm64": "0.3.0",
  ...
}
```

Use `pnpm sync-versions` to synchronize all package versions.

## Wrapper Scripts

### CLI Wrapper (`packages/cli/bin/harnspec-rust.js`)

```javascript
#!/usr/bin/env node
// Platform detection
const PLATFORM_MAP = {
  darwin: { x64: 'darwin-x64', arm64: 'darwin-arm64' },
  linux: { x64: 'linux-x64', arm64: 'linux-arm64' },
  win32: { x64: 'windows-x64' }
};

// Resolve binary from platform package
const packageName = `@harnspec/cli-${platformKey}`;
const binaryPath = require.resolve(`${packageName}/harnspec`);

// Spawn binary with all args
spawn(binaryPath, process.argv.slice(2), { stdio: 'inherit' });
```

### MCP Wrapper (`packages/mcp/bin/harnspec-mcp-rust.js`)

Same pattern as CLI wrapper, but for MCP binary (`harnspec-mcp`).

## Troubleshooting

### Binary permissions error (EACCES)

```
Error: spawn EACCES
```

This means the binary doesn't have execute permissions. This should be automatically fixed by the postinstall script, but if it persists:

```bash
# Manual fix
chmod +x /path/to/node_modules/@harnspec/cli-darwin-arm64/harnspec
```

**Root cause:** npm strips file permissions when creating tarballs. The `postinstall.js` script (included in platform packages) runs `chmod 0o755` on the binary after installation.

### Binary not found

```
Binary not found for darwin-arm64
Expected package: @harnspec/cli-darwin-arm64

To install:
  npm install -g harnspec

If you installed globally, try:
  npm uninstall -g harnspec && npm install -g harnspec
```

### Unsupported platform

```
Unsupported platform: freebsd-x64
Supported: macOS (x64/arm64), Linux (x64/arm64), Windows (x64)
```

### Installation fails

1. Check npm/node versions meet requirements (`node >= 18`)
2. Try clearing npm cache: `npm cache clean --force`
3. Try reinstalling: `npm uninstall -g harnspec && npm install -g harnspec`

## Migration from TypeScript

The current TypeScript CLI remains the default. When Rust binaries are ready:

1. Switch wrapper from `harnspec.js` to `harnspec-rust.js`
2. Add optionalDependencies to `packages/cli/package.json`
3. Publish platform packages first, then main package

## References

- [esbuild npm distribution](https://github.com/evanw/esbuild/tree/master/npm)
- [@swc/core](https://github.com/swc-project/swc/tree/main/npm)
- [npm optionalDependencies](https://docs.npmjs.com/cli/v8/configuring-npm/package-json#optionaldependencies)
