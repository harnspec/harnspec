---
status: complete
created: 2026-06-16
priority: medium
tags:
- branding
- assets
created_at: 2026-06-16T15:49:17.715872600Z
updated_at: 2026-06-16T15:59:21.107792100Z
completed_at: 2026-06-16T15:59:21.107792100Z
transitions:
- status: in-progress
  at: 2026-06-16T15:51:31.504794900Z
- status: complete
  at: 2026-06-16T15:59:21.107792100Z
---



---

# Update brand logo with new Intertwined Monogram

> **Status**: planned · **Priority**: medium · **Created**: 2026-06-16

Replace existing project logo and favicon with the newly chosen Intertwined Monogram Concept B cropped at 680x680.

## Overview

This spec covers replacing the existing project logo and favicon with the newly chosen Concept B (Intertwined Monogram - Original Dark), which has been cropped to 680x680 pixels. We will scale this icon into various sizes (16px to 512px), convert it to `.ico` for favicons, and update the logo references in the Web UI and the Docusaurus documentation site.

## Design

We will run a PowerShell script `C:\Users\YinHe\.gemini\antigravity\brain\30e08ba1-943f-4203-90b3-6ffe85dda108\scratch\generate_assets.ps1` that uses .NET's `System.Drawing` to:
- Resize the cropped 680x680 PNG to 16, 32, 64, 128, 256, and 512 sizes.
- Distribute them to `docs-site/static/img/` and `packages/ui/public/`.
- Convert the 32x32 image to a true ICO file using Win32 API GDI+ helper methods via PowerShell, and overwrite `favicon.ico` at the root, the UI app public folder, and the docs-site static folder.
- Update HTML and TypeScript config files to reference the new PNG/ICO assets.

## Plan

- [x] Run PowerShell script to scale and generate PNG/ICO files for all directories
- [x] Replace `favicon-16x16.png` and `favicon-32x32.png` in `docs-site/static/`
- [x] Update `docs-site/docusaurus.config.ts` to reference the new PNG logo instead of the old SVGs
- [x] Update `packages/ui/src/components/navigation.tsx` to reference the new PNG logo
- [x] Update `packages/ui/index.html` to point to `/favicon.ico` and `/logo-32.png` (or `/logo-128.png` for apple-touch-icon)

## Test

- [x] Validate spec is syntactically sound using `node bin/harnspec.mjs validate`
- [x] Ensure TS packages typecheck cleanly via `pnpm typecheck` (or `node_modules/turbo/bin/turbo run typecheck`)
- [x] Verify that UI app page loads without breaking, using the new logo
- [x] Verify that Docusaurus documentation site loads and shows the new logo

## Notes

- The original cropped PNG source is located at `C:\Users\YinHe\.gemini\antigravity\brain\30e08ba1-943f-4203-90b3-6ffe85dda108\harnspec_logo_concept_b_cropped_680.png`.
- We use `.png` instead of `.svg` since the new logo was generated as a high-quality raster image.
