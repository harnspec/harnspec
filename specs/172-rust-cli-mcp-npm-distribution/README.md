---
status: complete
created: '2025-12-18'
tags:
  - distribution
  - npm
  - rust
  - cli
  - mcp
  - publishing
  - packaging
priority: high
created_at: '2025-12-18T02:31:05.718Z'
depends_on:
  - 170-cli-mcp-core-rust-migration-evaluation
updated_at: '2025-12-18T05:34:10.279Z'
transitions:
  - status: in-progress
    at: '2025-12-18T05:33:31.803Z'
  - status: complete
    at: '2025-12-18T05:34:10.279Z'
completed_at: '2025-12-18T05:34:10.279Z'
completed: '2025-12-18'
---

# Rust CLI/MCP npm Distribution Infrastructure

> **Status**: ✅ Complete · **Priority**: High · **Created**: 2025-12-18 · **Tags**: distribution, npm, rust, cli, mcp, publishing, packaging

## Overview

### Problem Statement

The Rust CLI and MCP binaries need to be distributed via npm for easy installation (`npm install -g harnspec`), but npm doesn't natively support Rust binaries. We need a distribution strategy that:

- Downloads only the binary for the user's platform (not all platforms)
- Works with `npm`, `yarn`, `pnpm`, and `npx`
- Maintains familiar npm installation experience
- Supports all target platforms (macOS x64/arm64, Linux x64/arm64, Windows x64)

### Proven Pattern: Optional Dependencies

**Industry Standard** (used by `esbuild`, `swc`, `@tauri-apps/cli`):

```
Main Package (harnspec)
├── bin/harnspec (Node.js wrapper script)
└── optionalDependencies:
    ├── harnspec-darwin-x64
    ├── harnspec-darwin-arm64
    ├── harnspec-linux-x64
    ├── harnspec-linux-arm64
    └── harnspec-windows-x64
```

**How It Works:**

1. User runs `npm install -g harnspec`
2. npm detects platform and installs matching optional dependency
3. Wrapper script (`bin/harnspec`) detects platform and spawns Rust binary
4. Other platform packages are ignored (saves bandwidth)

### Benefits

- ✅ One command: `npm install -g harnspec` works everywhere
- ✅ Works with `npx harnspec` (no global install needed)
- ✅ Only downloads needed binary (~4-10MB vs ~50MB Node.js)
- ✅ Familiar npm workflow for users
- ✅ Compatible with monorepos and lockfiles
- ✅ Fast startup (~19ms vs ~200ms Node.js)

## Design

### Package Structure

**7 npm packages total:**

1. **Main Package** (`harnspec`):
   - Contains wrapper script
   - Lists platform packages as optional dependencies
   - Users install this directly

2. **Platform Packages** (6):
   - `harnspec-darwin-x64` - macOS Intel binary
   - `harnspec-darwin-arm64` - macOS Apple Silicon binary
   - `harnspec-linux-x64` - Linux x86_64 binary
   - `harnspec-linux-arm64` - Linux ARM64 binary (Raspberry Pi, etc.)
   - `harnspec-windows-x64` - Windows x64 binary
   - (Future: `harnspec-windows-arm64` - Windows ARM)

**Same structure for MCP:**

- Main: `@harnspec/mcp`
- Platforms: `@harnspec/mcp-darwin-x64`, `@harnspec/mcp-darwin-arm64`, etc.

### Main Package Configuration

**packages/cli/package.json:**

```json
{
  "name": "harnspec",
  "version": "0.3.0",
  "description": "Lightweight spec methodology for AI-powered development",
  "bin": {
    "harnspec": "./bin/harnspec"
  },
  "optionalDependencies": {
    "harnspec-darwin-x64": "0.3.0",
    "harnspec-darwin-arm64": "0.3.0",
    "harnspec-linux-x64": "0.3.0",
    "harnspec-linux-arm64": "0.3.0",
    "harnspec-windows-x64": "0.3.0"
  },
  "engines": {
    "node": ">=18"
  },
  "repository": {
    "type": "git",
    "url": "https://github.com/codervisor/harnspec.git"
  },
  "license": "MIT"
}
```

**Why Optional Dependencies?**

- npm installs them if available for the platform
- Installation doesn't fail if platform package is missing
- Users can override with environment variables if needed

### Wrapper Script Implementation

**packages/cli/bin/harnspec:**

```javascript
#!/usr/bin/env node
const { spawn } = require('child_process');
const { join } = require('path');
const { existsSync } = require('fs');

// Platform detection
const PLATFORM_MAP = {
  darwin: { x64: 'darwin-x64', arm64: 'darwin-arm64' },
  linux: { x64: 'linux-x64', arm64: 'linux-arm64' },
  win32: { x64: 'windows-x64', arm64: 'windows-arm64' }
};

function getBinaryPath() {
  const platform = process.platform;
  const arch = process.arch;
  
  const platformKey = PLATFORM_MAP[platform]?.[arch];
  if (!platformKey) {
    console.error(`Unsupported platform: ${platform}-${arch}`);
    console.error('Supported: macOS (x64/arm64), Linux (x64/arm64), Windows (x64)');
    process.exit(1);
  }

  // Try to resolve platform package
  const packageName = `harnspec-${platformKey}`;
  try {
    // Check if platform package is installed
    const binaryPath = require.resolve(`${packageName}/harnspec${platform === 'win32' ? '.exe' : ''}`);
    return binaryPath;
  } catch (e) {
    console.error(`Binary not found for ${platform}-${arch}`);
    console.error(`Expected package: ${packageName}`);
    console.error('');
    console.error('To install:');
    console.error(`  npm install -g harnspec`);
    console.error('');
    console.error('If you installed globally, try:');
    console.error(`  npm uninstall -g harnspec && npm install -g harnspec`);
    process.exit(1);
  }
}

// Execute binary
const binaryPath = getBinaryPath();
const child = spawn(binaryPath, process.argv.slice(2), {
  stdio: 'inherit',
  windowsHide: true,
});

child.on('exit', (code) => {
  process.exit(code ?? 1);
});

child.on('error', (err) => {
  console.error('Failed to start harnspec:', err.message);
  process.exit(1);
});
```

**Key Features:**

- ✅ Platform detection (OS + architecture)
- ✅ Clear error messages if binary missing
- ✅ Passes all args/stdio to Rust binary
- ✅ Preserves exit codes
- ✅ Handles Windows `.exe` extension

### Platform Package Configuration

**Example: packages/cli/binaries/darwin-x64/package.json:**

```json
{
  "name": "harnspec-darwin-x64",
  "version": "0.3.0",
  "description": "LeanSpec CLI binary for macOS x64",
  "os": ["darwin"],
  "cpu": ["x64"],
  "main": "harnspec",
  "files": [
    "harnspec"
  ],
  "repository": {
    "type": "git",
    "url": "https://github.com/codervisor/harnspec.git"
  },
  "license": "MIT"
}
```

**Key Fields:**

- `os` / `cpu` - npm uses these to auto-select correct package
- `main` - Points to binary (no extension on Unix)
- `files` - Only includes binary (no source code)

### Directory Structure

**After building:**

```
packages/
├── cli/
│   ├── package.json          # Main package
│   ├── bin/
│   │   └── harnspec         # Wrapper script
│   └── binaries/
│       ├── darwin-x64/
│       │   ├── package.json
│       │   └── harnspec     # Rust binary (from CI)
│       ├── darwin-arm64/
│       │   ├── package.json
│       │   └── harnspec
│       ├── linux-x64/
│       │   ├── package.json
│       │   └── harnspec
│       ├── linux-arm64/
│       │   ├── package.json
│       │   └── harnspec
│       └── windows-x64/
│           ├── package.json
│           └── harnspec.exe
└── mcp/
    ├── package.json          # Main MCP package
    ├── bin/
    │   └── mcp               # MCP wrapper script
    └── binaries/
        └── (same structure as CLI)
```

### Publishing Workflow

**Two-phase publishing process:**

**Phase 1: Publish Platform Packages** (from CI):

```bash
# After building binaries in CI (spec 173)
cd packages/cli/binaries/darwin-x64
npm publish --access public

cd ../darwin-arm64
npm publish --access public

# ... repeat for all platforms
```

**Phase 2: Publish Main Package** (after all platforms):

```bash
cd packages/cli
npm publish --access public
```

**Critical Order:**

- Platform packages **MUST** be published first
- Main package references specific versions of platform packages
- Users get broken installs if main package publishes before platform packages

### Version Synchronization

**All packages use same version:**

```json
{
  "harnspec": "0.3.0",
  "harnspec-darwin-x64": "0.3.0",
  "harnspec-darwin-arm64": "0.3.0",
  // ...
}
```

**Why?**

- Easier to reason about (one version = one release)
- Main package can use exact version match
- Users know what version they have

**Automation:** Use workspace script to sync versions:

```bash
pnpm sync-versions  # Updates all package.json files
```

### Fallback Strategy

**If binary not found:**

```javascript
try {
  binaryPath = require.resolve(`${packageName}/harnspec`);
} catch (e) {
  // Option A: Helpful error message (recommended)
  console.error('Binary not found. Please reinstall: npm install -g harnspec');
  process.exit(1);
  
  // Option B: Fallback to TypeScript (adds complexity)
  // console.warn('Binary not found, using TypeScript fallback (slower)');
  // require('./fallback/cli.js');
}
```

**Recommendation:** Error-only approach

- Keeps wrapper simple
- Forces users to fix installation issues
- No maintenance burden of two implementations

## Plan

### Phase 1: Package Structure Setup

- [ ] Create `packages/cli/binaries/` directory structure
- [ ] Create `package.json` for each platform package (6 total)
- [ ] Create `packages/mcp/binaries/` directory structure
- [ ] Create `package.json` for each MCP platform package (6 total)
- [ ] Update main `package.json` files with optional dependencies

### Phase 2: Wrapper Script Implementation

- [ ] Implement CLI wrapper (`packages/cli/bin/harnspec`)
  - [ ] Platform detection logic
  - [ ] Binary path resolution
  - [ ] Error handling and messages
  - [ ] Process spawning and stdio forwarding
- [ ] Implement MCP wrapper (`packages/mcp/bin/mcp`)
  - [ ] Same logic as CLI wrapper
  - [ ] MCP-specific binary name handling
- [ ] Test wrappers locally with mock binaries

### Phase 3: Version Synchronization

- [ ] Create `scripts/sync-versions.ts` script
  - [ ] Read version from root `package.json`
  - [ ] Update all platform package versions
  - [ ] Update optional dependencies versions
- [ ] Add `pnpm sync-versions` command to root
- [ ] Test version syncing across all packages

### Phase 4: Publishing Scripts

- [ ] Create `scripts/publish-platform-packages.ts`
  - [ ] Iterate through all platform packages
  - [ ] Run `npm publish` for each
  - [ ] Handle errors and retries
  - [ ] Verify publication success
- [ ] Create `scripts/publish-main-packages.ts`
  - [ ] Publish main CLI package
  - [ ] Publish main MCP package
  - [ ] Verify all dependencies are available
- [ ] Add `pnpm publish:platforms` and `pnpm publish:main` commands

### Phase 5: Local Testing

- [ ] Build Rust binaries for local platform
- [ ] Copy binaries to platform package directories
- [ ] Test `npm install` locally (using file: protocol)
- [ ] Test `npx harnspec` without global install
- [ ] Test global install: `npm install -g ./packages/cli`
- [ ] Verify binary detection and execution

### Phase 6: Documentation

- [ ] Document package structure in `packages/cli/README.md`
- [ ] Document publishing process in `docs/publishing.md`
- [ ] Add troubleshooting guide for installation issues
- [ ] Update contributor guide with npm distribution info
- [ ] Document version synchronization process

## Test

### Installation Testing

- [ ] Fresh install: `npm install -g harnspec` on all platforms
- [ ] Update install: `npm update -g harnspec` with existing version
- [ ] npx usage: `npx harnspec list` without global install
- [ ] pnpm install: `pnpm install -g harnspec`
- [ ] yarn install: `yarn global add harnspec`
- [ ] Monorepo install: Add to `package.json` and install

### Platform Detection

- [ ] macOS Intel: Installs `darwin-x64` package
- [ ] macOS Apple Silicon: Installs `darwin-arm64` package
- [ ] Linux x64: Installs `linux-x64` package
- [ ] Linux ARM64: Installs `linux-arm64` package (Raspberry Pi)
- [ ] Windows x64: Installs `windows-x64` package
- [ ] Unsupported platform: Shows clear error message

### Binary Execution

- [ ] Wrapper correctly finds binary path
- [ ] All CLI commands work: `harnspec list`, `harnspec create`, etc.
- [ ] All MCP commands work: `leanspec-mcp` starts server
- [ ] Arguments pass through correctly
- [ ] Exit codes preserved
- [ ] Stdio (stdout/stderr) works correctly

### Error Handling

- [ ] Missing binary shows helpful error message
- [ ] Wrong platform shows supported platforms
- [ ] Installation failure has recovery instructions
- [ ] Binary execution failure is reported clearly

### Version Synchronization

- [ ] `sync-versions` script updates all packages
- [ ] All platform packages use same version
- [ ] Main package references correct versions in optional dependencies
- [ ] Versions match across CLI and MCP packages

### Publishing Workflow

- [ ] Platform packages publish successfully
- [ ] Main package publishes after platforms
- [ ] Published packages are downloadable
- [ ] Package sizes are reasonable (<10MB per platform)
- [ ] npm search finds `harnspec` package

## Notes

### Alternative Approaches Considered

**1. Single Package with All Binaries**

- ✅ Pros: Simpler package structure
- ❌ Cons: Large download (~50MB with all platforms)
- **Decision**: Rejected - wastes bandwidth

**2. Postinstall Script with Downloads**

- ✅ Pros: Flexible, can download from GitHub
- ❌ Cons: Requires network access, slow, unreliable, security concerns
- **Decision**: Rejected - npm ecosystem standard is optional dependencies

**3. Native Node Modules (NAPI)**

- ✅ Pros: npm handles binaries automatically
- ❌ Cons: Requires C bindings, complex build, harder to maintain
- **Decision**: Rejected - wrapper script is simpler

### References

**Successful Implementations:**

- [esbuild](https://github.com/evanw/esbuild/tree/master/npm) - Original pattern
- [@swc/core](https://github.com/swc-project/swc/tree/main/npm) - Rust compiler
- [@tauri-apps/cli](https://github.com/tauri-apps/tauri/tree/dev/tooling/cli/node) - Tauri CLI
- [prisma](https://github.com/prisma/prisma/tree/main/packages/cli) - Database toolkit

**npm Documentation:**

- [Optional Dependencies](https://docs.npmjs.com/cli/v8/configuring-npm/package-json#optionaldependencies)
- [OS and CPU](https://docs.npmjs.com/cli/v8/configuring-npm/package-json#os)
- [Publishing Packages](https://docs.npmjs.com/cli/v8/commands/npm-publish)

### Security Considerations

**Binary Integrity:**

- Use checksums to verify binaries (spec 173 generates these)
- Sign npm packages with `npm publish` (uses npm's infrastructure)
- Consider GitHub Releases as source of truth

**Supply Chain:**

- Platform packages only contain binaries (no source code)
- Wrapper script is simple and auditable
- All packages published from CI (spec 173)

### Future Enhancements

**Auto-Update Support:**

- Add update checker in wrapper script
- Notify users of new versions
- `harnspec update` command to self-update

**Binary Caching:**

- Cache downloaded binaries globally
- Share binaries across projects
- Reduce installation time

**Fallback to TypeScript:**

- Keep TypeScript implementation as fallback
- Detect when Rust binary unavailable
- Warn about performance degradation

### Success Criteria

**Must Have:**

- ✅ One-command installation: `npm install -g harnspec`
- ✅ Works on all target platforms
- ✅ Only downloads needed binary
- ✅ Clear error messages

**Nice to Have:**

- ✅ Works with npx (no global install)
- ✅ Fast installation (<5 seconds)
- ✅ Small package sizes (<10MB per platform)

**Optional:**

- Auto-update notifications
- Binary caching
- Fallback to TypeScript
