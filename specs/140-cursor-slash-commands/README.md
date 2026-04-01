---
status: archived
created: '2025-12-04'
tags:
  - cursor
  - slash-commands
  - ai
  - ux
  - ide-integration
priority: medium
created_at: '2025-12-04T07:23:15.903Z'
depends_on:
  - 034-copilot-slash-commands
updated_at: '2026-01-16T07:26:23.286Z'
transitions:
  - status: archived
    at: '2026-01-16T07:26:23.286Z'
---

# Cursor IDE Slash Commands Integration

> **Status**: 📦 Archived · **Priority**: Medium · **Created**: 2025-12-04 · **Tags**: cursor, slash-commands, ai, ux, ide-integration

## Overview

**User Feedback**: "leanspec 是不支持 cursor slash commands 吗，听说 openspec 支持" (User asks if LeanSpec supports Cursor slash commands, noting that OpenSpec supports it)

Add LeanSpec integration directly into Cursor IDE via slash commands, enabling developers to interact with specs naturally through Cursor's AI chat without leaving their coding context.

**Why this matters**: Cursor is one of the most popular AI-native IDEs. Users expect native slash command integrations for their development tools.

## Design

**Cursor Slash Commands** use a `.cursorrules` or `.cursor/rules` file and custom slash command definitions.

**Implementation Options:**

**Option A: Cursor Rules File** (Simplest)

- Create `.cursorrules` template with LeanSpec context
- Include spec discovery, creation, and management prompts
- Users copy template to their projects

**Option B: Custom Slash Commands**

- Implement via Cursor's custom slash command API (if available)
- Register commands like `/harnspec-search`, `/harnspec-create`, `/harnspec-board`

**Option C: MCP Integration** (Recommended)

- Cursor supports MCP servers natively
- LeanSpec already has MCP server (`@harnspec/mcp`)
- Users just need to configure MCP in Cursor settings

**Proposed Commands:**

- `/harnspec-board` - Show Kanban board of specs
- `/harnspec-search <query>` - Search specs
- `/harnspec-create <name>` - Create new spec
- `/harnspec-view <spec>` - View spec content
- `/harnspec-update <spec> --status <status>` - Update spec status

## Plan

- [ ] Research Cursor's current slash command / extension API
- [ ] Document MCP server setup for Cursor (may already work!)
- [ ] Create `.cursorrules` template for LeanSpec workflows
- [ ] Test MCP integration in Cursor IDE
- [ ] Add Cursor setup guide to documentation
- [ ] Compare with OpenSpec's Cursor integration approach

## Test

- [ ] MCP server connects successfully in Cursor
- [ ] Slash commands return expected results
- [ ] Spec creation/update works through Cursor chat
- [ ] Documentation is clear and complete

## Notes

**Competitive Context:**

- OpenSpec reportedly supports Cursor slash commands
- Need to research their implementation approach

**Related:**

- Spec 034: GitHub Copilot Chat slash commands (similar concept, different IDE)
- LeanSpec MCP server already exists - may just need configuration docs

**Open Questions:**

- Does Cursor support custom slash commands natively, or only via MCP?
- What's the exact implementation OpenSpec uses?
- Can we provide a one-click setup experience?
