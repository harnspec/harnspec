//! Shared helpers for spec handlers

#![allow(clippy::result_large_err)]

use axum::http::StatusCode;
use axum::Json;
use sha2::{Digest, Sha256};
use std::path::Path as FsPath;

use harnspec_core::{
    global_token_counter, HarnSpecConfig, SpecLoader, TokenStatus, ValidationResult,
};

use crate::error::ApiError;
use crate::project_registry::Project;
use crate::state::AppState;
use crate::utils::resolve_project;

use crate::types::SpecDetail;
use crate::types::SpecSummary;
use crate::types::SubSpec;

/// Helper to get the spec loader for a project (required project_id)
pub(super) async fn get_spec_loader(
    state: &AppState,
    project_id: &str,
) -> Result<(SpecLoader, Project), (StatusCode, Json<ApiError>)> {
    let project = resolve_project(state, project_id).await?;
    let specs_dir = project.specs_dir.clone();
    Ok((SpecLoader::new(&specs_dir), project))
}

pub(super) fn hash_raw_content(content: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(content.as_bytes());
    format!("{:x}", hasher.finalize())
}

pub(super) fn token_status_label(status: TokenStatus) -> &'static str {
    match status {
        TokenStatus::Optimal => "optimal",
        TokenStatus::Good => "good",
        TokenStatus::Warning => "warning",
        TokenStatus::Excessive => "critical",
    }
}

pub(super) fn validation_status_label(result: &ValidationResult) -> &'static str {
    if result.has_errors() {
        "fail"
    } else if result.has_warnings() {
        "warn"
    } else {
        "pass"
    }
}

pub(super) fn spec_number_from_name(name: &str) -> Option<u32> {
    name.split('-').next()?.parse().ok()
}

pub(super) fn summary_from_record(
    project_id: &str,
    record: &crate::sync_state::SpecRecord,
) -> SpecSummary {
    let counter = global_token_counter();
    let token_result = counter.count_spec(&record.content_md);
    let token_status_str = token_status_label(token_result.status);

    let validation_status_str = if record.content_md.trim().is_empty() {
        "warn"
    } else {
        "pass"
    };

    SpecSummary {
        project_id: Some(project_id.to_string()),
        id: record.spec_name.clone(),
        spec_number: spec_number_from_name(&record.spec_name),
        spec_name: record.spec_name.clone(),
        title: record.title.clone(),
        status: record.status.clone(),
        priority: record.priority.clone(),
        tags: record.tags.clone(),
        assignee: record.assignee.clone(),
        created_at: record.created_at,
        updated_at: record.updated_at,
        completed_at: record.completed_at,
        file_path: record
            .file_path
            .clone()
            .unwrap_or_else(|| record.spec_name.clone()),
        depends_on: record.depends_on.clone(),
        parent: record.parent.clone(),
        children: Vec::new(),
        required_by: Vec::new(),
        content_hash: Some(record.content_hash.clone()),
        token_count: Some(token_result.total),
        token_status: Some(token_status_str.to_string()),
        validation_status: Some(validation_status_str.to_string()),
        relationships: None,
    }
}

pub(super) fn detail_from_record(
    project_id: &str,
    record: &crate::sync_state::SpecRecord,
) -> SpecDetail {
    let counter = global_token_counter();
    let token_result = counter.count_spec(&record.content_md);
    let token_status_str = token_status_label(token_result.status);

    let validation_status_str = if record.content_md.trim().is_empty() {
        "warn"
    } else {
        "pass"
    };

    SpecDetail {
        project_id: Some(project_id.to_string()),
        id: record.spec_name.clone(),
        spec_number: spec_number_from_name(&record.spec_name),
        spec_name: record.spec_name.clone(),
        title: record.title.clone(),
        status: record.status.clone(),
        priority: record.priority.clone(),
        tags: record.tags.clone(),
        assignee: record.assignee.clone(),
        content_md: record.content_md.clone(),
        created_at: record.created_at,
        updated_at: record.updated_at,
        completed_at: record.completed_at,
        file_path: record
            .file_path
            .clone()
            .unwrap_or_else(|| record.spec_name.clone()),
        depends_on: record.depends_on.clone(),
        parent: record.parent.clone(),
        children: Vec::new(),
        required_by: Vec::new(),
        content_hash: Some(record.content_hash.clone()),
        token_count: Some(token_result.total),
        token_status: Some(token_status_str.to_string()),
        validation_status: Some(validation_status_str.to_string()),
        relationships: None,
        sub_specs: None,
    }
}

pub(super) fn strip_frontmatter(content: &str) -> String {
    let trimmed = content.trim_start();
    if !trimmed.starts_with("---") {
        return content.to_string();
    }

    let mut lines = trimmed.lines();
    lines.next();

    let mut in_frontmatter = true;
    let mut body = String::new();

    for line in lines {
        if in_frontmatter && line.trim() == "---" {
            in_frontmatter = false;
            continue;
        }

        if !in_frontmatter {
            body.push_str(line);
            body.push('\n');
        }
    }

    if in_frontmatter {
        return content.to_string();
    }

    body
}

pub(super) fn format_sub_spec_name(file_name: &str) -> String {
    let base = file_name.trim_end_matches(".md");
    base.split(['-', '_'])
        .filter(|part| !part.is_empty())
        .map(|part| {
            if part.len() <= 4 && part.chars().all(|c| c.is_ascii_uppercase()) {
                part.to_string()
            } else {
                let mut chars = part.chars();
                if let Some(first) = chars.next() {
                    format!("{}{}", first.to_uppercase(), chars.as_str().to_lowercase())
                } else {
                    String::new()
                }
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

pub(super) fn detect_sub_specs(readme_path: &str) -> Vec<SubSpec> {
    let Some(parent_dir) = FsPath::new(readme_path).parent() else {
        return Vec::new();
    };

    let mut sub_specs = Vec::new();

    let entries = match std::fs::read_dir(parent_dir) {
        Ok(entries) => entries,
        Err(_) => return sub_specs,
    };

    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            continue;
        }

        let Some(file_name) = path.file_name().and_then(|n| n.to_str()) else {
            continue;
        };

        let lower_name = file_name.to_ascii_lowercase();
        if file_name == "README.md" || !lower_name.ends_with(".md") {
            continue;
        }

        let Ok(raw) = std::fs::read_to_string(&path) else {
            continue;
        };

        let content = strip_frontmatter(&raw);

        sub_specs.push(SubSpec {
            name: format_sub_spec_name(file_name),
            file: file_name.to_string(),
            content,
        });
    }

    sub_specs.sort_by(|a, b| a.file.to_lowercase().cmp(&b.file.to_lowercase()));
    sub_specs
}

pub(super) fn load_project_config(project_path: &FsPath) -> Option<HarnSpecConfig> {
    let config_json = project_path.join(".harnspec/config.json");
    if config_json.exists() {
        if let Ok(content) = std::fs::read_to_string(&config_json) {
            if let Ok(config) = serde_json::from_str::<HarnSpecConfig>(&content) {
                return Some(config);
            }
        }
    }

    let config_yaml = project_path.join(".harnspec/config.yaml");
    if config_yaml.exists() {
        if let Ok(content) = std::fs::read_to_string(&config_yaml) {
            if let Ok(config) = serde_yaml::from_str::<HarnSpecConfig>(&content) {
                return Some(config);
            }
        }
    }

    None
}
