use colored::Colorize;
use std::error::Error;
use std::fs;

/// Embedded HarnSpec methodology skills
const HARNSPEC_SKILL_MD: &str = include_str!("../../templates/skills/harnspec/SKILL.md");

/// Collection of reference files associated with the skill
const REF_BEST_PRACTICES: &str =
    include_str!("../../templates/skills/harnspec/references/best-practices.md");
const REF_COMMANDS: &str = include_str!("../../templates/skills/harnspec/references/commands.md");
const REF_EXAMPLES: &str = include_str!("../../templates/skills/harnspec/references/examples.md");
const REF_WORKFLOW: &str = include_str!("../../templates/skills/harnspec/references/workflow.md");

/// Install skills, optionally limited to specific agents.
/// If agents is None or empty, installs to all agents (fallback).
/// If skip_confirm is true, passes -y to skip interactive prompts.
pub fn install(_agents: Option<&[String]>, _skip_confirm: bool) -> Result<(), Box<dyn Error>> {
    let root = std::env::current_dir()?;
    let skill_root = root.join(".agents").join("skills").join("harnspec");
    let ref_dir = skill_root.join("references");

    // 1. Ensure directories exist
    if !skill_root.exists() {
        fs::create_dir_all(&skill_root)?;
    }
    if !ref_dir.exists() {
        fs::create_dir_all(&ref_dir)?;
    }

    // 2. Write main SKILL.md
    fs::write(skill_root.join("SKILL.md"), HARNSPEC_SKILL_MD)?;

    // 3. Write reference documents
    fs::write(ref_dir.join("best-practices.md"), REF_BEST_PRACTICES)?;
    fs::write(ref_dir.join("commands.md"), REF_COMMANDS)?;
    fs::write(ref_dir.join("examples.md"), REF_EXAMPLES)?;
    fs::write(ref_dir.join("workflow.md"), REF_WORKFLOW)?;

    println!(
        "{} Officially injected HarnSpec SDD methodology skills (with references).",
        "✓".green()
    );
    println!("{} Skills location: {}", "•".cyan(), skill_root.display());

    Ok(())
}
