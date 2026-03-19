//! GitHub integration CLI commands

use colored::Colorize;
use leanspec_core::github::{GitHubClient, RepoRef};
use std::error::Error;

pub enum GitHubCommand {
    /// Detect specs in a GitHub repository
    Detect {
        repo: String,
        branch: Option<String>,
        token: Option<String>,
    },
    /// Import a GitHub repo as a LeanSpec project
    Import {
        repo: String,
        branch: Option<String>,
        name: Option<String>,
        token: Option<String>,
    },
    /// List user's GitHub repos
    Repos { token: Option<String> },
}

pub fn run(cmd: GitHubCommand, output_format: &str) -> Result<(), Box<dyn Error>> {
    match cmd {
        GitHubCommand::Detect {
            repo,
            branch,
            token,
        } => detect(&repo, branch.as_deref(), token.as_deref(), output_format),
        GitHubCommand::Import {
            repo,
            branch,
            name,
            token,
        } => import(
            &repo,
            branch.as_deref(),
            name.as_deref(),
            token.as_deref(),
            output_format,
        ),
        GitHubCommand::Repos { token } => list_repos(token.as_deref(), output_format),
    }
}

fn detect(
    repo: &str,
    branch: Option<&str>,
    token: Option<&str>,
    output_format: &str,
) -> Result<(), Box<dyn Error>> {
    let repo_ref =
        RepoRef::parse(repo).ok_or_else(|| format!("Invalid repo: '{}'. Use owner/repo", repo))?;

    let client = make_client(token);
    let result = client.detect_specs(&repo_ref, branch)?;

    if output_format == "json" {
        println!("{}", serde_json::to_string_pretty(&result)?);
        return Ok(());
    }

    match result {
        Some(detection) => {
            println!(
                "{} Found {} specs in {}/{} (branch: {})",
                "✓".green().bold(),
                detection.spec_count,
                repo_ref.owner,
                repo_ref.repo,
                detection.branch
            );
            println!("  Specs directory: {}", detection.specs_dir.bold());
            println!();

            for spec in &detection.specs {
                let status = spec.status.as_deref().unwrap_or("unknown");
                let title = spec.title.as_deref().unwrap_or("(no title)");
                println!("  {} {} [{}]", spec.path.bold(), title, status.dimmed());
            }

            if detection.spec_count > detection.specs.len() {
                println!(
                    "  ... and {} more",
                    detection.spec_count - detection.specs.len()
                );
            }

            println!();
            println!(
                "To import: {}",
                format!("lean-spec github import {}", repo_ref.full_name()).cyan()
            );
        }
        None => {
            println!(
                "{} No specs found in {}/{}",
                "✗".red().bold(),
                repo_ref.owner,
                repo_ref.repo
            );
            println!("  Looked for: specs/, .lean-spec/specs/, doc/specs/, docs/specs/");
        }
    }

    Ok(())
}

fn import(
    repo: &str,
    branch: Option<&str>,
    name: Option<&str>,
    token: Option<&str>,
    output_format: &str,
) -> Result<(), Box<dyn Error>> {
    let repo_ref =
        RepoRef::parse(repo).ok_or_else(|| format!("Invalid repo: '{}'. Use owner/repo", repo))?;

    let client = make_client(token);

    // Detect specs
    let detection = client
        .detect_specs(&repo_ref, branch)?
        .ok_or_else(|| format!("No specs found in '{}'", repo))?;

    // Register in project registry
    let mut registry = leanspec_core::storage::project_registry::ProjectRegistry::new()?;
    let project = registry.add_github(
        &repo_ref.full_name(),
        &detection.branch,
        &detection.specs_dir,
        name,
    )?;

    // Sync specs into cache
    let items = client.list_contents(&repo_ref, &detection.specs_dir, Some(&detection.branch))?;
    let mut synced = 0;

    for item in &items {
        if item.item_type != "dir" {
            continue;
        }
        if !item.name.chars().next().is_some_and(|c| c.is_ascii_digit()) {
            continue;
        }

        let local_dir = project.specs_dir.join(&item.name);
        std::fs::create_dir_all(&local_dir)?;

        let readme_path = format!("{}/{}/README.md", detection.specs_dir, item.name);
        if let Ok(content) =
            client.get_file_content(&repo_ref, &readme_path, Some(&detection.branch))
        {
            std::fs::write(local_dir.join("README.md"), &content)?;
            synced += 1;
        }
    }

    if output_format == "json" {
        let result = serde_json::json!({
            "projectId": project.id,
            "projectName": project.name,
            "repo": repo_ref.full_name(),
            "branch": detection.branch,
            "specsPath": detection.specs_dir,
            "syncedSpecs": synced,
        });
        println!("{}", serde_json::to_string_pretty(&result)?);
        return Ok(());
    }

    println!(
        "{} Imported {} as project '{}'",
        "✓".green().bold(),
        repo_ref.full_name().bold(),
        project.name
    );
    println!("  Branch: {}", detection.branch);
    println!("  Specs dir: {}", detection.specs_dir);
    println!("  Synced: {} specs", synced);
    println!(
        "  Cache: {}",
        project.specs_dir.display().to_string().dimmed()
    );

    Ok(())
}

fn list_repos(token: Option<&str>, output_format: &str) -> Result<(), Box<dyn Error>> {
    let client = make_client(token);
    let repos = client.list_user_repos(30)?;

    if output_format == "json" {
        println!("{}", serde_json::to_string_pretty(&repos)?);
        return Ok(());
    }

    if repos.is_empty() {
        println!("No repositories found. Is GITHUB_TOKEN set?");
        return Ok(());
    }

    println!("{} repositories:\n", repos.len());
    for repo in &repos {
        let visibility = if repo.private { "private" } else { "public" };
        let desc = repo
            .description
            .as_deref()
            .unwrap_or("")
            .chars()
            .take(60)
            .collect::<String>();

        println!(
            "  {} [{}] {}",
            repo.full_name.bold(),
            visibility.dimmed(),
            desc.dimmed()
        );
    }

    Ok(())
}

fn make_client(token: Option<&str>) -> GitHubClient {
    match token {
        Some(t) => GitHubClient::with_token(t),
        None => GitHubClient::new(),
    }
}
