---
status: draft
created: 2026-03-30
priority: critical
tags:
- migration
- repository
- branding
- harnspec
created_at: 2026-03-30T23:51:00Z
updated_at: 2026-03-30T23:51:00Z
---

# HarnSpec Migration and Repository Transfer

## Overview

Following the rebranding initiative (Spec 381), this spec outlines the technical steps to complete the migration of the codebase to the new `harnspec` identity and transfer the repository to the `harnspec` organization on GitHub.

## Problem

The project is currently named `lean-spec` and hosted at `https://github.com/codervisor/lean-spec` (or similar). To fully embrace the new identity, the repository needs to be moved to `https://github.com/harnspec/harnspec` and all local configuration must reflect this change.

## Proposed Changes

### 1. Repository Migration
- Update git remote origin to `https://github.com/harnspec/harnspec.git`.
- Push the current state of the project to the new repository.

### 2. Branding and Naming
- Rename the root project from `lean-spec` to `harnspec` in `package.json`.
- Update all internal package names and references from `@leanspec` to `@harnspec`.
- Update all references to `lean-spec` in documentation, comments, and configuration files.

### 3. CI/CD Adjustments
- Update GitHub Actions workflows to point to the new repository and organization.
- Update any secrets or environment variables required for the new organization.

## Technical Details

### `package.json` Updates
```json
{
  "name": "harnspec",
  ...
  "scripts": {
    "cli": "node bin/harnspec.js",
    ...
  }
}
```

### Git Commands
```bash
git remote set-url origin https://github.com/harnspec/harnspec.git
git push -u origin main
```

## Acceptance Criteria
- [ ] Repository is successfully pushed to `https://github.com/harnspec/harnspec`.
- [ ] Root `package.json` reflects the name `harnspec`.
- [ ] `pnpm install` works after renaming.
- [ ] CLI can be invoked using the new command (if applicable).
