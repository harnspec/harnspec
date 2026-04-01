---
status: in-progress
created: 2026-03-30
priority: critical
tags:
- migration
- repository
- branding
- harnspec
created_at: 2026-03-30T23:51:00Z
updated_at: 2026-04-01T14:10:00Z
---

# HarnSpec Migration and Repository Transfer

## Overview

Following the branding reconstruction (Spec 381), this spec outlines the technical steps to complete the migration of the codebase to the new `harnspec` identity and transfer the repository to the `harnspec` organization on GitHub.

## Current Status

The core renaming of package names and project structure has been completed as part of Spec 381. The repository has been successfully created and pushed to the new `harnspec` organization.

## Proposed Changes

### 1. Repository Migration

- [x] Update git remote origin to `https://github.com/harnspec/harnspec.git`.
- [x] Push the current state of the project to the new repository.
- [x] Coordinate with the team to ensure the `harnspec` organization is ready.

### 2. Branding and Naming (Final Polish)

- [x] Rename the root project to `harnspec` in `package.json`.
- [x] Update internal package names from `@harnspec` to `@harnspec`.
- [x] Update all repository URL references from `codervisor/harnspec` to `harnspec/harnspec` in `package.json`, `Cargo.toml`, and README.
- [ ] Update `homepage` and `bugs` URLs in all packages.

### 3. CI/CD Adjustments

- Update GitHub Actions workflows to point to the new repository and organization.
- Update environment variables/secrets for the new organization (e.g., NPM_TOKEN if needed).

## Technical Details

### URL Updates

Change: `https://github.com/codervisor/harnspec` -> `https://github.com/harnspec/harnspec`

### Git Commands

```bash
gh repo create harnspec/harnspec --public --source=. --remote=origin
git push -u origin main
```

## Acceptance Criteria

- [x] Repository is successfully pushed to `https://github.com/harnspec/harnspec`.
- [x] Root `package.json` reflects the name `harnspec`.
- [x] `pnpm install` works with the new package names.
- [x] All repository references in `package.json`, `Cargo.toml`, and README point to the `harnspec` organization.
- [ ] GitHub Actions successfully run in the new repository.
