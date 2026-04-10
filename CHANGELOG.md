# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.3] - 2026-04-10

### Fixed

- **AI Session Startup** — Resolved `missing field projectPath` error during session creation by fixing a naming mismatch between the UI (snake_case) and the Backend (camelCase).
- **Spec Events SSE** — Resolved `503 Service Unavailable` error when file watching is disabled; the endpoint now returns a 200 OK stream with keep-alive signals.

### Changed

- **UI Backend Adapter** — Aligned the HTTP backend adapter with the Rust backend's `camelCase` naming convention for all session-related requests.

## [0.1.2] - 2026-04-10

### Added

- **AI Session Lifecycle Management** — Implemented backend for managing AI coding sessions with support for various runners.
- **Session Management Documentation** — Added comprehensive documentation for HarnSpec SDD methodology and CLI usage.
- **Agent Skills** — Introduced `harnspec` and `harnspec-development` skills to standardize agent-led development workflows.

## [0.1.1] - 2026-04-05

### Added

- **HTTP Server CLI Arguments** — Implemented command-line argument structure and boilerplate for the HTTP server to support programmatic control

### Changed

- **SDD methodology** — Transitioned from automated CLI-based ideation to a more flexible, AI-led specification generation process (Cognitive Decomposition)

### Removed

- **Proposal CLI Command** — Removed the `harnspec proposal` command in favor of the new AI-driven workflow

## [0.1.0] - 2026-04-05

### Added

- **Chinese Documentation Links** — Improved accessibility by updating SKILL.md templates with direct Chinese documentation links
- **CLI Argument Parsing** — Enhanced command processing logic in the Rust CLI

## [0.0.3] - 2026-04-05

### Added

- **Proposal Mode** — Interactive ideation for specification generation
  - `harnspec proposal` command to launch the flow
  - Built-in spec conversion capabilities

### Fixed

- **Docusaurus Build** — Added missing `@chevrotain/regexp-to-ast` dependency in `docs-site`
- **CLI Subcommands** — Resolved parsing issues preventing `harnspec ui` from launching
- **Global Installation** — Fixed path resolution errors and unsupported ESM URL schemes on Windows for `@harnspec/cli-wrapper`

### Technical

- **GitHub Actions Runners** — Migrated CI/CD workflows to Node.js 24 compatibility
- **Documentation Strategy** — Added specifications for monorepo skills integration and documentation pipeline

## [0.0.2] - 2026-04-05

### Added

- **UI Package & Command** — Launches a web interface for managing specifications
  - Full-featured web UI in `@harnspec/ui`
  - `harnspec ui` command to launch the interface
  - Support for local development and published assets
- **Global CLI Wrapper** — Optimized global installation experience
  - Lightweight `@harnspec/cli-wrapper` for faster downloads
  - Automatic delegation to the platform-specific CLI
- **Automated Validation** — New end-to-end testing for the demo project
  - `validate-demo` script verifies core CLI commands and workflow
- **Improved Distribution** — New scripts for workspace dependency resolution
  - Automated restoration of `workspace:*` protocols during publishing

### Technical

- **CI/CD Enhancements** — Multi-platform Rust builds and automated npm publishing
- **Docker Support** — Automated multi-arch Docker image builds published to GHCR
- **Monorepo Tooling** — Improved scripts for platform-specific binary distribution

## [0.0.1] - 2026-04-01

### Added

- **Version Reset** — Restarted versioning from 0.0.1

- **Documentation Deployment Pipeline** ([spec 383](https://harnspec.github.io/specs/383)) — Automated documentation publishing to `harnspec.github.io`

- **TUI Multi-Project Management** ([spec 372](https://harnspec.github.io/specs/372)) — Switch between and manage multiple projects from the TUI
- **TUI Sidebar Navigation & Tree View** ([spec 371](https://harnspec.github.io/specs/371)) — Sidebar with sort/filter controls and hierarchical tree view for specs
- **TUI Board View Enhancements** — Collapsible board groups with sort indicator, TOC overlay, and scrollbars
- **TUI Vertical Scrollbars** — Scrollbar widgets in list, board, and detail views
- **TUI Theme Overhaul** — Modern theme with Unicode symbols and RGB color palette
- **Configurable Project Sources** — Local and GitHub project sources via capabilities endpoint
- **GitHub Tab in Create Dialog** — New UI entry point for importing GitHub projects on mobile
- **Commons Clause Licensing** — Adds Commons Clause v1.0 to the MIT license to prevent unauthorized resale
  - Updates `LICENSE` with the Commons Clause condition
  - Updates `package.json` and `Cargo.toml` to use `MIT AND Commons-Clause-1.0`
  - Updates `README.md` with a prominent restrictive license badge

### Changed

- **Default TUI View** — Default view changed from Board to List
- **MCP Deprecation** — Removed MCP integration and deprecated `harnspec-mcp` package

### Fixed

- **TUI Mouse Scroll** — Routes mouse scroll events by cursor position instead of keyboard focus
- **Mobile Blank Page** — Fixes blank page on custom domain for mobile web
- **Clippy Lints** — Resolves `map_or` → `is_some_and` and redundant else branch warnings

### Technical

- **CI Speed-Up** — Removes `--test-threads=1` constraint and skips session tests for faster builds
- **MCP Test Cleanup** — Ignores MCP config test (feature deprecated)
- Adds specs 372–377 covering project management, UX defaults, real-time file watch, spec editing, and testing infrastructure

[Unreleased]: https://github.com/harnspec/harnspec/compare/v0.1.3...HEAD
[0.1.3]: https://github.com/harnspec/harnspec/releases/tag/v0.1.3
[0.1.2]: https://github.com/harnspec/harnspec/releases/tag/v0.1.2
[0.1.1]: https://github.com/harnspec/harnspec/releases/tag/v0.1.1
[0.1.0]: https://github.com/harnspec/harnspec/releases/tag/v0.1.0
[0.0.3]: https://github.com/harnspec/harnspec/releases/tag/v0.0.3
[0.0.2]: https://github.com/harnspec/harnspec/releases/tag/v0.0.2
[0.0.1]: https://github.com/harnspec/harnspec/releases/tag/v0.0.1