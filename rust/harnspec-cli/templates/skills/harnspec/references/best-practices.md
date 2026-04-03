# SDD Best Practices

Essential guidelines for effective Spec-Driven Development in HarnSpec.

## Guidelines

### 1. Context Economy
- **Keep specs under 2000 tokens**. Smaller specs are easier for AI to process and understand.
- **Split large initiatives early**. Break down complex tasks into parent and children.
- **Focus on the "Why" and "What"**. Let the implementation details emerge during the "how".

### 2. Discover First
- **Always run `harnspec board`** to see the overall state.
- **Search before creating**. Avoid duplication and maximize reuse.
- **Identify dependencies early** using `depends_on`.

### 3. Clear Requirements
- **Actionable checklist items** (`- [ ]`).
- **Independently verifiable goals**.
- **Avoid vague terms** (e.g., "improve things"). Use "Refactor X to use Y".
- **Non-Goals** section to prevent scope creep.

### 4. Meaningful Relationships
- **Parent/Child** for decomposition.
- **Depends On** for technical blockers.
- **Do not mix** both for the same pair of specs.

### 5. Verified Implementation
- **Run all checks** (`pnpm test`, `pnpm lint`, `pnpm typecheck`) before closing a spec.
- **Check implementation against requirements**, not just status.
- **Document trade-offs** made during implementation within the spec.

## Common Pitfalls

- **Empty Specs**: Never create a spec without initial content. Use `harnspec create --content "..."`.
- **Manual Edits**: Avoid modifying frontmatter manually. Use `harnspec update` and `harnspec rel`.
- **Neglecting Relationships**: Stale dependencies lead to confusion. Keep them updated.
- **Overcrowded Board**: Archive obsolete specs. Keep the board focused on current milestones.

## Acceptance Criteria Checklist

- [ ] Clear overview and problem statement.
- [ ] Actionable requirement checklist.
- [ ] Verified dependencies.
- [ ] Passes `harnspec validate`.
- [ ] Under 2000 tokens.
