---
status: complete
created: 2026-04-01
priority: high
tags:
- repository
- agent-skills
- cli
- npm
- integration
parent: 3-universal-skills-initiative
created_at: 2026-04-01T22:30:00Z
updated_at: 2026-04-05T04:26:46.080995100Z
---
# Integrated Skills Management via Monorepo

## Overview

### Problem

The current strategy of separating the official SDD methodology skills into a separate repository (`harnspec-skills`) creates friction for development and maintenance. Furthermore, relying on `npx skills add` for core skills installation is an external dependency that doesn't provide a cohesive HarnSpec user experience.

### Goals

1. **Consolidate Source**: Move the official skills source from the separate `harnspec-skills` repo into the main monorepo at `packages/skills`.
2. **NPM-Based Distribution**: Publish the skills as a dedicated `@harnspec/skills` package.
3. **Core CLI Integration**: 
   - Integrate skill injection into `harnspec init`.
   - Add a first-class `harnspec skills install` command.
4. **Automated Replacement**: Ensure that subsequent installations or updates overwrite existing skill files to keep users up-to-date.

## Design

### 1. Repository Structure

Move the contents of `harnspec-skills` to `packages/skills`.

```
harnspec/ (monorepo)
├── packages/
│   └── skills/
│       ├── package.json        # name: @harnspec/skills
│       ├── .agents/
│       │   └── skills/
│       │       └── harnspec/   # The SDD methodology skill
│       │           ├── SKILL.md
│       │           └── references/
│       └── README.md           # Registry documentation
└── ...
```

### 2. CLI Implementation (Rust)

#### `harnspec init` Update
- After initializing the project structure, prompt the user: `? Would you like to inject the official SDD methodology skills? (Y/n)`
- If `Y` (or `-y` flag is present):
  - Execute the skill injection logic.

#### `harnspec skills install` Command
- A new CLI command to specifically handle skill installation/updates.
- Logic:
  1. Determine the source (likely downloading the `@harnspec/skills` tarball from npm).
  2. Extract the `.agents/skills/harnspec/` directory.
  3. Copy its contents to the project's target directory: `.agents/skills/harnspec/`.
  4. If files already exist, overwrite them.

### 3. Distribution Strategy

- Use `pnpm` to manage the versioning and publishing of `@harnspec/skills`.
- The CLI will fetch the latest version from npm when `install` is called, ensuring users always get the freshest methodology.

## Plan

### Phase 1: Repository Consolidation
- [x] Create `packages/skills` directory.
- [x] Copy content from `harnspec-skills` (SKILL.md, references, etc.) to `packages/skills/.agents/skills/harnspec/`.
- [x] Initialize `packages/skills/package.json`.
- [x] Add `packages/skills` to `pnpm-workspace.yaml`.
- [x] Update `AGENTS.md` and `README.md` to reflect the new source location.

### Phase 2: CLI Integration (Rust)
- [x] Implement `skills install` command logic in `rust/harnspec-cli/src/commands/skill.rs`.
- [x] Add interactive prompt to `harnspec init` in `rust/harnspec-cli/src/commands/init.rs`.
- [x] Implement the downloader/extractor for the npm package (via npx + bin script).

### Phase 3: CI/CD & Testing
- [x] Update the publishing script to include `@harnspec/skills`.
- [ ] Test the full flow: `harnspec init` followed by AI interaction using the injected skills.
- [ ] Verify `harnspec skills install` correctly updates existing skills.

## Test

- [ ] `packages/skills` passes `pnpm lint` and contains all required files.
- [ ] Running `harnspec init -y` results in a `.agents/skills/harnspec` folder containing the methodology.
- [ ] Running `harnspec skills install` updates the files and provides feedback.
- [ ] The CLI does not crash if npm is unreachable (graceful failure).

## Notes

- This change effectively turns `harnspec-skills` into a legacy repository once the migration is complete.
- The use of a separate npm package `@harnspec/skills` keeps the core CLI binary small while allowing for independent methodology updates.
