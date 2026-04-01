# Dev Version Publishing - Quick Reference

Quick commands for publishing development versions of harnspec with Rust implementation.

## TL;DR

```bash
gh workflow run publish.yml --field dev=true    # Publish dev version (all platforms)

# Dry run (build/validate only)
gh workflow run publish.yml --field dev=true --field dry_run=true
```

## What Gets Published

### Platform Packages (Rust Binaries)

- `harnspec-darwin-arm64` - macOS Apple Silicon CLI binary
- `harnspec-darwin-x64` - macOS Intel CLI binary  
- `harnspec-linux-x64` - Linux x64 CLI binary
- `harnspec-linux-arm64` - Linux ARM64 CLI binary
- `harnspec-windows-x64` - Windows x64 CLI binary
- `@harnspec/mcp-darwin-arm64` - macOS Apple Silicon MCP binary
- `@harnspec/mcp-darwin-x64` - macOS Intel MCP binary
- `@harnspec/mcp-linux-x64` - Linux x64 MCP binary
- `@harnspec/mcp-linux-arm64` - Linux ARM64 MCP binary
- `@harnspec/mcp-windows-x64` - Windows x64 MCP binary

### Main Packages (JavaScript Wrappers)

- `harnspec` - CLI main package (detects platform, spawns binary)
- `@harnspec/mcp` - MCP main package (detects platform, spawns binary)
- `@harnspec/ui` - UI package

## Version Format

Dev versions use a workflow-run-id prerelease format:

```
0.2.10-dev.123456789
│      │   └─ GitHub Actions run id
│      └─ dev tag
└─ Base version
```

## Publishing Order (CRITICAL)

1. **Platform packages FIRST** ← Main packages reference these
2. Main packages SECOND

The workflow and scripts handle this automatically.

## Testing Dev Version

**From CI (cross-platform):**

```bash
# Install using dev tag (all platforms work)
npm install -g harnspec@dev
```

**From local publish (single platform):**

```bash
# Install using dev tag (all platforms work)
npm install -g harnspec@dev

# Verify
harnspec --version

# Uninstall
npm uninstall -g harnspec
```

### "Binary not found"

**Cause**: Rust binary not built or copied correctly  
**Fix**: Run `pnpm rust:build` before publishing

### "workspace:* in published package"

**Cause**: Forgot to run `pnpm prepare-publish`  
**Fix**: Only affects full releases, not dev versions (we don't use workspace:* in rust packages)

## Debugging

```bash
# Check if platform package exists
npm view harnspec-darwin-arm64 versions --json

# Check current dev tag version
npm view harnspec dist-tags

# Check what depends on platform packages
npm view harnspec optionalDependencies
```

## See Also

- [Full Publishing Guide](./PUBLISHING.md) - Complete release process
- [npm Distribution](./npm-distribution.md) - Architecture details
