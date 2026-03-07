---
status: in-progress
created: 2026-03-06
priority: high
tags:
- sessions
- runners
- git
- worktree
- parallel-execution
- isolation
depends_on:
- 357-shell-based-runner-execution
created_at: 2026-03-06T14:48:09.522503Z
updated_at: 2026-03-07T13:42:13.616514Z
transitions:
- status: in-progress
  at: 2026-03-07T13:15:41.830484Z
---

# Git Worktree-Based Agent Runner Session Execution

## Overview

### Problem

When multiple AI agent sessions run against the same repository, they conflict: uncommitted changes collide, git state is shared, file locks block each other, and one agent's half-finished work corrupts another's context. Today, sessions must run sequentially or risk undefined behavior.

### Solution

Use **git worktrees** to give each agent session an isolated working directory branched from the source repo. Each session operates in its own worktree with its own branch, enabling true parallel execution without interference. After a session completes, its changes are merged back into the target branch and the worktree is cleaned up.

### Scope

**In Scope**:
- Worktree lifecycle management (create, bind to session, cleanup)
- Branch naming conventions and automatic branch creation per session
- Parallel session execution using isolated worktrees
- Merge-back workflow (auto-merge, conflict detection, manual resolution)
- Worktree cleanup and garbage collection
- CLI commands for worktree-based session management
- Integration with shell-based runner execution (spec 357)

**Out of Scope**:
- Distributed execution across multiple machines
- Container-based isolation (Docker, etc.)
- Monorepo-specific subtree strategies
- CI/CD pipeline integration

## Design

### Worktree Lifecycle

```
┌──────────┐     ┌──────────────┐     ┌───────────┐     ┌───────────┐     ┌──────────┐
│  Create   │────▶│ Session Bind │────▶│  Running   │────▶│  Merge    │────▶│ Cleanup  │
│ Worktree  │     │  (branch)    │     │  (agent)   │     │  Back     │     │ Worktree │
└──────────┘     └──────────────┘     └───────────┘     └───────────┘     └──────────┘
      │                                     │                  │
      │                                     ▼                  ▼
      │                              ┌───────────┐     ┌───────────────┐
      │                              │  Failed    │     │ Conflict      │
      │                              │  (abort)   │     │ (manual)      │
      │                              └───────────┘     └───────────────┘
      │                                     │
      ▼                                     ▼
┌──────────────────────────────────────────────────┐
│               Cleanup / GC                        │
└──────────────────────────────────────────────────┘
```

### Worktree Structure

```
project-root/                          # Main worktree
├── .git/
│   └── worktrees/                     # Git-managed worktree metadata
│       ├── session-abc123/
│       └── session-def456/
├── .leanspec-worktrees/               # LeanSpec worktree registry (gitignored)
│   └── registry.json                  # Maps session IDs to worktree paths
└── ...

/tmp/leanspec-worktrees/               # Worktree checkouts (outside repo)
├── project-name-session-abc123/       # Worktree for session abc123
│   ├── ... (full repo checkout)
│   └── .leanspec-session              # Session metadata marker
└── project-name-session-def456/       # Worktree for session def456
```

### Branch Strategy

Each session gets its own branch, auto-created from the current HEAD:

```
main (or current branch)
├── leanspec/session/abc123-fix-auth-bug
├── leanspec/session/def456-add-search-api
└── leanspec/session/ghi789-refactor-db
```

Branch naming: `leanspec/session/{session-id}-{spec-slug}`

### Merge-Back Strategies

| Strategy     | When to Use            | Behavior                                               |
| ------------ | ---------------------- | ------------------------------------------------------ |
| `auto-merge` | Default. Clean changes | Fast-forward or automatic merge into target branch     |
| `squash`     | Multiple small commits | Squash into single commit on target branch             |
| `pr`         | Team workflows         | Create a branch, don't merge (user creates PR)         |
| `manual`     | Conflicts detected     | Leave branch, notify user, provide resolution commands |

### Conflict Handling

1. **Pre-merge check**: Before merging, do a dry-run merge to detect conflicts
2. **No conflicts**: Proceed with configured strategy (auto-merge/squash)
3. **Conflicts detected**: 
   - Keep the session branch and worktree alive
   - Report conflicting files to user
   - Provide CLI command to resolve: `lean-spec session merge <id> --resolve`
   - Optionally open a diff view in the UI

### Parallel Execution Flow

```
User: lean-spec run --spec 101 --spec 102 --spec 103 --parallel

  ┌─────────────────────────────────────────────────┐
  │              Session Orchestrator                │
  │                                                  │
  │  1. Create worktrees for each session            │
  │  2. Spawn runner processes in parallel           │
  │  3. Monitor all sessions                         │
  │  4. Merge completed sessions sequentially        │
  │  5. Report results                               │
  └─────────────────────────────────────────────────┘
       │              │              │
       ▼              ▼              ▼
  ┌─────────┐   ┌─────────┐   ┌─────────┐
  │WT: 101  │   │WT: 102  │   │WT: 103  │
  │ Branch A │   │ Branch B │   │ Branch C │
  │ Agent 1  │   │ Agent 2  │   │ Agent 3  │
  └─────────┘   └─────────┘   └─────────┘
       │              │              │
       ▼              ▼              ▼
  ┌─────────────────────────────────────────────────┐
  │         Sequential Merge (FIFO order)           │
  │  merge A → main, merge B → main, merge C → main │
  └─────────────────────────────────────────────────┘
```

### CLI Integration

```bash
# Run a session in a worktree (auto-creates worktree)
lean-spec run -p "fix auth bug" --worktree

# Run multiple specs in parallel worktrees
lean-spec run --spec 101 --spec 102 --parallel

# List active worktree sessions
lean-spec session worktrees

# Merge a completed session back
lean-spec session merge <session-id>
lean-spec session merge <session-id> --strategy squash

# Resolve merge conflicts
lean-spec session merge <session-id> --resolve

# Clean up a session worktree (abort/discard)
lean-spec session cleanup <session-id>

# GC: remove all stale worktrees
lean-spec session gc
```

### Session-Worktree Data Model

```rust
struct WorktreeSession {
    session_id: String,
    worktree_path: PathBuf,
    branch_name: String,
    base_branch: String,       // Branch this was created from
    base_commit: String,       // Commit SHA at creation
    status: WorktreeStatus,    // Created, Running, Completed, Failed, Merging, Merged, Conflict
    merge_strategy: MergeStrategy,
    created_at: DateTime,
    completed_at: Option<DateTime>,
    merged_at: Option<DateTime>,
    spec_ids: Vec<String>,     // Specs assigned to this session
}

enum WorktreeStatus {
    Created,                   // Worktree exists, agent not started
    Running,                   // Agent active in worktree
    Completed,                 // Agent finished, ready to merge
    Failed,                    // Agent errored out
    Merging,                   // Merge in progress
    Merged,                    // Successfully merged back
    Conflict,                  // Merge conflict, awaiting resolution
    Abandoned,                 // User discarded this session
}

enum MergeStrategy {
    AutoMerge,
    Squash,
    PullRequest,
    Manual,
}
```

## Requirements

Checked items below reflect requirements that are implemented in the current codebase. Acceptance criteria remain open until the CLI worktree flows pass end-to-end.

### Worktree Lifecycle
- [x] Create git worktree for a session with auto-generated branch name
- [x] Worktree created outside the main repo (e.g., `/tmp/leanspec-worktrees/`)
- [x] Session metadata stored in worktree registry
- [x] Worktree removed on cleanup (branch optionally preserved)
- [x] Stale worktree detection and garbage collection (`session gc`)

### Parallel Execution
- [x] `--worktree` flag on `lean-spec run` to enable worktree isolation
- [x] `--parallel` flag to run multiple specs concurrently in separate worktrees
- [x] Each parallel session gets its own runner process
- [x] Session orchestrator monitors all parallel sessions
- [x] Results aggregated and reported after all sessions complete

### Merge & Conflict Resolution
- [x] Pre-merge dry-run conflict detection
- [x] Auto-merge with fast-forward or 3-way merge
- [x] Squash merge option (`--strategy squash`)
- [x] PR mode: leave branch without merging (`--strategy pr`)
- [x] Conflict reporting with file-level detail
- [ ] `session merge --resolve` to retry after manual conflict resolution
- [ ] Sequential merge ordering for parallel sessions (FIFO)

### Integration
- [x] Works with shell-based runner execution (spec 357)
- [x] Session status reflected in the existing session tracking system
- [x] Worktree path passed to runner as working directory
- [x] `lean-spec session worktrees` lists active worktree sessions

## Non-Goals

- No distributed/remote execution — this is single-machine only
- No container isolation — git worktrees provide sufficient separation
- No automatic conflict resolution — conflicts require human input
- No worktree sharing between sessions — strict 1:1 session-to-worktree mapping
- No long-lived worktrees — worktrees are ephemeral per session

## Technical Notes

### Git Worktree Fundamentals
- `git worktree add <path> -b <branch>` creates a new worktree with a new branch
- Worktrees share the same `.git` object store — no full repo clone needed
- Worktrees are lightweight: only the working directory is duplicated
- A branch can only be checked out in one worktree at a time
- `git worktree remove <path>` cleans up the worktree
- `git worktree prune` removes stale worktree references

### Key Files (Existing)
- `rust/leanspec-core/src/sessions/` — Session management
- `rust/leanspec-cli/src/commands/session.rs` — Session CLI commands
- `schemas/runners.json` — Runner definitions

### Performance Considerations
- Worktree creation is fast (~100ms for typical repos)
- Disk usage: only working tree files are duplicated (objects shared)
- For large repos, consider shallow worktrees or sparse checkout
- Cleanup should be aggressive to prevent disk bloat

## Acceptance Criteria

- [ ] `lean-spec run -p "prompt" --worktree` creates a worktree, runs the agent, merges back, and cleans up
- [ ] `lean-spec run --spec 101 --spec 102 --parallel` runs two agents concurrently in separate worktrees
- [ ] Merge conflicts are detected and reported without data loss
- [ ] `lean-spec session worktrees` shows active worktree sessions with status
- [ ] `lean-spec session gc` cleans up stale worktrees and branches
- [ ] No interference between parallel sessions (verified with concurrent writes to same file)

## Progress Check

### 2026-03-07 Verification

Verified against the current codebase and targeted test runs.

Implemented and present in code:
- `rust/leanspec-core/src/sessions/worktree.rs` exists and adds worktree registry, branch naming, merge, cleanup, and GC helpers.
- `rust/leanspec-core/src/sessions/manager/lifecycle.rs` wires worktree metadata into session creation/start/completion and passes worktree paths as the runner working directory.
- `rust/leanspec-cli/src/commands/session.rs` and `rust/leanspec-cli/src/cli_args.rs` add `--worktree`, `--parallel`, `--merge-strategy`, plus `session worktrees|merge|cleanup|gc` commands.
- `.leanspec-worktrees/` is gitignored.

Validation results:
- `cargo test --manifest-path rust/Cargo.toml -p leanspec-core --features 'sessions storage' --quiet` passed.
- `cargo test --manifest-path rust/Cargo.toml -p leanspec-cli --test session -- --nocapture` failed.
- `pnpm typecheck` passed.

Verified blockers still preventing completion:
- The new single-worktree CLI path is not yet verified end-to-end. The added test currently fails before execution because `lean-spec run` still requires either `--prompt` or `--spec`.
- Parallel worktree execution is not yet acceptance-ready. The current implementation fails during sequential merge-back with `Validation error: Merge requires a clean target branch worktree`.
- Failed-session cleanup is not yet verified by passing tests. The current failing-worktree test shows the command completes and records a failed status, but the end-to-end expectation for preserved worktree cleanup is not yet passing.
- `session merge --resolve` is exposed, but current verification did not show behavior distinct from a normal merge retry.

Conclusion:
- Keep status as `in-progress`.
- Do not mark requirements or acceptance criteria complete until the CLI session test suite passes for the worktree flows and the merge/conflict path is verified end-to-end.