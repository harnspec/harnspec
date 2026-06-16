---
status: complete
created: 2026-06-16
priority: medium
tags:
- cleanup
- assets
created_at: 2026-06-16T16:05:03.607223900Z
updated_at: 2026-06-16T16:15:20.325055700Z
completed_at: 2026-06-16T16:15:20.325055700Z
transitions:
- status: in-progress
  at: 2026-06-16T16:05:15.441978100Z
- status: complete
  at: 2026-06-16T16:15:20.325055700Z
---





# Cleanup unused brand assets and blog references

> **Status**: planned · **Priority**: medium · **Created**: 2026-06-16

Remove qr-code, social-card, and blog-related content and scripts.

## Overview

We are cleaning up legacy branding and blog elements to align with the new brand logo. This includes removing old `qr-code.png`, `social-card.png`, and `social-github.png`, deleting the blog images directory, and updating scripts/docs to remove blog validation logic since the blog features are disabled.

## Design

- Use file system utilities to delete the unreferenced assets.
- Update `docs-site/docusaurus.config.ts` to replace the `og:image` path with the new logo.
- Clean up `docs-site/scripts/validate-mdx-syntax.js` to bypass searching for non-existent Chinese blog directories.
- Strip mentions of blog validation from instructions.

## Plan

- [x] Physically delete `qr-code.png`, `social-card.png`, and `social-github.png`
- [x] Physically delete the entire `docs-site/static/img/blog/` directory
- [x] Change `image` in `docusaurus.config.ts` to point to `img/logo-256.png`
- [x] Remove blog paths and options from `validate-mdx-syntax.js`
- [x] Clean up `README.md` and `AGENTS.md` in `docs-site/` to remove blog references

## Test

- [x] Verify specs validation: `node bin/harnspec.mjs validate`
- [x] Verify project builds and typechecks cleanly: `npx pnpm typecheck`
- [x] Verify Docusaurus site compiles cleanly: `npx pnpm docs:build`

## Notes

- Checked files: all checks passed successfully.
- Blog assets are completely removed, and Docusaurus meta tags now reference the new brand icon.
