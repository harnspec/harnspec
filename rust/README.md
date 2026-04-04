# HarnSpec Rust Implementation

This directory contains the Rust implementation of HarnSpec's core functionality, CLI, and HTTP server.

## Architecture

```
rust/
├── Cargo.toml              # Workspace configuration
├── harnspec-core/          # Core library crate
│   └── src/
│       ├── types/          # Data types (SpecInfo, SpecFrontmatter, etc.)
│       ├── parsers/        # Frontmatter parsing
│       ├── validators/     # Validation logic
│       └── utils/          # Utilities (dependency graph, token counter, etc.)
├── harnspec-cli/           # CLI binary crate
│   └── src/
│       ├── main.rs         # CLI entry point
│       └── commands/       # Command implementations
├── harnspec-http/          # HTTP server binary crate
│   └── src/
│       ├── main.rs         # HTTP server entry point
│       └── ...
└── npm-dist/               # npm distribution helpers
    └── binary-wrapper.js   # CLI binary wrapper for npm
```

## Building

```bash
# Build debug version
cargo build

# Build release version
cargo build --release

# Run tests
cargo test

# Check for issues
cargo clippy
```

## Binary Sizes

The release binaries are optimized for size:
- CLI binary: ~4.1MB
- HTTP binary: ~4.5MB

These are well under the 15MB target and significantly smaller than the Node.js alternatives (~50MB with dependencies).

## Performance

Estimated performance improvements over TypeScript implementation:
- Spec validation: ~10x faster
- Dependency graph: ~10x faster
- Search: ~10x faster
- CLI startup: ~20x faster (no Node.js runtime)

## Dependencies

### Core Crate
- `serde` + `serde_yaml` - YAML parsing
- `serde_json` - JSON serialization
- `walkdir` - File system traversal
- `petgraph` - Dependency graph computation
- `regex` - Pattern matching
- `chrono` - Date/time handling
- `tiktoken-rs` - Token counting

### CLI Crate
- `clap` - Command line parsing
- `colored` - Terminal colors
- `dialoguer` - Interactive prompts
- `indicatif` - Progress bars

### HTTP Crate
- `tokio` - Async runtime
- `sqlx` - SQLite database
- `axum` - Web framework (if used)

## Cross-Compilation

The binaries can be cross-compiled for multiple platforms using GitHub Actions:

- macOS (Intel + Apple Silicon)
- Linux (x64, arm64)
- Windows (x64)

See the CI workflow for cross-compilation configuration.
