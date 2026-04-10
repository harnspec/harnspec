---
status: completed
completed: 2026-04-10
created: 2026-04-06
priority: high
tags:
- bug
- session
- runner
created_at: 2026-04-06T12:52:39.529536100Z
updated_at: 2026-04-06T12:52:39.529536100Z
---
# Fix AI Session and Runner Startup Issues

## Overview
Fix issues where AI sessions and runners fail to start correctly. Specifically, the \project_path\ in session creation/running should default to the current working directory, and AI runners (like Claude) should have corrected command flags.

## Requirements
- [x] Update \SessionSubcommand::Create\ and \SessionSubcommand::Run\ to make \project_path\ optional (\Option<String>\).
- [x] In \main.rs\, default \project_path\ to the current working directory if not provided in CLI.
- [x] Ensure \project_path\ is converted to an absolute path before being used/stored (use \std::fs::canonicalize\ or \dunce::canonicalize\).
- [x] Fix \claude\ runner's \prompt_flag\ (should be \-p\, not \--print\).
- [x] Verify \copilot\ runner's \prompt_flag\ (currently \--prompt\, should verify if it's correct for the intended CLI tool).
- [x] Ensure \gemini\ runner's \prompt_flag\ is correct (currently \None\ for positional).
- [x] Ensure AI sessions correctly set the working directory to the project path in \SessionManager::start_session\.
- [x] Add a regression test to verify that a session can be created and started without an explicit project path.

## Non-Goals
- Adding new AI runners.
- Major refactoring of the session management system.

## Acceptance Criteria
- \harnspec session create\ works without \--project-path\ flag.
- AI sessions start successfully and run in the correct directory.
- Claude runner executes with the correct prompt flag.
- \harnspec session run\ also defaults to the current directory.