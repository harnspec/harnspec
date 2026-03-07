---
status: planned
created: 2026-03-07
priority: medium
tags:
- memory
- ai-agents
- llm
- compaction
- mcp
depends_on:
- '359'
created_at: 2026-03-07T05:48:45.441492Z
updated_at: 2026-03-07T05:48:45.441492Z
---

# LLM-Assisted Memory Compaction

## Overview

Spec 359 defines deterministic, rule-based memory compaction (dedup, supersession chains, category grouping). This works for exact duplicates and explicit supersession, but fails on:

1. **Semantic duplicates**: "Always use pnpm" and "Package management must use pnpm, not npm or yarn" — different text, same meaning
2. **Fragmented knowledge**: 5 entries about testing conventions that should be one coherent paragraph
3. **Stale entries**: Facts that were true when written but no longer match the codebase
4. **Index quality**: Terse bullet points lose nuance; an LLM can produce a coherent narrative summary within token budget

This spec adds an **optional LLM layer** on top of the deterministic compaction from spec 359. Deterministic compaction remains the default; LLM-assisted compaction is opt-in and enhances — never replaces — the rule-based foundation.

## Design

### Layered Compaction Pipeline

```
entries/*.json
    │
    ▼
[Stage 1: Deterministic]  ← spec 359 (always runs)
  - Exact dedup
  - Supersession chain resolution
  - Category grouping
    │
    ▼
[Stage 2: LLM-Assisted]   ← this spec (opt-in)
  - Semantic dedup
  - Entry fusion (merge related entries)
  - Staleness detection
  - Narrative index generation
    │
    ▼
compacted/*.md + index.md
```

Stage 1 always runs first, producing a clean intermediate state. Stage 2 operates on Stage 1's output, enhancing it with semantic understanding. If Stage 2 fails or is disabled, Stage 1's output is used as-is.

### Stage 2 Operations

#### Semantic Deduplication

After Stage 1 removes exact duplicates, Stage 2 detects **near-duplicates** by meaning:

- LLM receives pairs/clusters of entries within the same category
- Asked to identify entries that express the same fact in different words
- Produces a merged entry preserving the most complete phrasing
- Original entries are marked as `merged_into: <new-id>` (not deleted)

**Prompt strategy**: Small-batch comparison (5-10 entries per call) to keep context focused and costs low.

#### Entry Fusion

Multiple related entries about the same topic are fused into a single coherent entry:

- Input: 5 entries about "testing conventions" (different commands, patterns, rules)
- Output: 1 consolidated entry covering all testing conventions
- Original entries marked as `merged_into: <fused-id>`

**Constraint**: Fused entries must be **faithful** — no invented facts, only recombination of existing content. The LLM prompt explicitly instructs: "Only include information present in the source entries."

#### Staleness Detection

LLM reviews entries against current project context to flag potentially stale facts:

- Receives entry content + relevant file snippets from the codebase
- Asked: "Is this fact still accurate given the current code?"
- Stale entries are flagged (not auto-deleted) for human review via `lean-spec memory review`
- Optionally triggered on: post-merge, scheduled, or manual `lean-spec memory check-stale`

**Scope limit**: Only checks entries against files they reference (via tags or keyword matching). Does not scan the entire codebase.

#### Narrative Index Generation

Instead of terse bullet points, the LLM generates a coherent summary for `index.md`:

- Input: All compacted entries, grouped by category
- Output: A natural-language summary per category, within token budget
- Preserves all factual content but reads as prose rather than a list
- Falls back to bullet-point format if LLM is unavailable

### Configuration

```json
// .lean-spec/config.json
{
  "memory": {
    "compaction": {
      "llmAssisted": false,
      "llmProvider": "default",
      "semanticDedup": true,
      "entryFusion": true,
      "stalenessCheck": false,
      "narrativeIndex": true
    }
  }
}
```

All LLM features are individually toggleable. `llmAssisted: false` (default) disables all Stage 2 operations.

### Auditability

Every LLM-assisted change is traceable:

- Fused/merged entries retain `merged_into` field pointing to the new entry
- New entries created by fusion have `source: "compaction"` and `source_entries: [<ids>]` in their JSON
- Staleness flags include the LLM's reasoning as a `stale_reason` field
- `lean-spec memory log` shows compaction history (what was merged, when, by which stage)

### Cost Controls

- **Batch size cap**: Maximum 10 entries per LLM call
- **Frequency limit**: LLM compaction runs at most once per `lean-spec memory compact --llm` invocation (never auto-triggered in hot path)
- **Token budget**: LLM calls are bounded by configurable max tokens per compaction run
- **Dry run**: `lean-spec memory compact --llm --dry-run` shows proposed changes without applying

## Plan

- [ ] Define Stage 2 compaction pipeline that operates on Stage 1 output
- [ ] Implement semantic dedup: LLM-based near-duplicate detection within category groups
- [ ] Implement entry fusion: merge related entries into consolidated facts with `merged_into` tracking
- [ ] Implement staleness detection: cross-reference entries against codebase files
- [ ] Implement narrative index generation as alternative to bullet-point format
- [ ] Add `merged_into`, `source`, `source_entries`, `stale_reason` fields to entry schema
- [ ] Add `compaction.llmAssisted` and sub-toggles to config schema
- [ ] Add `lean-spec memory compact --llm [--dry-run]` CLI flag
- [ ] Add `lean-spec memory check-stale` CLI command
- [ ] Add `lean-spec memory log` for compaction audit history
- [ ] Expose LLM compaction via MCP tool (`memory_compact` with `llm` parameter)

## Test

- [ ] Semantic dedup correctly merges "use pnpm" and "package manager must be pnpm" into one entry
- [ ] Entry fusion combines 5 testing-related entries into 1 coherent entry
- [ ] Fused entries contain only facts from source entries (no hallucination)
- [ ] Original entries retain `merged_into` references after fusion
- [ ] Staleness check flags an entry that contradicts current code
- [ ] Staleness check does NOT flag entries that are still accurate
- [ ] Narrative index reads as coherent prose, not a bullet list
- [ ] All LLM operations degrade gracefully when LLM is unavailable (Stage 1 output used)
- [ ] `--dry-run` shows proposed changes without modifying any files
- [ ] Token/cost budget is respected across all LLM calls
- [ ] `memory log` shows full audit trail of compaction operations

## Notes

### Why Not LLM-First?

Deterministic compaction (spec 359) handles 80% of cases with zero cost, zero latency, and full predictability. LLM compaction adds value for the remaining 20% (semantic understanding) but introduces cost, latency, and non-determinism. The layered design ensures the system works well without LLM access and improves with it.

### Provider Flexibility

`llmProvider: "default"` uses whatever LLM provider is configured in LeanSpec's AI settings. This avoids coupling memory compaction to a specific model. Projects can use local models (Ollama), API providers (OpenAI, Anthropic), or runner-provided models.

### Future Considerations

- **Embedding-based clustering**: Use embeddings to pre-cluster entries before LLM fusion (reduces LLM calls)
- **Incremental compaction**: Only process entries added since last compaction (avoid re-processing stable entries)
- **User feedback loop**: Track which fused entries users accept vs. revert, to improve fusion quality over time