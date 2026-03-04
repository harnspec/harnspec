//! Create tool — create a new spec

use super::helpers::{
    create_content_description, get_next_spec_number, is_draft_status_enabled, load_config,
    merge_frontmatter, resolve_project_root, resolve_template_variables, to_title_case,
    MergeFrontmatterInput,
};
use chrono::Utc;
use leanspec_core::TemplateLoader;
use serde_json::{json, Value};

/// Strip a leading numeric prefix like "006-" from a spec name.
/// AI agents often pass names already prefixed (e.g., "006-cli-mvp") despite instructions not to.
fn strip_numeric_prefix(name: &str) -> &str {
    if name.len() > 4 && name.as_bytes()[3] == b'-' && name[..3].bytes().all(|b| b.is_ascii_digit())
    {
        &name[4..]
    } else {
        name
    }
}

pub(crate) fn tool_create(specs_dir: &str, args: Value) -> Result<String, String> {
    let name = args
        .get("name")
        .and_then(|v| v.as_str())
        .ok_or("Missing required parameter: name")?;

    let title_input = args.get("title").and_then(|v| v.as_str());
    let status_input = args.get("status").and_then(|v| v.as_str());
    let priority = args
        .get("priority")
        .and_then(|v| v.as_str())
        .or(Some("medium"));
    let template_name = args.get("template").and_then(|v| v.as_str());
    let content_override = args.get("content").and_then(|v| v.as_str());
    let tags: Vec<String> = args
        .get("tags")
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|v| v.as_str().map(String::from))
                .collect()
        })
        .unwrap_or_default();
    let parent = args.get("parent").and_then(|v| v.as_str());
    let depends_on: Vec<String> = args
        .get("dependsOn")
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|v| v.as_str().map(String::from))
                .collect()
        })
        .unwrap_or_default();

    let next_number = get_next_spec_number(specs_dir)?;
    // Strip leading numeric prefix (e.g., "006-cli-mvp" → "cli-mvp") since AI agents
    // often pass the name already prefixed despite the schema description saying not to.
    let stripped_name = strip_numeric_prefix(name);
    let spec_name = format!("{:03}-{}", next_number, stripped_name);

    let title = title_input
        .map(String::from)
        .unwrap_or_else(|| to_title_case(stripped_name));
    let now = Utc::now();
    let created_date = now.format("%Y-%m-%d").to_string();

    let project_root = resolve_project_root(specs_dir)?;
    let resolved_status = status_input.unwrap_or_else(|| {
        if is_draft_status_enabled(&project_root) {
            "draft"
        } else {
            "planned"
        }
    });

    let base_content = if let Some(content) = content_override {
        content.to_string()
    } else {
        let config = load_config(&project_root);
        let loader = TemplateLoader::with_config(&project_root, config);
        let template = loader
            .load(template_name)
            .map_err(|e| format!("Failed to load template: {}", e))?;
        resolve_template_variables(&template, &title, resolved_status, priority, &created_date)
    };

    let content = merge_frontmatter(&MergeFrontmatterInput {
        content: &base_content,
        status: resolved_status,
        priority,
        tags: &tags,
        created_date: &created_date,
        now,
        title: &title,
        parent,
        depends_on: &depends_on,
    })?;

    let spec_dir = std::path::Path::new(specs_dir).join(&spec_name);
    std::fs::create_dir_all(&spec_dir).map_err(|e| e.to_string())?;
    std::fs::write(spec_dir.join("README.md"), &content).map_err(|e| e.to_string())?;

    Ok(format!("Created spec: {}", spec_name))
}

pub(crate) fn get_definition() -> crate::protocol::ToolDefinition {
    crate::protocol::ToolDefinition {
        name: "create".to_string(),
        description: "Create a new spec".to_string(),
        input_schema: json!({
            "type": "object",
            "properties": {
                "name": {
                    "type": "string",
                    "description": "Short spec name in kebab-case (e.g., 'my-feature'). NOTE: DO NOT add spec number (NNN), it will be auto-generated."
                },
                "title": {
                    "type": "string",
                    "description": "Human-readable title"
                },
                "status": {
                    "type": "string",
                    "description": "Initial status (defaults to config or 'planned')",
                    "enum": ["draft", "planned", "in-progress", "complete", "archived"]
                },
                "priority": {
                    "type": "string",
                    "description": "Priority level (defaults to 'medium')",
                    "enum": ["low", "medium", "high", "critical"]
                },
                "template": {
                    "type": "string",
                    "description": "Template name to load from .lean-spec/templates"
                },
                "content": {
                    "type": "string",
                    "description": create_content_description()
                },
                "tags": {
                    "type": "array",
                    "items": { "type": "string" },
                    "description": "Tags for categorization"
                },
                "parent": {
                    "type": "string",
                    "description": "Parent umbrella spec path or number"
                },
                "dependsOn": {
                    "type": "array",
                    "items": { "type": "string" },
                    "description": "Specs this new spec depends on (blocking dependencies)"
                }
            },
            "required": ["name"]
        }),
    }
}
