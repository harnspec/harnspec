# AI Agent Instructions

## Project: HarnSpec

Lightweight spec methodology for AI-powered development.

## Skills

This project uses the Agent Skills framework for domain-specific guidance. **Read the appropriate skill when working on related tasks.**

### Core Skills

1. **harnspec** - Spec-Driven Development methodology
   - Install: `npx skills add harnspec/skills@harnspec`
   - Source: [harnspec/skills](https://github.com/harnspec/skills)
   - Use when: Working with specs, planning features, multi-step changes
   - Key: Run `board` or `search` before creating specs

2. **harnspec-development** - Development, commands, publishing, CI/CD, and runner research
   - Location: [.agents/skills/harnspec-development/SKILL.md](.agents/skills/harnspec-development/SKILL.md)
   - Use when: Contributing code, running tests, publishing, CI/CD, or looking up commands
   - Key: Always use `pnpm`, follow DRY principle

3. **agent-browser** - Browser automation for testing web apps
   - Location: [.agents/skills/agent-browser/SKILL.md](.agents/skills/agent-browser/SKILL.md)
   - Use when: Testing web UIs, interacting with websites, filling forms, taking screenshots
   - Key: Use `agent-browser` CLI instead of Playwright MCP for browser automation

## Project-Specific Rules
0. "Chinese preferred" - 在对话交流的过程中，优先使用中文

1. **Use pnpm** - Never npm or yarn. All package management uses pnpm.
2. **DRY Principle** - Extract shared logic; avoid duplication.
3. **Skills First** - Read the relevant skill file before starting work on development, specs, or publishing tasks.
4. **Context Economy** - Keep specs under 2000 tokens. Split large tasks.
5. **Progressive Disclosure** - Use skills and references for detailed guidance.
