---
status: complete
created: 2026-01-30
priority: high
tags:
- refactoring
- ui
- dedupe
parent: 259-technical-debt-refactoring
created_at: 2026-01-30T09:19:33.073388Z
updated_at: 2026-01-30T14:42:56.867100Z
completed_at: 2026-01-30T14:42:56.867100Z
transitions:
- status: in-progress
  at: 2026-01-30T09:49:14.484107Z
- status: complete
  at: 2026-01-30T14:42:56.867100Z
---

# UI Utilities Consolidation

## Overview

Consolidate duplicate UI utilities so @harnspec/ui uses the shared implementations from @harnspec/ui-components.

## Design

- Canonical utility implementations live in ui-components.
- @harnspec/ui re-exports from ui-components to avoid breaking imports.

## Plan

- [x] Locate usages of packages/ui/src/lib/date-utils.ts, packages/ui/src/lib/utils.ts, and packages/ui/src/hooks/use-local-storage.ts.
- [x] Move or re-create these utilities in packages/ui-components with identical APIs.
- [x] Update @harnspec/ui imports to point to ui-components equivalents.
- [x] Add re-export stubs in @harnspec/ui if external imports rely on old paths.
- [x] Delete the old utility files and remove the empty packages/ui/src/lib/__tests__/ directory.

## Test

- [x] pnpm pre-release
- [x] No TypeScript errors in @harnspec/ui

## Notes

Keep API signatures unchanged unless explicitly documented in this spec.
