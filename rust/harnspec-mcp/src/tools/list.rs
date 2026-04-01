//! List tool — list specs with optional filtering

use harnspec_core::SpecLoader;
use serde_json::{json, Value};

pub(crate) fn tool_list(specs_dir: &str, args: Value) -> Result<String, String> {
    let loader = SpecLoader::new(specs_dir);
    let specs = loader.load_all_metadata().map_err(|e| e.to_string())?;

    let status_filter = args.get("status").and_then(|v| v.as_str());
    let tags_filter: Option<Vec<&str>> = args
        .get("tags")
        .and_then(|v| v.as_array())
        .map(|arr| arr.iter().filter_map(|v| v.as_str()).collect());
    let priority_filter = args.get("priority").and_then(|v| v.as_str());

    let filtered: Vec<_> = specs
        .iter()
        .filter(|spec| {
            if let Some(status) = status_filter {
                if spec.frontmatter.status.to_string() != status {
                    return false;
                }
            }
            if let Some(ref tags) = tags_filter {
                if !tags
                    .iter()
                    .all(|t| spec.frontmatter.tags.contains(&t.to_string()))
                {
                    return false;
                }
            }
            if let Some(priority) = priority_filter {
                if spec.frontmatter.priority.map(|p| p.to_string()) != Some(priority.to_string()) {
                    return false;
                }
            }
            true
        })
        .collect();

    let output: Vec<_> = filtered
        .iter()
        .map(|s| {
            json!({
                "path": s.path,
                "title": s.title,
                "status": s.frontmatter.status.to_string(),
                "priority": s.frontmatter.priority.map(|p| p.to_string()),
                "tags": s.frontmatter.tags,
            })
        })
        .collect();

    serde_json::to_string_pretty(&json!({
        "count": filtered.len(),
        "specs": output
    }))
    .map_err(|e| e.to_string())
}

pub(crate) fn get_definition() -> crate::protocol::ToolDefinition {
    crate::protocol::ToolDefinition {
        name: "list".to_string(),
        description: "List all specs with optional filtering by status, tags, or priority"
            .to_string(),
        input_schema: json!({
            "type": "object",
            "properties": {
                "status": {
                    "type": "string",
                    "description": "Filter by status: draft, planned, in-progress, complete, archived",
                    "enum": ["draft", "planned", "in-progress", "complete", "archived"]
                },
                "tags": {
                    "type": "array",
                    "items": { "type": "string" },
                    "description": "Filter by tags (spec must have ALL specified tags)"
                },
                "priority": {
                    "type": "string",
                    "description": "Filter by priority: low, medium, high, critical",
                    "enum": ["low", "medium", "high", "critical"]
                }
            }
        }),
    }
}
