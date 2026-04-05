---
status: planned
priority: high
tags:
- cli
- cleanup
- methodology
---

# Remove Proposal Mode CLI Commands and Shift to AI Methodology

## Overview

HarnSpec's ultimate goal is high-efficiency human-machine collaboration. A dedicated interactive "proposal" CLI command designed for humans is unnecessary, as AI agents are perfectly capable of handling the "proposal" cognitive workflow autonomously. Therefore, we will remove the `proposal` CLI command and redefine "Proposal Mode" entirely as an AI-driven methodology within our Agent Skills.

## Intent / Scope
- Remove the `harnspec proposal` CLI implementation from the Rust codebase.
- Remove references to the CLI command from the CLI docs.
- Enforce that "Proposal Mode" strictly refers to the AI reasoning protocol (Brainstorm -> Confirm -> Create Parent -> Loop Create Children) defined in `SKILL.md`.

## Requirements
- [ ] Remove `rust/harnspec-cli/src/commands/proposal.rs` and unregister the command in the CLI router.
- [ ] Ensure all references to `harnspec proposal` are purged from `docs-site`.
- [ ] Verify that the CLI binaries compile and pass all existing tests successfully.

## Acceptance Criteria
- [ ] Running `harnspec proposal` returns a "command not found" or "unrecognized subcommand" error.
- [ ] The docs-site builds without referencing the old proposal command.
