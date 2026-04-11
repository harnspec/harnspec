# Fix AI Session Startup: Program Not Found on Windows

## Problem
The `harnspec session start` command fails with "program not found" on Windows when trying to spawn runners like `gemini` that are installed as node scripts (e.g., `gemini.cmd` or `gemini.ps1`). This happens because `std::process::Command` in Rust does not always find these scripts even if they are in the PATH, especially if multiple versions (no extension, `.cmd`, `.ps1`) exist and have different priorities in the shell versus the OS.

## Solution
1. Use the `which` crate to resolve the exact absolute path of the runner command before spawning.
2. If the resolved path points to a `.ps1` file on Windows, execute it via `pwsh -File` to ensure it runs correctly regardless of `PATHEXT` or execution policy settings for direct invocation.
3. Update the `gemini` runner definition to include the `--yolo` flag, as recommended by the user for autonomous mode.

## Implementation Details

### `rust/harnspec-core/src/sessions/runner.rs`
- Modify `RunnerDefinition::build_command` to resolve the command path.
- Handle `.ps1` files on Windows.
- Update `gemini` builtin runner definition.

### `rust/harnspec-core/src/sessions/manager/lifecycle.rs`
- Ensure the resolved path is used correctly.
