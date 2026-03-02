---
status: planned
created: 2026-03-02
priority: high
tags:
- architecture
- rust
- storage
- cloud
- multi-backend
- core
depends_on:
- 329-database-consolidation-multi-backend
created_at: 2026-03-02T01:12:19.671209148Z
updated_at: 2026-03-02T01:12:19.671209148Z
---

# Spec Storage Provider Abstraction

## Overview

Today `SpecLoader` is hardwired to the local filesystem — it reads directly from `PathBuf` via `walkdir` + `std::fs`. This means specs **must** live alongside the project that references them. Teams increasingly need to:

1. **Separate specs from source** — keep specs in a dedicated repo while source code lives in another.
2. **Persist specs in a database** — store specs in PostgreSQL/SQLite for the cloud UI, multi-user editing, search indexing, and audit trails.
3. **Read specs from remote APIs** — pull specs from GitHub repos, GitLab, S3, or other external sources without cloning.
4. **Composite sources** — merge specs from multiple backends (local + cloud, two repos, etc.) into a single unified view.

Spec 067 originally proposed a `SpecStorage` interface (TypeScript), but the Rust rewrite never adopted it. Spec 082 described a "dual-mode" (filesystem vs. database) architecture that was partially stubbed. Spec 213 (Sync Bridge, now deprecated) streamed filesystem changes to a cloud viewer but kept the filesystem as sole source of truth.

This spec introduces a proper **trait-based `SpecProvider` abstraction** in Rust that decouples spec loading, writing, and watching from any single storage backend.

## Design

### `SpecProvider` Trait

```rust
#[async_trait]
pub trait SpecProvider: Send + Sync {
    /// List all spec identifiers (relative paths or keys).
    async fn list(&self) -> Result<Vec<String>>;

    /// Read raw content of a spec by identifier.
    async fn read(&self, id: &str) -> Result<String>;

    /// Write spec content. Returns error if provider is read-only.
    async fn write(&self, id: &str, content: &str) -> Result<()>;

    /// Delete a spec. Returns error if provider is read-only.
    async fn delete(&self, id: &str) -> Result<()>;

    /// Check existence.
    async fn exists(&self, id: &str) -> Result<bool>;

    /// Return provider capabilities.
    fn capabilities(&self) -> ProviderCapabilities;

    /// Optional: subscribe to change events (filesystem watch, DB listen/notify, webhook).
    fn watch(&self) -> Option<Pin<Box<dyn Stream<Item = SpecChangeEvent> + Send>>>;
}

pub struct ProviderCapabilities {
    pub writable: bool,
    pub watchable: bool,
    pub supports_metadata_query: bool,
}
```

### Built-in Providers

| Provider | Backend | Writable | Watchable | Use Case |
|---|---|---|---|---|
| `FilesystemProvider` | Local `specs/` dir | Yes | Yes (notify) | Default, current behavior |
| `DatabaseProvider` | SQLite / PostgreSQL | Yes | Yes (listen/notify) | Cloud UI, multi-user |
| `GitHubProvider` | GitHub Contents API | Read-only | No | Read external repos without cloning |
| `CompositeProvider` | Wraps N providers | Delegates | Merged | Unified view across sources |

### Integration with `SpecLoader`

`SpecLoader` currently takes a `PathBuf`. Refactor it to accept `Arc<dyn SpecProvider>`:

```rust
pub struct SpecLoader {
    provider: Arc<dyn SpecProvider>,
    config: Option<LeanSpecConfig>,
    cache: RwLock<CachedDirectory>,
}
```

All existing filesystem logic moves into `FilesystemProvider`. The cache layer stays in `SpecLoader` and works identically — the cache key becomes the spec identifier string instead of a `PathBuf`.

### Configuration

Add a `providers` section to `.lean-spec/config.json`:

```json
{
  "specs_dir": "specs",
  "providers": [
    { "type": "filesystem", "path": "specs", "mount": "/" },
    { "type": "github", "owner": "acme", "repo": "acme-specs", "path": "specs", "mount": "acme/" },
    { "type": "database", "connection": "env:DATABASE_URL", "mount": "cloud/" }
  ]
}
```

When `providers` is absent, the current filesystem-only behavior is preserved (zero breaking changes).

### Conflict Resolution via Namespace Mounting

With `CompositeProvider`, the same spec ID could theoretically appear in multiple backends. Instead of fragile merge-and-resolve strategies (first-match, latest-modified), the spec uses **namespace mounting** to make collisions structurally impossible:

- Each provider has a `mount` prefix prepended to all its spec IDs.
- A local spec `042-auth-flow` → id `042-auth-flow` (root mount `/`).
- A GitHub spec `042-auth-flow` → id `acme/042-auth-flow` (mount `acme/`).
- When `mount` is omitted, it defaults to `"/"` (root). **Only one provider may use the root mount.**

**Why not merge-and-resolve?** Strategies like "first-match" or "latest-modified" create unpredictable behavior when two providers have conflicting content for the same ID. Namespace mounting is deterministic, transparent, and requires zero conflict resolution logic.

**Listing**: `CompositeProvider::list()` returns all specs across all mounted providers. The namespace prefix in the ID (`acme/042-auth-flow`) tells the user which source it came from.

**Write routing**: `write(id)` inspects the namespace prefix to dispatch to the correct provider. Writing to a read-only provider → `ProviderError::ReadOnly`. Writing with no matching mount → `ProviderError::NoProvider`.

### AI Agent & Tool Compatibility

Today, every write path (MCP tools, CLI commands, HTTP handlers) is hardwired to `std::fs`. This section describes how tools remain compatible with non-filesystem providers without changing their interface.

**What stays the same for AI agents:**
- MCP tool parameters (`update`, `create`, `view`, etc.) are unchanged.
- Content operations (`replacements`, `sectionUpdates`, `checklistToggles`) are pure string transforms — already provider-agnostic.
- Agents call the same tools with the same arguments regardless of backend.

**What changes internally:**
- MCP tools currently receive `specs_dir: &str` and build filesystem paths. They must instead receive `Arc<dyn SpecProvider>` (or a `SpecLoader` backed by one).
- `SpecWriter::atomic_write_file()` (currently `fs::write` + `fs::rename`) must delegate to `provider.write()`.
- `SpecLoader::create_spec()` (currently `fs::create_dir_all` + `fs::write`) must delegate to `provider.write()`.

**New concern — optimistic concurrency:**
- The current `contentHash` mismatch check works for local files. For database/API providers, the provider should also support version-based optimistic locking (e.g., `ETag` or `version` column).
- Add an optional `version` field to the `SpecProvider` trait responses so `SpecLoader` can perform compare-and-swap on writes.

**Net effect:** An AI agent calling `update 042 --status in-progress --replacements [...]` works identically on filesystem, database, or GitHub providers. The agent doesn't know or care which provider backs the spec — the MCP/CLI tool layer handles dispatch transparently.

## Plan

- [ ] Define `SpecProvider` trait and `ProviderCapabilities` in `leanspec-core`
- [ ] Implement `FilesystemProvider` by extracting current `SpecLoader` fs logic
- [ ] Refactor `SpecLoader` to accept `Arc<dyn SpecProvider>` instead of `PathBuf`
- [ ] Refactor `SpecWriter` to call `provider.write()` instead of `fs::write()`
- [ ] Refactor MCP tools to receive provider reference instead of `specs_dir: &str`
- [ ] Refactor HTTP handlers to use provider-backed `SpecLoader`/`SpecWriter`
- [ ] Add optimistic concurrency (`version`/`ETag`) to `SpecProvider` read/write contract
- [ ] Update all call sites (CLI, HTTP, MCP) to construct provider from config
- [ ] Implement `DatabaseProvider` (depends on spec 329 for sqlx migration)
- [ ] Implement `GitHubProvider` (read-only, GitHub Contents API via `octocrab`)
- [ ] Implement `CompositeProvider` with namespace mounting
- [ ] Add `providers` config section parsing
- [ ] Wire up `watch()` for filesystem and database providers

## Test

- [ ] `FilesystemProvider` passes all existing `SpecLoader` integration tests unchanged
- [ ] `DatabaseProvider` round-trips create/read/update/delete specs
- [ ] `GitHubProvider` reads specs from a public repo (integration test with mock)
- [ ] `CompositeProvider` correctly routes reads/writes by namespace across two providers
- [ ] Zero-config (no `providers` key) falls back to filesystem-only behavior
- [ ] Read-only providers reject write/delete with clear error

## Notes

- **Spec 067** proposed this as a TypeScript interface — this spec is the Rust-native equivalent.
- **Spec 082** dual-mode concept is subsumed: database mode becomes `DatabaseProvider`, filesystem mode becomes `FilesystemProvider`.
- **Spec 213** (deprecated Sync Bridge) solved a different problem (streaming to cloud viewer). Providers solve storage location directly rather than bridging.
- **Spec 329** (database consolidation) is a prerequisite for `DatabaseProvider` since it migrates to `sqlx` with async pooling.
- The trait is intentionally simple (read/write/list/delete/exists) to keep implementations straightforward. Higher-level concerns (caching, relationship indexing, hierarchy) remain in `SpecLoader`.
- AI agent tooling (MCP, CLI) does **not** need a new spec — the tool interface (parameters, content ops) is already provider-agnostic. Only the internal plumbing (`fs::write` → `provider.write()`) needs refactoring, which is scoped within this spec.