# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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