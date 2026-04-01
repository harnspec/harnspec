---
status: planned
created: 2026-04-01
priority: high
tags:
- automation
- testing
- validation
- demo
- cli
- sdd
parent: 3-universal-skills-initiative
---

# 5-automated-demo-validation

## 概述 (Overview)

### 问题 (Problem)
随着 HarnSpec 项目功能的不断丰富，核心指令（如 `init`, `skills install`, `spec` 管理等）在不同环境下的一致性和稳定性变得至关重要。手动测试这些核心流程不仅耗时且容易遗漏回归问题。此外，为了确保 SDD（Spec-Driven Development）方法论的有效执行，需要一套自动化的验证方案来模拟真实的用户开发流程。

### Goals (目标)
1. **自动化验证流程**：在根目录下生成 demo 项目，通过脚本自动化执行 HarnSpec 核心指令。
2. **核心指令覆盖**：
    - **必须测试**：harnspec 的打包 (package)、安装 (install) 以及 skills 注入流程。
    - **方法论验证**：sdd 驱动的有效性、指令遵循度。
3. **功能模块验证**：
    - Spec 管理：创建 (create)、更新 (update)、删除 (delete)、拆分 (split)、父子关系 (parent-child)。
    - UI/TUI 验证：`harnspec tui` 和 `harnspec ui` 的连通性和基础功能。

## 设计 (Design)

### 1. 测试环境 (Test Environment)
- 在项目根目录动态创建一个 `harnspec-demo` 文件夹作为隔离的测试空间。
- 使用本地编译的二进制文件进行测试，以确保最新的代码变更得到验证。

### 2. 核心指令测试 (Mandatory Tests)
- **打包与安装**：模拟执行打包流程，并在 demo 项目中进行“本地安装”验证。
- **Skills 注入**：验证 `harnspec init` 能否正确拉取并注入官方方法论 skills。

### 3. SDD 与 Spec 管理测试 (Secondary Tests)
- **Spec 完整生命周期**：
    - 自动化脚本依次执行 `create`, `update` (状态/元数据), `rel add` (建立父子/依赖关系), `split` (验证大 spec 拆分逻辑)。
- **指令遵循度验证**：通过模拟 AI Agent 的输入，验证其是否能根据注入的 skills 指令格式正确操作。

### 4. 界面与交互验证 (UI/TUI)
- **TUI**：启动 `harnspec tui` 并验证其基础信息（如版本号）是否正确渲染。
- **UI**：启动 `harnspec ui` 服务，并进行基础的 HTTP 健康检查（/api/health 等）。

## 实施计划 (Plan)

### 第一阶段：测试基座建设
- [ ] 创建自动化测试脚本 `scripts/validate-demo.mjs`。
- [ ] 实现 demo 项目的初始化与清理逻辑。

### 第二阶段：核心指令自动化
- [ ] 编写打包逻辑验证测试。
- [ ] 编写 `harnspec init` 与 `skills install` 的自动化断言。

### 第三阶段：Spec 管理深度测试
- [ ] 编写涵盖创建、更新、关联、拆分的完整 spec 链条测试。
- [ ] 记录并验证指令执行的日志。

### 第四阶段：界面冒烟测试
- [ ] 实现针对 TUI 和 UI 服务的快速启动验证。

## 验收标准 (Acceptance Criteria)
- [ ] 运行自动化测试脚本后，所有指令均能产生预期的文件变更或标准输出。
- [ ] Demo 项目在测试完成后能被正确清理。
- [ ] 测试报告清晰展示每项指令的通过情况。

## 备注 (Notes)
- 只需要新建 spec 而不在此任务中执行。
- 自动化测试应尽量保持轻量，不引入沉重的外部依赖。
