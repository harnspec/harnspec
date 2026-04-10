---
status: complete
created: 2026-04-05
priority: high
tags:
- cli
- interactive
- workflow
- umbrella
- proposal
created_at: 2026-04-05T04:52:27.177146800Z
updated_at: 2026-04-06T13:32:33.315595500Z
transitions:
- status: in-progress
  at: 2026-04-05T05:16:51.954266Z
---
# Proposal Mode - Interactive Idea-to-Specs Workflow

## Overview

When users have only a vague idea or a high-level goal but don't know how to implement it, harnspec currently lacks a guided workflow to help them go from fuzzy intent to structured, actionable specs. **Proposal Mode** fills this gap by providing an interactive, AI-assisted process that transforms rough ideas into a well-organized spec hierarchy.

### Problem

1. Users with a **vague idea** — they know roughly what they want but can't articulate concrete features or implementation paths.
2. Users with a **clear goal** — they know the destination but not the route — what features are needed, how to decompose the work, or what the dependencies are.

In both cases, the current SDD workflow requires the user to already understand how to break their intent into specs. This puts a cognitive burden on the user before any AI assistance begins.

### Solution

A new `harnspec proposal` command (or `harnspec propose`) that launches **Proposal Mode** — an interactive, multi-phase workflow:

1. **Propose** — User describes their idea/goal in natural language
2. **Clarify** — System asks clarifying questions to refine scope and intent
3. **Design** — System generates a proposed approach with feature decomposition
4. **Confirm** — User reviews and approves/modifies the proposed plan
5. **Generate** — System creates a parent (umbrella) spec + child specs for each feature
6. **Panorama** — Full spec landscape is displayed for final review
7. **Execute** — User confirms and implementation begins

## Requirements

- [x] Add `harnspec proposal` (alias: `propose`) CLI command
- [x] Implement interactive proposal intake — accept natural language idea/goal description
- [x] Implement clarification phase — ask targeted questions to disambiguate intent, scope, and constraints
- [x] Implement design phase — generate feature decomposition from refined intent
- [x] Display proposed plan in structured format for user review (panorama view)
- [x] Allow user to modify/approve the proposed plan before spec generation
- [x] Auto-generate parent (umbrella) spec capturing the original intent
- [x] Auto-generate child specs for each decomposed feature, linked to parent
- [x] Generate spec panorama showing full hierarchy after creation
- [x] Support non-interactive mode with pre-written proposal document (--file flag)
- [x] Integrate with existing harnspec create/rel commands for spec generation
- [ ] Add i18n support for all user-facing strings (en + zh-CN)
- [x] Update SDD methodology skills (SKILL.md, workflow.md, commands.md) to document proposal workflow
- [x] Add proposal command reference to docs-site CLI documentation (cli.mdx)

## Non-Goals

- This is NOT an AI chat interface — it's a structured, phase-based workflow
- Does not replace manual spec creation — proposal mode is an alternative entry point
- Does not auto-implement specs — only creates them; implementation follows normal SDD lifecycle
- Does not require always-on internet — should work with local LLM providers if configured

## Design

### Architecture

The proposal workflow consists of distinct phases, each producing an artifact that feeds the next:

```
User Input (idea/goal)
    │
    ▼
┌──────────────┐
│   Propose    │  ← Natural language intake
└──────┬───────┘
       │
       ▼
┌──────────────┐
│   Clarify    │  ← Interactive Q&A to refine scope
└──────┬───────┘
       │
       ▼
┌──────────────┐
│   Design     │  ← Feature decomposition + approach
└──────┬───────┘
       │
       ▼
┌──────────────┐
│   Confirm    │  ← User reviews panorama, edits plan
└──────┬───────┘
       │
       ▼
┌──────────────┐
│   Generate   │  ← Create parent spec + child specs
└──────┬───────┘
       │
       ▼
┌──────────────┐
│   Panorama   │  ← Display full spec landscape
└──────────────┘
```

### Parent Spec (Umbrella)

The parent spec captures:
- **Original intent** — the raw idea/goal as described by the user
- **Refined scope** — the clarified version after Q&A
- **Feature list** — summary of all child features with links
- **Acceptance criteria** — how to know the original goal is met

### Child Specs

Each child spec represents one decomposed feature:
- Independently implementable
- Linked to parent via `parent` relationship
- Has its own requirements, scope, and acceptance criteria
- May have `depends_on` relationships with sibling specs

### CLI Interface

```bash
# Start interactive proposal mode
harnspec proposal

# Start with an initial idea
harnspec proposal "I want to add real-time collaboration to my app"

# From a file
harnspec proposal --file proposal.md

# Resume an interrupted proposal
harnspec proposal --resume

# Non-interactive (for AI agent use)
harnspec proposal --non-interactive --file detailed-proposal.md
```

### Panorama View

After spec generation, display a panorama showing:

```
📋 Proposal Panorama: Real-time Collaboration
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
🎯 Parent: 7-realtime-collaboration (umbrella)
   Status: planned | Priority: high

📦 Child Specs:
   ├── 8-websocket-infrastructure    [planned] ← foundation
   ├── 9-presence-awareness          [planned] depends_on: 8
   ├── 10-conflict-resolution        [planned] depends_on: 8
   └── 11-collaborative-cursors      [planned] depends_on: 8, 9

📊 Total: 1 parent + 4 children = 5 specs created
```

## Acceptance Criteria

- Running `harnspec proposal` launches an interactive workflow that guides the user from idea to specs
- The generated parent spec accurately captures the original intent
- Child specs are independently actionable and properly linked
- The panorama view clearly shows the full spec hierarchy
- The workflow can be interrupted and resumed
- All phases provide clear progress indication
- Works with both interactive terminals and AI agent non-interactive mode