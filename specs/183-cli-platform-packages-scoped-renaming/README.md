---
status: complete
created: 2025-12-18
priority: medium
tags:
- packaging
- distribution
- breaking-change
created_at: 2025-12-18T13:48:54.278021Z
updated_at: 2026-01-12T08:27:41.583704878Z
---
# Rename CLI Platform Packages to @harnspec/cli-* Scope

> **Status**: planned · **Priority**: medium · **Created**: 2025-12-18

## Overview

Currently, CLI platform binary packages use inconsistent naming:

- **Unscoped**: `harnspec-darwin-arm64`, `harnspec-darwin-x64`, etc.
- **Scoped**: `@harnspec/mcp-darwin-arm64`, `@harnspec/mcp-darwin-x64`, etc. (MCP)

This creates namespace pollution and inconsistency. We need unified scoped naming while keeping the main `harnspec` CLI package name for user-friendliness.

**Goals:**

- Clean namespace under `@harnspec/` org
- Consistent with MCP pattern (`@harnspec/mcp-*`)
- Maintain simple user-facing name (`harnspec`)
- Professional package management

## Design

### Package Renaming

```
Current                      → New
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
harnspec-darwin-arm64       → @harnspec/cli-darwin-arm64
harnspec-darwin-x64         → @harnspec/cli-darwin-x64
harnspec-linux-x64          → @harnspec/cli-linux-x64
harnspec-linux-arm64        → @harnspec/cli-linux-arm64
harnspec-windows-x64        → @harnspec/cli-windows-x64
```

**Keep unchanged:**

- `harnspec` (main CLI wrapper)
- `@harnspec/mcp` (MCP wrapper)
- `@harnspec/mcp-*` (MCP platform packages)

### Directory Structure

```
packages/cli/binaries/
├── darwin-arm64/
│   ├── package.json          # name: "@harnspec/cli-darwin-arm64"
│   └── harnspec             # binary
├── darwin-x64/
│   ├── package.json          # name: "@harnspec/cli-darwin-x64"
│   └── harnspec
└── ...
```

### Updated Main Package Dependencies

[packages/cli/package.json](../../packages/cli/package.json):

```json
{
  "name": "harnspec",
  "optionalDependencies": {
    "@harnspec/cli-darwin-arm64": "0.2.10",
    "@harnspec/cli-darwin-x64": "0.2.10",
    "@harnspec/cli-linux-x64": "0.2.10",
    "@harnspec/cli-linux-arm64": "0.2.10",
    "@harnspec/cli-windows-x64": "0.2.10"
  }
}
```

### Migration Strategy

**Phase 1: Publish both versions**

- Publish new `@harnspec/cli-*` packages
- Keep old `harnspec-*` packages active
- Main package depends on both (new takes precedence)

**Phase 2: Deprecation warning**

- Add deprecation warning to old packages
- Documentation updates point to new packages

**Phase 3: Full migration** (after 3+ months)

- Remove old packages from optionalDependencies
- Official deprecation on npm

## Plan

### Phase 1: Rename and Publish New Packages

- [x] Rename platform package directories and package.json
  - [x] `darwin-arm64/package.json`: `name` → `@harnspec/cli-darwin-arm64`
  - [x] `darwin-x64/package.json`: `name` → `@harnspec/cli-darwin-x64`
  - [x] `linux-x64/package.json`: `name` → `@harnspec/cli-linux-x64`
  - [x] `linux-arm64/package.json`: `name` → `@harnspec/cli-linux-arm64`
  - [x] `windows-x64/package.json`: `name` → `@harnspec/cli-windows-x64`

- [x] Update main CLI wrapper package
  - [x] [packages/cli/package.json](../../packages/cli/package.json): Replace `optionalDependencies` with scoped names
  - [x] [packages/cli/bin/harnspec-rust.js](../../packages/cli/bin/harnspec-rust.js): Update binary resolution logic to use scoped names

- [x] Update CI/CD workflows
  - [x] Unified dev publishing via [.github/workflows/publish.yml](../../.github/workflows/publish.yml) (use `dev=true`)
  - [x] Update platform package publishing loops (already dynamic via package.json)

- [x] Update build and distribution scripts
  - [x] [scripts/copy-rust-binaries.mjs](../../scripts/copy-rust-binaries.mjs): No changes needed (uses directories, not package names)
  - [x] [scripts/sync-rust-versions.ts](../../scripts/sync-rust-versions.ts): Already handles CLI packages correctly
  - [x] [scripts/publish-platform-packages.ts](../../scripts/publish-platform-packages.ts): Already reads package.json name dynamically

- [x] Documentation updates
  - [x] Update [npm-distribution.md](../../docs/npm-distribution.md)
  - [x] [AGENTS.md](../../AGENTS.md): No package names mentioned
  - [x] README: No platform packages mentioned

### Phase 2: Validation and Testing

- [x] Test local builds
  - [x] `pnpm rust:build` successfully copies binaries
  - [x] Package versions sync correctly
  - [x] Wrapper resolves scoped package names correctly

- [ ] Test dry-run publish
  - [ ] `gh workflow run publish.yml --field dev=true --field dry_run=true`
  - [ ] Verify all package names are scoped correctly

- [ ] Test actual dev publish
  - [ ] Publish dev version with new names
  - [ ] Install and verify: `npm i -g harnspec@dev`
  - [ ] Test all platforms (if possible via CI)

### Phase 3: Deprecation (Future)

- [ ] Add deprecation notices to old packages
- [ ] Monitor adoption of new packages
- [ ] Remove old packages from dependencies after transition period

## Test

- [x] Local build produces packages with correct scoped names
- [x] `harnspec` wrapper correctly resolves new scoped platform packages
- [ ] Dev publish workflow publishes all 5 platform packages + main wrapper
- [ ] Fresh install of `harnspec@dev` works on macOS ARM64
- [ ] Package search shows packages under `@harnspec` org
- [x] Version sync scripts update all packages correctly

## Notes

### Implementation Completion (2025-12-18)

**Completed:**

- ✅ All 5 platform packages renamed to `@harnspec/cli-*` in package.json
- ✅ Main CLI package updated to use scoped names in optionalDependencies
- ✅ Wrapper script (harnspec-rust.js) updated to resolve scoped package names
- ✅ CI/CD workflow (publish.yml with `dev=true`) publishes dev builds
- ✅ Documentation (npm-distribution.md) fully updated
- ✅ Build scripts already handle packages dynamically
- ✅ Local testing confirms wrapper resolves scoped names correctly

**Ready for:**

- Dev publish testing (`gh workflow run publish.yml --field dev=true`)
- Production release in next version

### Why Keep `harnspec` Unscoped?

User experience: `npm i -g harnspec` is simpler than `npm i -g @harnspec/cli`. The main CLI should be easy to discover and install. Only internal platform dependencies need scoping.

### Breaking Change Scope

This is a **non-breaking change** for end users:

- Main package name (`harnspec`) unchanged
- User commands unchanged
- Only internal optional dependencies change

### Publishing Order

**Critical**: Platform packages MUST be published before main wrapper, as implemented in [.github/workflows/publish.yml](../../.github/workflows/publish.yml):

1. Publish all 5 `@harnspec/cli-*` platform packages
2. Then publish `harnspec` main wrapper

### Rollback Strategy

If issues arise, we can:

1. Re-publish old `harnspec-*` names from same binaries
2. Revert optionalDependencies in main package
3. Re-publish main package pointing to old names
