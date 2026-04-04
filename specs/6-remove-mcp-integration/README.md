---
status: in-progress
created: 2026-04-01
priority: high
tags:
- repository
- refactor
- mcp
- cli
- cleanup
parent: 4-integrated-skills-management
created_at: 2026-04-01T23:30:00Z
updated_at: 2026-04-04T10:04:35.565213500Z
---

# Spec 6: 去除 MCP 集成，全面转向 CLI + Skills 驱动

## 概述

### 背景与问题

目前 HarnSpec 同时支持 MCP (Model Context Protocol) 和 CLI 两种方式与 AI 助手集成。虽然 MCP 提供了一种标准化的协议，但它增加了项目的维护复杂性（需要专门的 Rust 包、Node 包以及复杂的发布流程），且对用户的配置要求较高。

随着 SDD Skills (基于 `npx skills`) 和 HarnSpec CLI 功能的完善，通过 CLI 配合 Skills 驱动已经能够提供更轻量、更灵活且更通用的 AI 辅助开发体验。

### 目标

1. **简化架构**：移除所有 MCP 相关的源代码和包，降低项目维护负担。
2. **统一入口**：全面转向以 CLI 为中心，配合 Agent Skills 来驱动 AI 助手的集。
3. **清理文档**：从 README、文档和模板中删除所有关于 MCP 的配置说明，避免误导用户。
4. **提升体验**：强化 `harnspec skills install` 作为 AI 集成的推荐方式。

## 设计方案

### 1. 待删除的代码与包

#### Rust 部分
- `rust/harnspec-mcp/`：完整的 MCP 服务器实现。
- `rust/harnspec-cli/src/commands/mcp.rs`：CLI 中的 `mcp` 命令入口。
- `rust/harnspec-cli/src/commands/init/mcp_config.rs`：初始化时的 MCP 配置逻辑。
- `rust/npm-dist/mcp-wrapper.js`：用于 npm 分发的 MCP 包装脚本。

#### Node/TypeScript 部分
- `packages/mcp/`：MCP 相关的 Node 包。
- 根目录 `package.json` 中的 MCP 相关 scripts（如 `build:mcp` 等）。
- 各个平台的 MCP 包（如 `@harnspec/mcp-windows-x64` 等，通过脚本生成的部分）。

### 2. 待清理的文档与模板

- `README.md` (根目录)：移除 "AI Integration" 章节中关于 MCP 的配置代码块。
- `packages/cli/templates/`：更新所有 `AGENTS.md` 模板，移除 MCP 相关的说明。
- `docs/` 和 `docs-site/`：
    - 删除专门的 MCP 章节（如有）。
    - 在快速起步和 AI 集成指南中，将 MCP 替换为 CLI + Skills 的方式。
- `docs/i18n/README.md`：移除术语表中对 MCP 的引用。

### 3. 集成方式的转变

- **核心指令**：AI 助手现在应该始终通过运行 `harnspec` CLI 命令（如 `harnspec board`, `harnspec list`, `harnspec create`）来获取信息和执行操作。
- **Skills 强化**：通过 `.agents/skills/harnspec` 给 AI 提供方法论指导，AI 自行决定调用哪些 CLI 命令。

## 实施计划

### 阶段 1：清理源码 (清理)

- [x] 删除 `rust/harnspec-mcp/` 目录。
- [x] 删除 `packages/mcp/` 目录。
- [x] 修改 `rust/harnspec-cli/src/commands/mod.rs`（或相关入口），移除 `mcp` 命令。
- [x] 删除 `rust/harnspec-cli/src/commands/mcp.rs` 和 `rust/harnspec-cli/src/commands/init/mcp_config.rs`。

### 阶段 2：清理构建与分发配置

- [x] 从 `rust/Cargo.toml` 的 workspace 中移除 `harnspec-mcp`。
- [x] 从 `pnpm-workspace.yaml` 中移除 `packages/mcp`。
- [ ] 更新 `scripts/` 下的所有发布和构建脚本（如 `publish-platform-packages.ts`, `copy-rust-binaries.mjs` 等），移除对 MCP 的处理逻辑。 (部分完成，仍需清理 `prepare-publish.ts`, `restore-packages.ts`, `sync-rust-versions.ts`)

### 阶段 3：文档与模板更新

- [x] 更新根目录 `README.md`，聚焦于 CLI + Skills 集成。
- [ ] 批量替换 `packages/cli/templates` 下所有 `AGENTS.md` 的内容。
- [ ] 检查并更新 `docs/` 目录下的所有 markdown 文件。 (部分完成，`docs-site` 仍有大量残留)

### 阶段 4：验证

- [ ] 确保 `pnpm build` 和 `pnpm pre-release` 不再包含 MCP 相关步骤且正常执行。
- [ ] 验证 `harnspec init` 不再提示 MCP 配置。
- [ ] 验证 `harnspec --help` 中不再出现 `mcp` 命令。

## 验收项

- [ ] `rust/harnspec-mcp` 目录已彻底删除。
- [ ] `packages/mcp` 目录已彻底删除。
- [ ] 项目中不再包含任何名为 `mcp` 的 CLI 命令。
- [ ] 根目录 `README.md` 中不再提及 MCP 配置。
- [ ] `AGENTS.md` 模板已清理完毕。
- [ ] 所有 CI/CD 流程运行正常，不再包含 MCP 相关任务。
