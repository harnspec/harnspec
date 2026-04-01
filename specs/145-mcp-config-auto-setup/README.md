---
status: complete
created: '2025-12-05'
tags:
  - init
  - mcp
  - onboarding
  - dx
  - ai-agents
priority: high
created_at: '2025-12-05T02:38:41.988Z'
depends_on:
  - 121-mcp-first-agent-experience
updated_at: '2025-12-05T03:13:34.496Z'
transitions:
  - status: in-progress
    at: '2025-12-05T03:11:45.997Z'
  - status: complete
    at: '2025-12-05T03:13:34.496Z'
completed_at: '2025-12-05T03:13:34.496Z'
completed: '2025-12-05'
---

# MCP Config Auto-Setup During Init

> **Status**: ✅ Complete · **Priority**: High · **Created**: 2025-12-05 · **Tags**: init, mcp, onboarding, dx, ai-agents

**Project**: harnspec  
**Team**: Core Development

## Overview

### Problem: MCP Configuration Is a Manual, Error-Prone Step

User feedback reveals a critical gap in onboarding:

> "no MCP config file auto setup based on ai tools"

**Current State** (after `harnspec init`):

- ✅ Creates AGENTS.md with MCP-first instructions
- ✅ Creates AI tool symlinks (CLAUDE.md, etc.)
- ❌ User must manually configure MCP server in their AI tool
- ❌ Manual process is different for each tool (Claude Desktop, VS Code, Cursor, etc.)
- ❌ Users may not know where config files are located
- ❌ Path resolution is error-prone (absolute vs relative paths)

**Why This Matters**:

- MCP provides **richer context** than CLI (structured data, real-time validation)
- MCP is the **recommended** method per our AGENTS.md
- But onboarding friction means users fall back to CLI or skip MCP entirely
- First impression of "MCP is better" is undermined by setup complexity

### Success Criteria

After implementation:

- ✅ `harnspec init` offers to configure MCP for detected AI tools
- ✅ Generates correct MCP config entries with proper paths
- ✅ Handles tool-specific config file locations
- ✅ Provides copy-paste instructions if auto-setup isn't possible
- ✅ Users can use MCP immediately after init without manual configuration

## Design

### Tool-Specific MCP Configuration

| AI Tool | Config File Location | Format | Notes |
|---------|---------------------|--------|-------|
| Claude Code | `.mcp.json` (project, git-tracked) | JSON | Also supports `~/.claude.json` for user scope |
| VS Code (Copilot) | `.vscode/mcp.json` (workspace) | JSON | |
| Cursor | `.cursor/mcp.json` (workspace) | JSON | |
| Windsurf | `.windsurf/mcp.json` (workspace) | JSON | |
| Gemini CLI | `.gemini/settings.json` (user scope) | JSON | User-level config at `~/.gemini/settings.json` |
| OpenAI Codex | N/A - uses `AGENTS.md` | Markdown | No local MCP; uses remote MCP via API |

**Note on OpenAI Codex**: Codex reads `AGENTS.md` files for instructions (same as our existing approach). For MCP, it uses remote HTTP servers via the OpenAI API, not local configuration files. Our `AGENTS.md` already works with Codex.

### MCP Server Entry Format

**Claude Code** (project-scoped `.mcp.json`):

```json
{
  "mcpServers": {
    "harnspec": {
      "command": "npx",
      "args": ["-y", "@harnspec/mcp", "--project", "${PWD}"]
    }
  }
}
```

**VS Code / Cursor / Windsurf** (workspace config):

```json
{
  "mcpServers": {
    "harnspec": {
      "command": "npx",
      "args": ["-y", "@harnspec/mcp", "--project", "/absolute/path/to/project"]
    }
  }
}
```

### Init Flow Enhancement

```
$ harnspec init

Welcome to LeanSpec! 🚀

? Which AI tools do you use? (auto-detected tools will be pre-selected)
  ◯ Claude Code
  ◯ VS Code (GitHub Copilot)
  ◯ Cursor
  ◯ Windsurf
  ◯ Gemini CLI
  ◯ OpenAI Codex

Creating LeanSpec project...
  ✓ .harnspec/config.json
  ✓ specs/
  ✓ AGENTS.md
  ✓ Tool-specific symlinks (based on selection)

? Configure MCP server for your AI tools? (Recommended)
  ❯ Yes - Auto-configure where possible
    No - I'll configure manually later

Configuring MCP...
  ✓ [Configured for each selected tool]

🎉 LeanSpec initialized!

Next: Open your AI tool and ask "Show me the project board"
```

### Configuration Strategies

**Strategy 1: Project-Scoped Config (Auto) - Claude Code**

- Creates `.mcp.json` at project root
- Git-trackable (team can share config)
- Claude Code supports `${PWD}` for relative paths

**Strategy 2: Workspace-Local Config (Auto)**

- Tools: VS Code, Cursor, Windsurf
- Create config file in project directory (`.vscode/mcp.json`, etc.)
- Fully automated, no user intervention needed

**Strategy 2b: User-Scoped Config (Auto) - Gemini CLI**

- Creates `~/.gemini/settings.json` in user home directory
- Merge with existing settings if present

**Strategy 3: AGENTS.md Only (No MCP Config Needed)**

- Tools: OpenAI Codex
- Codex reads `AGENTS.md` for instructions (already created by init)
- For MCP, Codex uses remote HTTP servers via OpenAI API

**Strategy 4: Config Merge (Careful Auto)**

- If config file already exists, merge rather than overwrite
- Detect existing `mcpServers` entries
- Add `harnspec` entry without removing others
- Backup original file before modification

### Path Resolution

**Critical**: MCP servers need absolute paths.

```typescript
const projectPath = path.resolve(process.cwd());
// Result: /home/user/my-project (not "./")

const mcpConfig = {
  command: "npx",
  args: ["-y", "@anthropic/harnspec-mcp", "--project", projectPath]
};
```

### Edge Cases

1. **Existing MCP config**: Merge, don't overwrite
2. **harnspec already configured**: Skip with message "Already configured"
3. **No write permission**: Fall back to manual instructions
4. **Windows paths**: Handle backslash vs forward slash
5. **Monorepo**: Allow configuring for specific workspace

## Plan

### Phase 1: Core MCP Config Generation

- [ ] Create `generateMcpConfig(projectPath, tool)` function
- [ ] Handle path resolution (absolute paths, cross-platform)
- [ ] Support all target AI tools (Claude Code, VS Code, Cursor, Windsurf)

### Phase 2: Init Flow Integration

- [ ] Add "Configure MCP?" prompt after AI tool selection
- [ ] Implement workspace-local config creation (VS Code, Cursor, etc.)
- [ ] Implement global config instructions (Claude Desktop)
- [ ] Handle existing config merge gracefully

### Phase 3: Non-Interactive Mode

- [ ] Add `--mcp-config` flag for non-interactive init
- [ ] `--mcp-config all` - Configure all detected tools
- [ ] `--mcp-config vscode,cursor` - Configure specific tools
- [ ] `--mcp-config none` - Skip MCP configuration

### Phase 4: Polish & Documentation

- [ ] Update docs with MCP auto-setup feature
- [ ] Add troubleshooting guide for common MCP issues
- [ ] Test with real AI tools end-to-end

## Test

### Unit Tests

- [ ] `generateMcpConfig()` produces valid JSON for each tool
- [ ] Path resolution works on Linux, macOS, Windows
- [ ] Config merge preserves existing entries
- [ ] Backup is created before modifying existing config

### Integration Tests

- [ ] `harnspec init` with Claude Code creates `.mcp.json`
- [ ] `harnspec init` with VS Code creates `.vscode/mcp.json`
- [ ] `harnspec init` with Cursor creates `.cursor/mcp.json`
- [ ] `harnspec init` with Gemini CLI creates `~/.gemini/settings.json`
- [ ] `harnspec init --mcp-config vscode` works non-interactively
- [ ] Existing config is merged, not overwritten

### End-to-End Tests

- [ ] Claude Code with configured MCP can use harnspec tools
- [ ] VS Code with configured MCP can use harnspec tools
- [ ] Cursor with configured MCP can use harnspec tools
- [ ] Gemini CLI with configured MCP can use harnspec tools

### Success Metrics

- [ ] User can use MCP tools immediately after init (no manual steps for workspace-local tools)
- [ ] Clear instructions provided for global config tools
- [ ] No data loss from config merge operations

## Notes

### Why MCP > CLI for AI Agents

| Aspect | MCP | CLI |
|--------|-----|-----|
| Data format | Structured JSON | Text parsing required |
| Real-time feedback | Yes | No |
| Context awareness | Project state known | Stateless |
| Tool integration | Native | Shell execution |
| Error handling | Typed errors | Exit codes |

MCP should be the **default** path, not an advanced option.

### Related Specs

- `121-mcp-first-agent-experience` - MCP-first AGENTS.md and symlinks (prerequisite)
- `072-ai-agent-first-use-workflow` - First interaction protocol (complementary)
- `102-mcp-wrapper-package` - MCP package for npm (used in config)

### Open Questions

1. **Should we auto-detect installed AI tools?**
   - Check for `.vscode/`, `.cursor/` directories
   - Detect Claude Code installation
   - Auto-select tools in init prompt

2. **Should MCP config include project name?**
   - `"harnspec"` vs `"harnspec-myproject"`
   - Matters for multi-project users

3. **How to handle package manager differences?**
   - `npx` (npm), `pnpm dlx`, `yarn dlx`
   - Detect from lockfile and use appropriate runner

4. **Should we create a `harnspec mcp-config` standalone command?**
   - For adding MCP to existing projects
   - `harnspec mcp-config --tool vscode`
