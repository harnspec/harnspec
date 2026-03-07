---
status: planned
created: 2026-03-07
priority: high
tags:
- memory
- ai-agents
- architecture
- mcp
- core
created_at: 2026-03-07T02:39:24.545079Z
updated_at: 2026-03-07T02:39:24.545079Z
---

# Project-Scoped Memory for AI Agents

## Overview

AI agents lose project-specific knowledge between sessions. LeanSpec should provide a **project-scoped memory layer** that persists learnings, decisions, patterns, and facts across agent sessions — complementing specs (intent) with operational memory (experience).

Inspired by OpenClaw's memory architecture (Markdown files + semantic search + hybrid retrieval), but adapted for LeanSpec's project-centric, multi-agent model where memory belongs to the **project**, not individual agents.

## Problem

1. **Knowledge evaporation**: Agents rediscover the same patterns, conventions, and pitfalls every session
2. **No shared memory**: When multiple agents (Copilot, Claude, Cursor) work on the same project, each starts cold
3. **Specs ≠ memory**: Specs capture *intent* (what to build), but not *experience* (how things work, what failed, conventions discovered)
4. **Runner-specific silos**: GitHub Copilot's `/memories/repo/` and Claude's `CLAUDE.md` are runner-scoped — knowledge doesn't transfer between agents

## Design

### Architecture: Write-Ahead Log + Materialized Views

Memory follows a **WAL + materialized view** pattern that separates write and read concerns:

- **Write path** (`entries/`): Append-only JSON files, one per fact, timestamp-prefixed — optimized for concurrency, natural ordering, and conflict-free git merges
- **Read path** (`compacted/`, `index.md`): Consolidated Markdown files rebuilt from entries by compaction — optimized for human readability and agent context injection

Agents always **write** to `entries/`. They **read** from `compacted/` and `index.md`. Compaction is the bridge between write-optimized storage and read-optimized output.

### Memory Scopes

| Scope       | Purpose                                            | Storage                   | Injected Into                 |
| ----------- | -------------------------------------------------- | ------------------------- | ----------------------------- |
| **Project** | Conventions, architecture facts, verified commands | `.lean-spec/memory/`      | All agent sessions in project |
| **Session** | Task-specific working context, in-progress notes   | Ephemeral / session-local | Current session only          |

**Key distinction from OpenClaw**: Memory is project-scoped (shared across agents), not agent-scoped (siloed per agent identity).

### File Layout

```
.lean-spec/
  memory/
    index.md              # Auto-generated token-budgeted summary (from compacted/)
    compacted/            # Read-optimized Markdown (rebuilt by compact)
      conventions.md
      architecture.md
      operations.md
      decisions.md
    entries/              # Write-optimized JSON (one file per fact, append-only)
      <timestamp>-<short-uuid>.json   # e.g. 2026-03-07T14-30-00Z-a1b2.json
    daily/                # Session logs (Markdown, auto-pruned)
      YYYY-MM-DD-<uuid>.md
```

**Git tracking**: Only `entries/*.json` is tracked in git. Everything else is derived:

```gitignore
# .gitignore
.lean-spec/memory/index.md
.lean-spec/memory/compacted/
# daily/ tracking is optional
```

This ensures timestamp-prefixed JSON entries merge without conflicts across branches, while derived files are rebuilt locally.

### Entry Format (JSON)

Entries use JSON instead of Markdown because they are **machine-written and machine-consumed**. JSON eliminates YAML frontmatter parsing edge cases and aligns with MCP's native format.

```json
{
  "id": "a1b2c3d4-5678-90ab-cdef-1234567890ab",
  "category": "conventions",
  "content": "Always use pnpm, never npm or yarn.",
  "created": "2026-03-07T14:30:00Z",
  "agent": "copilot",
  "tags": ["pnpm", "tooling"],
  "supersedes": null,
  "confidence": "high"
}
```

| Field        | Type        | Required | Purpose                                                   |
| ------------ | ----------- | -------- | --------------------------------------------------------- |
| `id`         | UUID string | yes      | Unique identifier (full UUID for programmatic references) |
| `category`   | enum        | yes      | `conventions`, `architecture`, `operations`, `decisions`  |
| `content`    | string      | yes      | The fact itself, plain text                               |
| `created`    | ISO 8601    | yes      | Timestamp of creation                                     |
| `agent`      | string      | no       | Which agent/runner wrote this                             |
| `tags`       | string[]    | no       | Grouping labels for search and clustering                 |
| `supersedes` | UUID string | no       | ID of the entry this replaces                             |
| `confidence` | enum        | no       | `high`, `medium`, `low` — affects compaction priority     |

### Memory Types

1. **Entries** (`entries/*.json`): Individual facts, one per file. Write-optimized. All entries are source-of-truth for compaction and index generation.
2. **Compacted views** (`compacted/*.md`): Category-grouped Markdown files rebuilt by compaction. Read-optimized. Human-browsable, injected into agent context.
3. **Daily logs** (`daily/*.md`): Per-session log files with UUID suffix. Searchable but not auto-injected. Auto-pruned after configurable retention period.

### Compaction

Compaction is the mechanism that **fuses** entries into coherent, read-optimized output. It bridges the write path (many small JSON files) and the read path (few consolidated Markdown files).

**Trigger**: `lean-spec memory compact` (manual, post-merge hook, or auto-triggered on read when stale)

**Process**:
1. Scan all entries in `entries/`
2. Group by `category`
3. Within each category, resolve:
   - **Exact duplicates**: Normalize content text, keep newest, discard others
   - **Supersession chains**: Follow `supersedes` references transitively (A supersedes B supersedes C → only A survives). Dangling references (superseded entry not yet merged) are ignored gracefully.
   - **Contradictions**: Last-write-wins (by `created` timestamp). Conflicts logged for `lean-spec memory review`.
   - **Confidence tiebreaker**: When timestamps are equal, `high` > `medium` > `low`
4. Write surviving entries into `compacted/<category>.md` as bullet points
5. Rebuild `index.md` from compacted files with token budget

**Compaction is deterministic** — no LLM required. Dedup, supersession chain resolution, and category grouping are all rule-based. LLM-assisted summarization is a potential v2 enhancement.

**Superseded entries stay on disk** in `entries/` for audit trail. They are excluded from compacted output.

### Index Generation

`index.md` is built from `compacted/` files with a hard token budget:

1. Source from `compacted/*.md` (already deduplicated and resolved)
2. Allocate token budget proportionally across categories (configurable weights)
3. Within each category, prioritize by recency (newest first)
4. Format as terse bullet points under category headers
5. Overflow entries are omitted from index but findable via `memory search`

Example generated `index.md`:

```markdown
<!-- Auto-generated. Do not edit. Rebuild: lean-spec memory compact -->

## Conventions
- Always use pnpm, never npm or yarn
- Use `cargo test -p leanspec-core` for core Rust tests
- Follow DRY principle; extract shared logic

## Architecture
- Monorepo: packages/ (TS), rust/ (Rust), docs-site/ (Docusaurus)
- Rust binaries distributed via npm platform packages

## Operations
- Build: `pnpm build`, Test: `pnpm test`
- CI validates with `pnpm typecheck` before merge
```

### Retrieval Mechanisms

- **Auto-injection**: `index.md` included in system prompt context (token-budgeted, configurable cap)
- **Search**: `lean-spec memory search "query"` — keyword + fuzzy match over all entry files
- **Read**: `lean-spec memory read <id>` — targeted entry access by UUID
- **List**: `lean-spec memory list [--category <cat>]` — list entries with optional filtering
- **MCP tools**: `memory_search`, `memory_read`, `memory_write`, `memory_delete` exposed via MCP server

### Write Mechanisms

- **MCP tool**: `memory_write` — creates a new entry JSON file with timestamp-prefixed filename (never modifies existing files)
- **CLI**: `lean-spec memory add "fact" --category conventions`
- **Auto-capture**: Optional hook that prompts agent to flush durable notes before session ends
- **Delete**: `memory_delete` removes an entry by ID. Agents can also supersede entries by writing a new entry with `supersedes` set.

### Concurrency Safety

The one-file-per-entry design is intentionally chosen for **safe parallel execution**:

| Concern                           | Shared-file design (flawed)     | One-file-per-entry design                  |
| --------------------------------- | ------------------------------- | ------------------------------------------ |
| Two agents write simultaneously   | Last-write-wins → data loss     | Each creates a separate file → no conflict |
| Git merge after parallel sessions | Merge conflicts on shared files | No conflicts (different files touched)     |
| Daily log appends                 | Concurrent appends corrupt file | UUID-suffixed files → no collision         |
| Entry deletion during read        | Partial read / corruption       | Atomic file delete; reader retries         |

**Deduplication**: `memory_write` performs a lightweight similarity check against existing entries (exact match on normalized text). Duplicates are silently skipped. Near-duplicates are flagged for human review via `lean-spec memory review`.

### Branching Strategy

Memory entries travel with the branch they're created on, merging naturally through git:

**Principle**: Only `entries/*.json` is tracked in git. All read-optimized files (`compacted/`, `index.md`) are derived and rebuilt locally after merge.

| Scenario                       | Behavior                                                                                                                                                                 |
| ------------------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Short-lived feature branch     | Entries merge with main on PR. Compaction rebuilds locally.                                                                                                              |
| Long-lived branch              | Periodically rebase/merge main to get latest entries. Normal git workflow.                                                                                               |
| Abandoned branch               | Entries never merge. No cleanup needed.                                                                                                                                  |
| Two agents, same branch        | Timestamp-prefixed entries with UUID suffix — no conflict.                                                                                                               |
| Two agents, different branches | Each writes to own branch. Entries merge cleanly. Compaction resolves dupes post-merge.                                                                                  |
| Revert a PR                    | Entries from that PR stay (additive). Use `memory delete` to explicitly remove.                                                                                          |
| `supersedes` across branches   | If branch B supersedes an entry from branch A that hasn't merged yet, compaction ignores the dangling reference. When A merges, compaction resolves the chain correctly. |

**Post-merge hook** (optional):
```sh
# .git/hooks/post-merge
lean-spec memory compact
```

Automatically rebuilds compacted views after any merge brings in new entries.

### Retention & Decay

- **Superseded entries**: Kept in `entries/` indefinitely (audit trail), excluded from compacted output
- **Daily logs**: Auto-pruned after `dailyRetentionDays` (configurable, default 30)
- **Active entries**: No automatic deletion — compaction consolidates but preserves
- **Confidence decay**: `low` confidence entries are deprioritized in index generation (omitted first when over token budget)
- **Manual cleanup**: `lean-spec memory prune --before 2026-01-01` for explicit cleanup

### Configuration

```json
// .lean-spec/config.json
{
  "memory": {
    "enabled": true,
    "autoInjectTokenBudget": 2000,
    "dailyRetentionDays": 30,
    "autoFlush": true,
    "autoCompact": true
  }
}
```

### Integration with Runner Instruction Files

LeanSpec memory complements, not replaces, runner-specific files:

| Source               | Scope        | LeanSpec Role                                       |
| -------------------- | ------------ | --------------------------------------------------- |
| `CLAUDE.md`          | Claude only  | LeanSpec can sync curated memory → CLAUDE.md        |
| `/memories/repo/`    | Copilot only | LeanSpec can export memory as Copilot repo memories |
| `AGENTS.md`          | All runners  | Static instructions (not memory)                    |
| `.lean-spec/memory/` | All runners  | Dynamic, evolving project knowledge                 |

### Relationship to Spec 159

Spec 159 defines LeanSpec as the "memory layer" for agent orchestration. This spec implements the **concrete memory storage and retrieval system** that enables that vision. Specs remain for *intent*; memory is for *experience*.

## Non-Goals

- Agent-scoped memory (per-agent identity isolation) — use runner-native features
- Embedding/vector search — start with keyword/fuzzy, add semantic later if needed
- Cloud sync of memory — handled by existing cloud sync infrastructure
- Replacing AGENTS.md or runner instruction files
- Graph/ontology-based memory — overkill for expected data volume (50-500 facts); tag-based clustering provides sufficient structure

## Requirements

- [ ] Define entry file format (JSON with id, category, content, created, agent, tags, supersedes, confidence)
- [ ] Implement `memory/entries/`, `memory/compacted/`, and `memory/daily/` directory initialization in `lean-spec init`
- [ ] Add `memory_write` MCP tool — creates new entry JSON file with timestamp-prefixed filename (`<timestamp>-<short-uuid>.json`), performs dedup check
- [ ] Add `memory_read` MCP tool for targeted entry access by UUID
- [ ] Add `memory_search` MCP tool for keyword/fuzzy search over all entries
- [ ] Add `memory_delete` MCP tool for removing entries
- [ ] Add CLI commands: `lean-spec memory add|search|read|list|delete|compact|review|prune`
- [ ] Implement compaction: dedup, supersession chain resolution, contradiction detection, category grouping
- [ ] Implement `index.md` generation from compacted files (token-budgeted, category-proportional, recency-prioritized)
- [ ] Auto-inject generated `index.md` into MCP context with configurable token budget
- [ ] Support daily log files with UUID suffix and auto-pruning
- [ ] Add `memory` section to `.lean-spec/config.json` schema
- [ ] Add `.gitignore` entries for derived files (`index.md`, `compacted/`)
- [ ] Support optional post-merge git hook for auto-compaction
- [ ] Document memory workflow, compaction model, and branching strategy in AGENTS.md template

## Acceptance Criteria

- Agent can write a fact via MCP and retrieve it in a new session
- `index.md` contents appear in agent context without explicit tool call
- Multiple runners (Copilot, Claude) can read the same project memory
- Memory search returns relevant results across all entry files
- **Two agents writing concurrently produce no conflicts or data loss**
- **Git merge after parallel agent sessions has zero conflicts in memory files**
- **Compaction correctly resolves duplicates, supersession chains, and contradictions**
- **Supersession across branches resolves correctly after merge**
- Duplicate entries are detected and silently skipped on write
- Daily logs auto-prune after retention period
- Token budget for auto-injection is respected
- `index.md` and `compacted/` are fully regenerable from `entries/` alone

## Notes

### OpenClaw Architecture Takeaways

| OpenClaw Feature                    | LeanSpec Adaptation                                          |
| ----------------------------------- | ------------------------------------------------------------ |
| `MEMORY.md` (curated long-term)     | `index.md` — auto-generated from compacted entries           |
| `memory/YYYY-MM-DD.md` (daily logs) | `daily/YYYY-MM-DD-<uuid>.md` — UUID-suffixed for concurrency |
| Per-agent SQLite vector index       | Deferred — start with keyword/fuzzy search                   |
| Hybrid search (BM25 + vectors)      | Start with full-text, add vectors in v2                      |
| Memory flush before compaction      | Auto-flush hook before session end                           |
| Agent-scoped workspace isolation    | Project-scoped sharing across agents                         |

### Key Differentiator
OpenClaw's memory is **agent-scoped** (each agent has its own workspace and memory). LeanSpec's memory is **project-scoped** (all agents working on the project share the same memory). This reflects LeanSpec's position as a project-level tool, not an agent runtime.

### Design Decisions

- **JSON for entries, Markdown for reads**: Entries are machine-written and machine-consumed; JSON eliminates YAML parsing edge cases and aligns with MCP's native format. Compacted views and index are human-facing, so they use Markdown.
- **No graph/ontology**: At 50-500 facts, full-text search over flat files is as effective as graph traversal and far simpler. Tags provide implicit clustering without upfront schema design. Graph can be revisited if memory grows past ~500 entries.
- **Deterministic compaction**: No LLM needed for compaction — dedup, supersession, and grouping are rule-based. LLM-assisted summarization is a v2 enhancement.
- **Timestamp-prefixed filenames**: Entry files are named `<ISO-timestamp>-<short-uuid>.json` (e.g. `2026-03-07T14-30-00Z-a1b2.json`). Timestamps provide natural chronological ordering via `ls`, make creation time visible without parsing JSON, and enable filename-based pruning. The short UUID suffix prevents collision when two agents write at the same millisecond. The full UUID remains inside the JSON `id` field for programmatic references (`supersedes`, `memory read`).
- **Branch-local memory**: Entries travel with their branch and merge via git. This avoids cross-branch git operations while leveraging unique filenames for conflict-free merges. Derived files are rebuilt locally post-merge.