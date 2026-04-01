---
status: complete
created: '2025-11-18'
tags:
  - ui
  - architecture
  - simplification
priority: high
created_at: '2025-11-18T06:55:41.560Z'
updated_at: '2025-11-28T03:30:16.690Z'
transitions:
  - status: in-progress
    at: '2025-11-18T07:17:02.226Z'
  - status: complete
    at: '2025-11-18T07:19:10.108Z'
completed_at: '2025-11-18T07:19:10.108Z'
completed: '2025-11-18'
---

# UI Standalone Consolidation

> **Status**: ✅ Complete · **Priority**: High · **Created**: 2025-11-18 · **Tags**: ui, architecture, simplification

**Project**: harnspec  
**Team**: Core Development

## Overview

**Problem**: Current `@harnspec/ui` tries to package `@harnspec/web`'s Next.js standalone build, which creates symlink and node_modules distribution failures. When users run `harnspec ui` via `pnpm dlx @harnspec/ui`, the published package can't resolve `next` dependency because:

- Standalone build uses pnpm symlinks that break when packaged
- Symlink rewriting logic is complex and error-prone
- Materializing symlinks hits broken symlink targets
- The two-package separation adds unnecessary indirection

**Solution**: Merge `@harnspec/web` directly into `@harnspec/ui` as a single publishable Next.js app package.

## Design

### Architecture Changes

**Before**:

```
@harnspec/web (workspace-only)
  └─> Next.js app with standalone output
@harnspec/ui (published)
  └─> CLI wrapper + copies @harnspec/web standalone build
```

**After**:

```
@harnspec/ui (published)
  └─> Next.js app + CLI entry point
```

### Implementation Strategy

1. **Move all `@harnspec/web` source to `@harnspec/ui`**:
   - `packages/ui/src/` ← all web app source
   - `packages/ui/public/` ← static assets
   - `packages/ui/next.config.ts`, etc.

2. **Update `@harnspec/ui/package.json`**:
   - Add Next.js dependencies
   - Keep existing CLI bin entry
   - Add Next.js build script
   - Set `output: "standalone"` in Next config

3. **Simplify CLI launcher** (`packages/ui/bin/ui.js`):
   - Production: Just run `node dist/standalone/server.js`
   - No complex copying or symlink rewriting
   - Dependencies bundled by Next.js standalone output

4. **Update monorepo references**:
   - Remove `@harnspec/web` from workspace
   - Update `harnspec ui --dev` to use local `@harnspec/ui`
   - Update build pipelines

### Dev vs Prod Modes

**Dev mode** (monorepo): `harnspec ui --dev`

- Runs `next dev` directly in `packages/ui`
- No standalone build needed

**Prod mode** (published): `npx @harnspec/ui` or `harnspec ui`

- Runs pre-built standalone server
- All dependencies bundled by Next.js

## Plan

- [ ] Create `packages/ui/src/app/` and move all web app source
- [ ] Move `next.config.ts`, `tailwind.config.ts`, etc. to `packages/ui/`
- [ ] Update `packages/ui/package.json` with Next.js deps
- [ ] Simplify `packages/ui/bin/ui.js` launcher
- [ ] Remove `packages/ui/scripts/prepare-dist.mjs` complexity
- [ ] Update `packages/cli/src/commands/ui.ts` for new structure
- [ ] Update monorepo build scripts
- [ ] Archive/remove `packages/web/`
- [ ] Test `pnpm dlx @harnspec/ui` flow
- [ ] Update documentation

## Test

- [ ] `harnspec ui --dev` works in monorepo
- [ ] `pnpm pack` produces valid tarball
- [ ] `pnpm dlx file:./tarball` successfully launches UI
- [ ] Published `@harnspec/ui` resolves all dependencies
- [ ] No "Cannot find module 'next'" errors
- [ ] Ctrl+C/Ctrl+D stop the UI cleanly (from earlier fix)
- [ ] UI connects to filesystem specs correctly

## Notes

**Rationale**: The two-package split was premature optimization. Next.js standalone output is designed to be self-contained and portable. Trying to repackage it breaks that model. By making `@harnspec/ui` the Next.js app itself, we leverage Next.js's built-in bundling and eliminate all symlink/node_modules issues.

**Breaking Changes**: None for users. The `harnspec ui` command and `npx @harnspec/ui` usage remain identical. Only internal package structure changes.
