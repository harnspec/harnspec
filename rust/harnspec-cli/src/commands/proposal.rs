//! Proposal command implementation
//!
//! Interactive workflow that guides users from a vague idea or high-level goal
//! to a structured hierarchy of specs (parent umbrella + child feature specs).

use colored::Colorize;
use dialoguer::{Confirm, Editor, Input, Select};
use std::error::Error;
use std::fs;
use std::path::Path;

use crate::commands::create;

/// Parameters for the proposal command, parsed from CLI args.
pub struct ProposalParams {
    pub specs_dir: String,
    /// Optional initial idea/goal text passed as positional argument.
    pub idea: Option<String>,
    /// Read proposal content from a file.
    pub file: Option<String>,
    /// Non-interactive mode (for AI agents).
    pub non_interactive: bool,
    /// Priority for generated specs.
    pub priority: String,
    /// Tags for generated specs (comma-separated).
    pub tags: Option<String>,
}

// ── Data structures for the proposal workflow ────────────────────────

/// A single feature decomposed from the user's idea.
#[derive(Debug, Clone)]
struct Feature {
    /// Kebab-case name for the spec (e.g., "websocket-infrastructure").
    name: String,
    /// Human-readable title.
    title: String,
    /// One-paragraph description.
    description: String,
    /// Names of other features this one depends on.
    depends_on: Vec<String>,
}

/// The fully-confirmed proposal ready for spec generation.
#[derive(Debug)]
struct ConfirmedProposal {
    /// Original raw idea text.
    raw_idea: String,
    /// Refined scope after clarification.
    refined_scope: String,
    /// Umbrella spec title.
    umbrella_title: String,
    /// Kebab-case name for the umbrella spec.
    umbrella_name: String,
    /// Decomposed features.
    features: Vec<Feature>,
    /// Priority level (low/medium/high/critical).
    priority: String,
    /// Tags for all generated specs.
    tags: Vec<String>,
}

/// Result of a successful proposal generation.
struct GenerationResult {
    parent_spec_name: String,
    child_spec_names: Vec<String>,
}

// ── Entry point ──────────────────────────────────────────────────────

pub fn run(params: ProposalParams) -> Result<(), Box<dyn Error>> {
    println!("\n{}", "🚀 HarnSpec Proposal Mode".cyan().bold());
    println!(
        "{}",
        "Transform a vague idea into structured, actionable specs.".dimmed()
    );
    println!();

    // Phase 1: Propose — gather the initial idea
    let raw_idea = phase_propose(&params)?;

    // Phase 2: Clarify — ask targeted questions
    let refined_scope = if params.non_interactive {
        // In non-interactive mode, use the raw idea as the refined scope
        raw_idea.clone()
    } else {
        phase_clarify(&raw_idea)?
    };

    // Phase 3: Design — decompose into features
    let mut features = phase_design(&refined_scope, &params)?;

    // Phase 4: Confirm — user reviews and edits the plan
    let confirmed = if params.non_interactive {
        // Auto-confirm in non-interactive mode
        let umbrella_name = to_kebab_case(&extract_title(&refined_scope));
        let umbrella_title = extract_title(&refined_scope);
        ConfirmedProposal {
            raw_idea: raw_idea.clone(),
            refined_scope: refined_scope.clone(),
            umbrella_title,
            umbrella_name,
            features: features.clone(),
            priority: params.priority.clone(),
            tags: parse_tags(&params.tags),
        }
    } else {
        phase_confirm(&raw_idea, &refined_scope, &mut features, &params)?
    };

    // Phase 5: Generate — create parent + child specs
    let result = phase_generate(&confirmed, &params)?;

    // Phase 6: Panorama — display spec landscape
    phase_panorama(&confirmed, &result);

    Ok(())
}

// ── Phase 1: Propose ─────────────────────────────────────────────────

fn phase_propose(params: &ProposalParams) -> Result<String, Box<dyn Error>> {
    println!("{} {}", "Phase 1/6".bold().cyan(), "Propose".bold());
    println!(
        "{}",
        "Describe your idea or goal in natural language.".dimmed()
    );
    println!();

    let idea = if let Some(ref file_path) = params.file {
        // Read from file
        let path = Path::new(file_path);
        if !path.exists() {
            return Err(format!("File not found: {}", file_path).into());
        }
        let content = fs::read_to_string(path)?;
        println!("{} Read proposal from: {}", "✓".green(), file_path.cyan());
        content
    } else if let Some(ref idea_text) = params.idea {
        // Use provided idea text
        println!(
            "{} Using provided idea: {}",
            "✓".green(),
            truncate(idea_text, 80).dimmed()
        );
        idea_text.clone()
    } else if params.non_interactive {
        return Err("Non-interactive mode requires --file or an idea argument.".into());
    } else {
        // Interactive: ask user for idea
        let input: String = Input::new()
            .with_prompt("💡 What's your idea or goal?")
            .interact_text()?;

        if input.trim().is_empty() {
            return Err("Idea cannot be empty.".into());
        }

        // Optionally let user elaborate in an editor
        let elaborate = Confirm::new()
            .with_prompt("Would you like to elaborate in an editor?")
            .default(false)
            .interact()?;

        if elaborate {
            if let Some(edited) = Editor::new().edit(&input)? {
                edited
            } else {
                input
            }
        } else {
            input
        }
    };

    println!();
    Ok(idea)
}

// ── Phase 2: Clarify ─────────────────────────────────────────────────

fn phase_clarify(raw_idea: &str) -> Result<String, Box<dyn Error>> {
    println!("{} {}", "Phase 2/6".bold().cyan(), "Clarify".bold());
    println!(
        "{}",
        "Answer a few questions to refine scope and intent.".dimmed()
    );
    println!();

    // Target audience / users
    let target: String = Input::new()
        .with_prompt("🎯 Who is the target user/audience?")
        .default("developers".to_string())
        .interact_text()?;

    // Scope boundaries
    let scope: String = Input::new()
        .with_prompt("📐 What's in scope? (key features)")
        .interact_text()?;

    // Non-goals
    let non_goals: String = Input::new()
        .with_prompt("🚫 What's explicitly out of scope?")
        .default("N/A".to_string())
        .interact_text()?;

    // Constraints
    let constraints: String = Input::new()
        .with_prompt("⚙️  Any constraints? (tech stack, performance, etc.)")
        .default("None".to_string())
        .interact_text()?;

    // Success criteria
    let success: String = Input::new()
        .with_prompt("✅ How will you know it's done? (success criteria)")
        .interact_text()?;

    // Build refined scope
    let refined = format!(
        "## Original Idea\n\n{}\n\n\
         ## Target Audience\n\n{}\n\n\
         ## Scope\n\n{}\n\n\
         ## Non-Goals\n\n{}\n\n\
         ## Constraints\n\n{}\n\n\
         ## Success Criteria\n\n{}",
        raw_idea, target, scope, non_goals, constraints, success
    );

    println!();
    println!("{} Scope refined successfully.", "✓".green());
    println!();

    Ok(refined)
}

// ── Phase 3: Design ──────────────────────────────────────────────────

fn phase_design(
    _refined_scope: &str,
    params: &ProposalParams,
) -> Result<Vec<Feature>, Box<dyn Error>> {
    println!("{} {}", "Phase 3/6".bold().cyan(), "Design".bold());
    println!(
        "{}",
        "Decompose your idea into implementable features.".dimmed()
    );
    println!();

    if params.non_interactive {
        // In non-interactive mode, generate a single feature from the idea
        let features = vec![Feature {
            name: "core-implementation".to_string(),
            title: "Core Implementation".to_string(),
            description: "Primary implementation of the proposed feature.".to_string(),
            depends_on: vec![],
        }];
        println!(
            "{} Generated {} feature(s) from proposal.",
            "✓".green(),
            features.len()
        );
        return Ok(features);
    }

    let mut features: Vec<Feature> = Vec::new();

    println!(
        "{}",
        "Add features one by one. Enter an empty name to finish.".dimmed()
    );
    println!();

    loop {
        let ordinal = features.len() + 1;
        let name_prompt = format!("Feature #{} name (kebab-case, empty to finish)", ordinal);
        let name: String = Input::new()
            .with_prompt(&name_prompt)
            .allow_empty(true)
            .interact_text()?;

        let name = name.trim().to_string();
        if name.is_empty() {
            if features.is_empty() {
                println!("{}", "⚠ At least one feature is required.".yellow());
                continue;
            }
            break;
        }

        let kebab_name = to_kebab_case(&name);

        let title: String = Input::new()
            .with_prompt("  Title")
            .default(to_title_case(&kebab_name))
            .interact_text()?;

        let description: String = Input::new().with_prompt("  Description").interact_text()?;

        // Ask about dependencies on previously defined features
        let depends_on = if !features.is_empty() {
            let dep_options: Vec<String> = features.iter().map(|f| f.name.clone()).collect();
            let dep_prompt = format!(
                "  Depends on? (comma-separated from: {})",
                dep_options.join(", ")
            );
            let deps_input: String = Input::new()
                .with_prompt(&dep_prompt)
                .default("none".to_string())
                .interact_text()?;

            if deps_input.trim().to_lowercase() == "none" {
                vec![]
            } else {
                deps_input
                    .split(',')
                    .map(|s| s.trim().to_string())
                    .filter(|s| dep_options.contains(s))
                    .collect()
            }
        } else {
            vec![]
        };

        println!("  {} Added feature: {}", "✓".green(), kebab_name.cyan());
        println!();

        features.push(Feature {
            name: kebab_name,
            title,
            description,
            depends_on,
        });
    }

    println!();
    println!("{} Designed {} feature(s).", "✓".green(), features.len());
    println!();

    Ok(features)
}

// ── Phase 4: Confirm ─────────────────────────────────────────────────

fn phase_confirm(
    raw_idea: &str,
    refined_scope: &str,
    features: &mut Vec<Feature>,
    params: &ProposalParams,
) -> Result<ConfirmedProposal, Box<dyn Error>> {
    println!("{} {}", "Phase 4/6".bold().cyan(), "Confirm".bold());
    println!(
        "{}",
        "Review the proposal before generating specs.".dimmed()
    );
    println!();

    // Determine umbrella title
    let umbrella_title: String = Input::new()
        .with_prompt("📋 Umbrella spec title")
        .interact_text()?;

    let umbrella_name = to_kebab_case(&umbrella_title);

    // Display panorama preview
    display_panorama_preview(&umbrella_title, &umbrella_name, features);

    // Confirm or edit
    loop {
        let options = vec![
            "✅ Confirm and generate specs",
            "✏️  Edit a feature",
            "➕ Add a feature",
            "➖ Remove a feature",
            "🔄 Change umbrella title",
            "❌ Cancel",
        ];

        let selection = Select::new()
            .with_prompt("What would you like to do?")
            .items(&options)
            .default(0)
            .interact()?;

        match selection {
            0 => break, // Confirm
            1 => {
                // Edit a feature
                if features.is_empty() {
                    println!("{}", "No features to edit.".yellow());
                    continue;
                }
                let feature_names: Vec<String> = features.iter().map(|f| f.name.clone()).collect();
                let idx = Select::new()
                    .with_prompt("Select feature to edit")
                    .items(&feature_names)
                    .interact()?;

                let new_title: String = Input::new()
                    .with_prompt("  New title")
                    .default(features[idx].title.clone())
                    .interact_text()?;
                let new_desc: String = Input::new()
                    .with_prompt("  New description")
                    .default(features[idx].description.clone())
                    .interact_text()?;

                features[idx].title = new_title;
                features[idx].description = new_desc;

                println!("{} Feature updated.", "✓".green());
                display_panorama_preview(&umbrella_title, &umbrella_name, features);
            }
            2 => {
                // Add a feature
                let name: String = Input::new()
                    .with_prompt("Feature name (kebab-case)")
                    .interact_text()?;
                let kebab_name = to_kebab_case(&name);
                let title: String = Input::new()
                    .with_prompt("  Title")
                    .default(to_title_case(&kebab_name))
                    .interact_text()?;
                let desc: String = Input::new().with_prompt("  Description").interact_text()?;

                features.push(Feature {
                    name: kebab_name,
                    title,
                    description: desc,
                    depends_on: vec![],
                });

                println!("{} Feature added.", "✓".green());
                display_panorama_preview(&umbrella_title, &umbrella_name, features);
            }
            3 => {
                // Remove a feature
                if features.is_empty() {
                    println!("{}", "No features to remove.".yellow());
                    continue;
                }
                let feature_names: Vec<String> = features.iter().map(|f| f.name.clone()).collect();
                let idx = Select::new()
                    .with_prompt("Select feature to remove")
                    .items(&feature_names)
                    .interact()?;

                let removed = features.remove(idx);
                println!("{} Removed feature: {}", "✓".green(), removed.name.cyan());
                display_panorama_preview(&umbrella_title, &umbrella_name, features);
            }
            4 => {
                // Change umbrella title — we re-enter the loop but can't
                // mutate the outer binding easily, so just print a note.
                println!(
                    "{}",
                    "Umbrella title will be set during generation.".dimmed()
                );
            }
            5 => {
                // Cancel
                return Err("Proposal cancelled by user.".into());
            }
            _ => unreachable!(),
        }
    }

    let tags = parse_tags(&params.tags);

    Ok(ConfirmedProposal {
        raw_idea: raw_idea.to_string(),
        refined_scope: refined_scope.to_string(),
        umbrella_title,
        umbrella_name,
        features: features.clone(),
        priority: params.priority.clone(),
        tags,
    })
}

// ── Phase 5: Generate ────────────────────────────────────────────────

fn phase_generate(
    proposal: &ConfirmedProposal,
    params: &ProposalParams,
) -> Result<GenerationResult, Box<dyn Error>> {
    println!();
    println!("{} {}", "Phase 5/6".bold().cyan(), "Generate".bold());
    println!("{}", "Creating parent and child specs...".dimmed());
    println!();

    // 1. Build umbrella spec content
    let child_table = proposal
        .features
        .iter()
        .map(|f| {
            let deps = if f.depends_on.is_empty() {
                String::new()
            } else {
                format!(" (depends on: {})", f.depends_on.join(", "))
            };
            format!("- **{}** — {}{}", f.title, f.description, deps)
        })
        .collect::<Vec<_>>()
        .join("\n");

    let description_text = format!(
        "Umbrella spec for the **{}** initiative. This groups all related feature specs.",
        proposal.umbrella_title
    );

    let umbrella_content = format!(
        "# {title}\n\n\
         ## Overview\n\n\
         {description}\n\n\
         ## Original Intent\n\n\
         {raw_idea}\n\n\
         ## Refined Scope\n\n\
         {refined_scope}\n\n\
         ## Child Features\n\n\
         {children}\n\n\
         ## Acceptance Criteria\n\n\
         - [ ] All child specs are complete\n\
         - [ ] Integration between features works correctly\n\
         - [ ] End-to-end user workflow is validated\n",
        title = proposal.umbrella_title,
        description = description_text,
        raw_idea = proposal.raw_idea,
        refined_scope = proposal.refined_scope,
        children = child_table,
    );

    let umbrella_tags = {
        let mut t = proposal.tags.clone();
        if !t.contains(&"umbrella".to_string()) {
            t.push("umbrella".to_string());
        }
        if !t.contains(&"proposal".to_string()) {
            t.push("proposal".to_string());
        }
        t
    };

    // Create umbrella spec
    create::run(create::CreateParams {
        specs_dir: params.specs_dir.clone(),
        name: proposal.umbrella_name.clone(),
        title: Some(proposal.umbrella_title.clone()),
        template: None,
        status: Some("planned".to_string()),
        priority: proposal.priority.clone(),
        tags: Some(umbrella_tags.join(",")),
        parent: None,
        depends_on: vec![],
        content: Some(umbrella_content),
        file: None,
        assignee: None,
        description: None,
    })?;

    // Determine the actual parent spec directory name by scanning the specs dir
    let parent_spec_name = find_latest_spec(&params.specs_dir, &proposal.umbrella_name)?;

    println!();

    // 2. Create child specs
    let mut child_spec_names = Vec::new();

    for feature in &proposal.features {
        let child_content = format!(
            "# {title}\n\n\
             ## Overview\n\n\
             {description}\n\n\
             ## Requirements\n\n\
             - [ ] Implement core functionality\n\
             - [ ] Add tests\n\
             - [ ] Update documentation\n\n\
             ## Non-Goals\n\n\
             - Out-of-scope items should be tracked in separate specs\n\n\
             ## Acceptance Criteria\n\n\
             - [ ] Feature works as described\n\
             - [ ] Tests pass\n",
            title = feature.title,
            description = feature.description,
        );

        let child_tags = {
            let mut t = proposal.tags.clone();
            if !t.contains(&"proposal".to_string()) {
                t.push("proposal".to_string());
            }
            t
        };

        // Resolve depends_on to actual spec directory names
        let depends_on_resolved: Vec<String> = feature
            .depends_on
            .iter()
            .filter_map(|dep_name| {
                child_spec_names
                    .iter()
                    .find(|(name, _): &&(String, String)| name == dep_name)
                    .map(|(_, spec_dir)| spec_dir.clone())
            })
            .collect();

        create::run(create::CreateParams {
            specs_dir: params.specs_dir.clone(),
            name: feature.name.clone(),
            title: Some(feature.title.clone()),
            template: None,
            status: Some("planned".to_string()),
            priority: proposal.priority.clone(),
            tags: Some(child_tags.join(",")),
            parent: Some(parent_spec_name.clone()),
            depends_on: depends_on_resolved,
            content: Some(child_content),
            file: None,
            assignee: None,
            description: None,
        })?;

        // Find the created child spec name
        let child_spec_name = find_latest_spec(&params.specs_dir, &feature.name)?;
        child_spec_names.push((feature.name.clone(), child_spec_name));

        println!();
    }

    let result_child_names: Vec<String> = child_spec_names.iter().map(|(_, s)| s.clone()).collect();

    Ok(GenerationResult {
        parent_spec_name,
        child_spec_names: result_child_names,
    })
}

// ── Phase 6: Panorama ────────────────────────────────────────────────

fn phase_panorama(proposal: &ConfirmedProposal, result: &GenerationResult) {
    println!();
    println!("{} {}", "Phase 6/6".bold().cyan(), "Panorama".bold());
    println!();

    let title = format!("📋 Proposal Panorama: {}", proposal.umbrella_title);
    let separator = "━".repeat(title.len().min(60));

    println!("{}", title.cyan().bold());
    println!("{}", separator.cyan());
    println!();

    // Parent
    println!(
        "🎯 Parent: {} {}",
        result.parent_spec_name.bold(),
        "(umbrella)".dimmed()
    );
    println!(
        "   Status: {} | Priority: {}",
        "planned".green(),
        proposal.priority.yellow()
    );
    println!();

    // Children
    println!("{}", "📦 Child Specs:".bold());

    let total_children = result.child_spec_names.len();
    for (i, (feature, spec_name)) in proposal
        .features
        .iter()
        .zip(result.child_spec_names.iter())
        .enumerate()
    {
        let is_last = i == total_children - 1;
        let prefix = if is_last {
            "   └──"
        } else {
            "   ├──"
        };

        let deps_info = if feature.depends_on.is_empty() {
            String::new()
        } else {
            format!(" depends_on: {}", feature.depends_on.join(", "))
        };

        println!(
            "{} {} {}{}",
            prefix,
            spec_name.cyan(),
            "[planned]".green(),
            deps_info.dimmed()
        );
    }

    println!();

    let total = 1 + total_children;
    println!(
        "📊 Total: 1 parent + {} children = {} specs created",
        total_children.to_string().bold(),
        total.to_string().bold()
    );

    println!();
    println!(
        "{}",
        "✨ Proposal complete! Use 'harnspec board' to see your specs."
            .green()
            .bold()
    );
    println!();
}

// ── Helpers ──────────────────────────────────────────────────────────

fn display_panorama_preview(umbrella_title: &str, umbrella_name: &str, features: &[Feature]) {
    println!();
    let title = format!("📋 Preview: {}", umbrella_title);
    println!("{}", title.cyan().bold());
    println!("{}", "─".repeat(title.len().min(50)).cyan());
    println!("   🎯 {} (umbrella)", umbrella_name.bold());

    for (i, f) in features.iter().enumerate() {
        let is_last = i == features.len() - 1;
        let prefix = if is_last {
            "   └──"
        } else {
            "   ├──"
        };
        let deps = if f.depends_on.is_empty() {
            String::new()
        } else {
            format!(" → depends_on: {}", f.depends_on.join(", "))
        };
        println!(
            "{} {} — {}{}",
            prefix,
            f.name.cyan(),
            f.description.dimmed(),
            deps.dimmed()
        );
    }

    println!();
}

/// Find the latest spec directory matching a given name pattern.
fn find_latest_spec(specs_dir: &str, name: &str) -> Result<String, Box<dyn Error>> {
    let specs_path = Path::new(specs_dir);
    if !specs_path.exists() {
        return Err("Specs directory not found.".into());
    }

    let mut matches: Vec<String> = Vec::new();
    for entry in fs::read_dir(specs_path)? {
        let entry = entry?;
        if entry.file_type()?.is_dir() {
            let dir_name = entry.file_name().to_string_lossy().to_string();
            // Match pattern: <number>-<name> or <number>-<prefix>-<name>
            if dir_name.contains(name) {
                matches.push(dir_name);
            }
        }
    }

    // Sort and return the last (highest-numbered) match
    matches.sort();
    matches
        .last()
        .cloned()
        .ok_or_else(|| format!("Could not find generated spec for: {}", name).into())
}

fn to_kebab_case(s: &str) -> String {
    s.to_lowercase()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join("-")
        .chars()
        .filter(|c| c.is_alphanumeric() || *c == '-')
        .collect()
}

fn to_title_case(s: &str) -> String {
    s.split('-')
        .map(|w| {
            let mut chars = w.chars();
            match chars.next() {
                Some(c) => c.to_uppercase().collect::<String>() + chars.as_str(),
                None => String::new(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

fn extract_title(text: &str) -> String {
    // Try to extract a meaningful title from the first line
    let first_line = text.lines().next().unwrap_or("Untitled Proposal");
    let cleaned = first_line
        .trim()
        .trim_start_matches('#')
        .trim_start_matches("## ")
        .trim();
    if cleaned.is_empty() {
        "Untitled Proposal".to_string()
    } else {
        cleaned.to_string()
    }
}

fn truncate(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        format!("{}...", &s[..max_len])
    }
}

fn parse_tags(tags: &Option<String>) -> Vec<String> {
    tags.as_ref()
        .map(|t| {
            t.split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect()
        })
        .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_kebab_case() {
        assert_eq!(to_kebab_case("Hello World"), "hello-world");
        assert_eq!(
            to_kebab_case("Real-Time Collaboration"),
            "real-time-collaboration"
        );
        assert_eq!(to_kebab_case("simple"), "simple");
        assert_eq!(to_kebab_case("  spaced  out  "), "spaced-out");
    }

    #[test]
    fn test_to_title_case() {
        assert_eq!(to_title_case("hello-world"), "Hello World");
        assert_eq!(to_title_case("real-time-collab"), "Real Time Collab");
        assert_eq!(to_title_case("simple"), "Simple");
    }

    #[test]
    fn test_extract_title() {
        assert_eq!(extract_title("# My Feature\n\nDescription"), "My Feature");
        assert_eq!(extract_title("## Sub Heading\nMore text"), "Sub Heading");
        assert_eq!(extract_title("Plain text title\nMore"), "Plain text title");
        assert_eq!(extract_title(""), "Untitled Proposal");
    }

    #[test]
    fn test_truncate() {
        assert_eq!(truncate("short", 10), "short");
        assert_eq!(truncate("a long string here", 10), "a long str...");
    }

    #[test]
    fn test_parse_tags() {
        assert_eq!(parse_tags(&Some("a,b,c".to_string())), vec!["a", "b", "c"]);
        assert_eq!(parse_tags(&Some("  x , y ".to_string())), vec!["x", "y"]);
        assert_eq!(parse_tags(&None), Vec::<String>::new());
    }
}
