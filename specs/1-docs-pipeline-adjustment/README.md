---
status: in-progress
created: 2026-04-01
priority: high
tags:
- documentation
- ci-cd
- github-pages
- docusaurus
created_at: 2026-04-01T14:15:00Z
---

# Documentation Pipeline Adjustment for GitHub Pages

## Overview

The documentation site (Docusaurus) needs its deployment pipeline adjusted to publish to the new `harnspec/harnspec.github.io` repository, which serves as the official website for HarnSpec.

## Current Status

-   The documentation site is located in `docs-site/`.
-   [x] Integrated `docs-site` into the workspace.
-   [x] Verified that `pnpm docs:build` works correctly from the root.
-   [x] Created the GitHub Action `.github/workflows/docs.yml`.
-   [x] Updated `pnpm-lock.yaml` to include `docs-site` and fixed 404 errors for internal dependencies.
-   [ ] Waiting for the next CI run to confirm success.

## Objectives

1.  **Integrate `docs-site` into the Workspace**: Add `docs-site/` to `pnpm-workspace.yaml` to enable filtering/running from the root.
2.  **Add Documentation Pipeline**: Create a GitHub Action `.github/workflows/docs.yml` that builds and deploys the docs.
3.  **Cross-Repository Deployment**: Configure the workflow to push built site files to the `harnspec/harnspec.github.io` repository.

## Proposed Changes

### 1. Workspace Configuration

Update `pnpm-workspace.yaml` to include the `docs-site` directory:

```yaml
packages:
  - packages/*
  - docs-site
```

### 2. GitHub Action (`docs.yml`)

Create a new workflow that:
-   Runs on pushes to `main` involving `docs/` or `docs-site/`.
-   Builds the Docusaurus project using `pnpm docs:build`.
-   Deploys the output (`docs-site/build/`) to `harnspec/harnspec.github.io` using a deployment action.

### 3. Secret Management

The deployment to a separate repository requires a `DOCUMENTATION_DEPLOY_TOKEN` (a Personal Access Token) with content write access to `harnspec/harnspec.github.io`.

## Technical Details

### Deployment Action

Use `peaceiris/actions-gh-pages` with the following configuration:

```yaml
- name: Deploy to GitHub Pages
  uses: peaceiris/actions-gh-pages@v4
  with:
    external_repository: harnspec/harnspec.github.io
    publish_branch: main
    publish_dir: ./docs-site/build
    personal_token: ${{ secrets.DOCUMENTATION_DEPLOY_TOKEN }}
```

## Acceptance Criteria

- [ ] `docs-site` is part of the pnpm workspace.
- [ ] `pnpm docs:build` runs successfully from the root.
- [ ] `.github/workflows/docs.yml` is created and configured correctly.
- [ ] Any URLs in `docusaurus.config.ts` are verified to point to `harnspec.github.io`.
