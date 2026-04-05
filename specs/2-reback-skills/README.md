---
status: complete
created: 2026-03-25
priority: high
tags:
- repository
- agent-skills
- distribution
- breaking-change
parent: 3-universal-skills-initiative
created_at: 2026-03-25T00:00:00Z
updated_at: 2026-04-05T04:26:24.759999800Z
---
# Reorganize Skills Distribution via harnspec/skills

## Overview

### Problem

The project previously lacked a clear, public distribution mechanism for agent skills:

1. **Missing user-facing skill** — The primary SDD methodology skill is not currently distributed via a public registry.
2. **Internal skills leak** — Distributing from the main repository would expose contributor-only skills.
3. **Registry discovery** — Users need a central, trustworthy source for HarnSpec-compatible skills.

### Goals

1. Create `harnspec/skills` repository for public skill distribution.
2. Distribute the primary SDD methodology skill simply as `harnspec`.
3. Ensure internal skills remains in the main repository, never distributed.

## Design

### Skills Distribution via `harnspec/skills`

#### Why `harnspec/skills`

- Provides a clean, standalone repository for users to discover all HarnSpec-compatible skills.
- Enables cleaner installation: `npx skills add harnspec/skills@harnspec`.
- Keeps the main `harnspec/harnspec` repository focused on code, not distribution.

#### Repository structure

```
harnspec/skills
├── README.md                      # Skill catalog + install instructions
├── LICENSE                        # MIT
├── .agents/
│   └── skills/
│       └── harnspec/              # The user-facing SDD methodology skill
│           ├── SKILL.md           # name: harnspec
│           └── references/
│               ├── workflow.md
│               ├── best-practices.md
│               ├── commands.md
│               └── examples.md
└── .github/
    └── workflows/
        └── validate.yml
```

#### Skill Content

The `harnspec` skill is built by recovering and updating the historical `harnspec-sdd` content:
- Rename frontmatter `name: harnspec-sdd` → `name: harnspec`.
- Update any command references to the unified `harnspec` CLI patterns.
- Ensure all skill documentation follows the latest project standards.

### Internal vs. Public Skills

Only the methodology skill is moved to `harnspec/skills`. Contributor-focused skills remain in the main repo:
- `harnspec-development` — Contributor dev workflows.
- `agent-browser` — Internal browser testing.
- `parallel-worktrees` — Parallel implementation workflows.

## Plan

### Phase 1: Create `harnspec/skills` repo
- [x] Create `harnspec/skills` GitHub repository (public, MIT)
- [x] Recover `harnspec` skill content from git history
- [x] Set up CI for skill validation
- [ ] Add README with catalog and install instructions
- [ ] Verify `npx skills add harnspec/skills@harnspec` works

### Phase 2: Update References & Documentation
- [x] Update root `AGENTS.md` and `README.md` to point to the new repo.
- [ ] Update `harnspec init` templates to use the new installation source.
- [ ] Update existing specs that reference old naming or distribution paths.

## Test

- [ ] `npx skills add harnspec/skills@harnspec` installs the skill correctly.
- [ ] Internal skills are NOT available via the public skills registry.
- [ ] `harnspec init` generates correct `AGENTS.md` references pointing to the new source.

## Notes

### Relationship to other specs

- **Absorbs 378** (skills-repo-reorganization).
- **Supersedes 290** (skills-repository-migration) — already archived.
- **Part of 3-universal-skills-initiative** — fulfills the distribution primary goal.
