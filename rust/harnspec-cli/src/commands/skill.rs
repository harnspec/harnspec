use std::error::Error;
use std::process::Command;

/*
/// Maps runner IDs to skills.sh agent names
pub fn runner_to_skills_agent(runner_id: &str) -> Option<&'static str> {
    match runner_id {
        "claude" => Some("claude-code"),
        "copilot" => Some("github-copilot"),
        "cursor" => Some("cursor"),
        "gemini" => Some("gemini-cli"),
        "codex" => Some("codex"),
        "cline" => Some("cline"),
        "continue" => Some("continue"),
        "windsurf" => Some("windsurf"),
        "aider" => Some("aider"),
        "opencode" => Some("opencode"),
        _ => None,
    }
}
*/

/// Install skills, optionally limited to specific agents.
/// If agents is None or empty, installs to all agents (fallback).
/// If skip_confirm is true, passes -y to skip interactive prompts.
pub fn install(_agents: Option<&[String]>, _skip_confirm: bool) -> Result<(), Box<dyn Error>> {
    let args = vec![
        "@harnspec/skills@latest",
        "-y", // Skip npx prompt to install
    ];

    run_npx(&args)
}

fn run_npx(args: &[&str]) -> Result<(), Box<dyn Error>> {
    match Command::new("npx").args(args).status() {
        Ok(status) if status.success() => Ok(()),
        Ok(status) => Err(format!("npx {} exited with {status}", args.join(" ")).into()),
        Err(err) => {
            // npm v10 removed npx; fallback to npm exec with the same args
            if err.kind() == std::io::ErrorKind::NotFound {
                let mut npm_args = vec!["exec".to_string(), "--yes".to_string()];
                // `npx foo --bar` becomes `npm exec --yes -- foo --bar`
                npm_args.push("--".to_string());
                npm_args.extend(args.iter().map(|s| s.to_string()));

                let status = Command::new("npm")
                    .args(&npm_args)
                    .status()
                    .map_err(|err| {
                        format!("Failed to run npm exec (is Node.js installed?): {err}")
                    })?;

                if status.success() {
                    return Ok(());
                }
                return Err(format!("npm exec {} exited with {status}", args.join(" ")).into());
            }
            Err(format!("Failed to run npx (is Node.js installed?): {err}").into())
        }
    }
}
