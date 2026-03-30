---
status: draft
created: 2026-03-30
priority: critical
tags:
- branding
- rebranding
- project-management
created_at: 2026-03-30T22:10:00Z
updated_at: 2026-03-30T22:10:00Z
---

# Brand Reconstruction: HarnSpec

## Overview

LeanSpec is being rebranded to **HarnSpec** to reflect a new focus on **Collaborative Specification and Harness-Driven Development** for small teams (2-10 people). This rebranding involves updating all package names, CLI commands, and documentation. The project will now be hosted on GitHub Pages (`https://harnspec.github.io/`) to maintain a lightweight and community-driven presence.

## Problem

The original "LeanSpec" name and its associated vision (as outlined in Spec 380) have diverged from the current goals of the project. The project needs a fresh identity that emphasizes concurrency, collaboration, and the symbiotic relationship between Specifications and Testing Harnesses.

## Proposed Changes

### 1. Project Naming
- **Project Name**: HarnSpec
- **CLI Command**: `harnspec`
- **npm Scope**: `@harnspec`

### 2. Package Updates
- Root project renamed to `harnspec`.
- CLI package renamed to `harnspec`.
- All `@leanspec/*` packages renamed to `@harnspec/*`.

### 3. Documentation
- Update all instances of "LeanSpec" to "HarnSpec" in the documentation site.
- Move documentation hosting to GitHub Pages (`https://harnspec.github.io/`).
- Remove references to `lean-spec.dev` and `web.lean-spec.dev`.
- Update branding assets.

### 4. Preservation of History
- Existing specifications (`specs/001-*` through `specs/380-*`) will remain unchanged to preserve the project's evolution history.

## Technical Details

### Node.js Monorepo
- Update `package.json` in root and all packages.
- Rename binary in `packages/cli/package.json`.
- Rename `bin/lean-spec.js` to `bin/harnspec.js`.

### Rust Components
- Rename crates from `leanspec-*` to `harnspec-*`.
- Update directory names in the `rust/` folder.

### Verification

### Acceptance Criteria
- [ ] `harnspec --help` works correctly with the new name.
- [ ] All packages publishable under `@harnspec` scope.
- [ ] Documentation site correctly displays "HarnSpec" branding.
- [ ] All tests pass after renaming.
