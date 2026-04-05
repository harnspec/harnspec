# SDD Workflow

Comprehensive guide to the Spec-Driven Development (SDD) process in HarnSpec projects.

## Lifecycle Phases

### 1. Discovery & Planning
**Goal**: Understand project state and identify needs.

- `harnspec board`: Visualize current status.
- `harnspec search "query"`: Find related specs (avoid duplication).
- `harnspec stats`: Check project health.

### 1b. Proposal Mode (AI Cognitive Workflow)
**Goal**: Transform a vague idea into structured specs.

When dealing with a vague or uncertain idea, act as the "Proposal Engine" using this flow:

1. **Brainstorm & Clarify**: Propose a breakdown and approach to the user and resolve uncertainties.
2. **Ask Confirmation**: Wait for the user to confirm the proposed plan.
3. **Generate Parent Spec**: Use `harnspec create` to form an Umbrella Spec outlining the intent and the feature decomposition list.
4. **Loop Create Children**: Based on the decomposition list in the parent spec, loop through and run `harnspec create` for each sub-feature, linking them to the parent (`harnspec rel add <child> --parent <parent>`).

*(Skip this mode if the user's intent is already clear and exact. Use `harnspec create` directly instead.)*

### 2. Specification
**Goal**: Define clear, actionable requirements.

- `harnspec create <spec>`: Use descriptive names (e.g., `user-login-google-oauth`).
- **Required sections**:
  - Overview
  - Requirements (checklist)
  - Acceptance Criteria
- `harnspec rel add <child> --parent <parent>`: Group related specs.
- `harnspec validate`: Ensure spec quality.

### 3. Implementation
**Goal**: Execute against the spec.

- `harnspec update <spec> --status in-progress`.
- Check off items as completed.
- **Verification**: Run `pnpm test`, `pnpm lint`, `pnpm typecheck` before completion.

### 4. Completion
**Goal**: Verify and close the spec.

- `harnspec view <spec>`: Final review.
- `harnspec update <spec> --status complete`.

## Handling Changes

- **Scope Creep**: If requirements grow, split into a new spec.
- **Blockers**: Use `harnspec rel add <spec> --depends-on <other>`.
- **Refinement**: Update specs during implementation to reflect discoveries.

## Best Practices

- **Context Economy**: Split specs if they exceed 2000 tokens.
- **Relationships**: Use `parent` for decomposition, `depends_on` for technical blockers.
- **Checkboxes**: Use only for verifiable implementation steps.
- **Archive, Don't Delete**: Keep historical context for future reference.
