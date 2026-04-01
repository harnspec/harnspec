---
status: complete
created: '2025-11-18'
tags:
  - mcp
  - integration
  - npm-package
  - developer-experience
priority: high
created_at: '2025-11-18T03:06:33.496Z'
updated_at: '2025-11-18T03:29:28.816Z'
transitions:
  - status: in-progress
    at: '2025-11-18T03:24:10.457Z'
  - status: complete
    at: '2025-11-18T03:29:28.816Z'
completed_at: '2025-11-18T03:29:28.816Z'
completed: '2025-11-18'
---

# @harnspec/mcp - MCP Server Integration Wrapper

> **Status**: ✅ Complete · **Priority**: High · **Created**: 2025-11-18 · **Tags**: mcp, integration, npm-package, developer-experience

**Project**: harnspec  
**Team**: Core Development

## Overview

Create a lightweight CLI wrapper that makes `harnspec mcp` more discoverable and easier to use. The existing `harnspec` CLI has many features, making MCP setup less obvious. This dedicated package provides a simple entry point for users to quickly onboard MCP with their preferred IDE/tools.

**Problem**: `harnspec` is a full-featured CLI tool. When users configure MCP servers in their IDE, they need to know to use `harnspec mcp`, which isn't obvious. Also, the package name `harnspec` doesn't clearly indicate MCP functionality.

**Solution**: Ship `@harnspec/mcp` as a thin wrapper. IDEs can call `npx @harnspec/mcp` directly, which just delegates to `harnspec mcp`. This makes the MCP server more discoverable and the package name more intuitive.

## Design

### Package Structure

```
@harnspec/mcp/
├── bin/
│   └── leanspec-mcp.js   # Thin wrapper CLI
├── package.json          # Depends on harnspec
└── README.md             # Quick start guide
```

### How It Works

The package is a **simple passthrough**:

1. User adds MCP server to their IDE config: `npx @harnspec/mcp`
2. When IDE needs the server, npx auto-installs `@harnspec/mcp` and its `harnspec` dependency
3. Script delegates to: `harnspec mcp`
4. MCP server starts and IDE can communicate with it

No interaction needed - the IDE handles everything.

### Usage in IDE Configs

**Claude Desktop** (`claude_desktop_config.json`):

```json
{
  "mcpServers": {
    "leanspec": {
      "command": "npx",
      "args": ["-y", "@harnspec/mcp"]
    }
  }
}
```

**Cline** (VS Code `settings.json`):

```json
{
  "cline.mcpServers": {
    "leanspec": {
      "command": "npx",
      "args": ["-y", "@harnspec/mcp"]
    }
  }
}
```

**Zed** (`settings.json`):

```json
{
  "context_servers": {
    "leanspec": {
      "command": "npx",
      "args": ["-y", "@harnspec/mcp"]
    }
  }
}
```

### Key Design Decisions

**Pure passthrough**: Just delegates to `harnspec mcp`, no logic needed.

**Better naming**: `@harnspec/mcp` is more intuitive than `harnspec mcp` for MCP use cases.

**Auto-install**: npx automatically installs both `@harnspec/mcp` and its `harnspec` dependency when needed.

**No interaction**: MCP servers are called by IDEs, not by users directly. No wizard needed.

**Simpler docs**: Users just copy-paste the config snippet, IDE handles the rest.

## Plan

- [ ] Create minimal package structure in `packages/mcp/`
- [ ] Write simple passthrough script (delegates to `harnspec mcp`)
- [ ] Add `harnspec` as dependency in package.json
- [ ] Create README with config examples for different IDEs
- [ ] Test with Claude Desktop, Cline, Zed
- [ ] Publish to npm as `@harnspec/mcp`
- [ ] Update main docs with `@harnspec/mcp` examples

## Test

- [ ] Config with `npx @harnspec/mcp` works in Claude Desktop
- [ ] Config with `npx @harnspec/mcp` works in Cline
- [ ] Config with `npx @harnspec/mcp` works in Zed
- [ ] Server starts correctly when IDE calls it
- [ ] npx auto-installs dependencies on first run
- [ ] Works on macOS, Windows, Linux
- [ ] Package size is minimal (<5KB)

## Notes

### Why This Approach

**Maximum simplicity**: Just a ~10 line passthrough script.

**No complexity**: No IDE detection, no config merging, no interaction. Users just copy config.

**Better naming**: Package name clearly indicates MCP functionality.

**Zero maintenance**: All logic lives in `harnspec mcp`, wrapper just delegates.

**Discoverability**: Searching for "leanspec mcp" finds the right package immediately.

### Implementation

```javascript
#!/usr/bin/env node
const { spawn } = require('child_process');

// Simply delegate to harnspec mcp
const child = spawn('harnspec', ['mcp'], { stdio: 'inherit' });
child.on('exit', (code) => process.exit(code));
```

### Documentation Example

Users see this in the docs:

> **Quick Setup**
>
> Add to your Claude Desktop config:
>
> ```json
> {
>   "mcpServers": {
>     "leanspec": {
>       "command": "npx",
>       "args": ["-y", "@harnspec/mcp"]
>     }
>   }
> }
> ```
>
> Restart Claude Desktop. Done!

### Dependencies

- `harnspec` - The actual MCP server (peer dependency)

### Related

- Existing `harnspec mcp` command (what this delegates to)
