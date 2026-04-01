//! Board tool — show project board view

use harnspec_core::SpecLoader;
use serde_json::{json, Value};

pub(crate) fn tool_board(specs_dir: &str, args: Value) -> Result<String, String> {
    let group_by = args
        .get("groupBy")
        .and_then(|v| v.as_str())
        .unwrap_or("status");

    let loader = SpecLoader::new(specs_dir);
    let specs = loader.load_all_metadata().map_err(|e| e.to_string())?;

    let mut groups: std::collections::HashMap<String, Vec<serde_json::Value>> =
        std::collections::HashMap::new();

    for spec in &specs {
        let key = match group_by {
            "status" => spec.frontmatter.status.to_string(),
            "priority" => spec
                .frontmatter
                .priority
                .map(|p| p.to_string())
                .unwrap_or_else(|| "none".to_string()),
            "assignee" => spec
                .frontmatter
                .assignee
                .clone()
                .unwrap_or_else(|| "unassigned".to_string()),
            "tag" => {
                for tag in &spec.frontmatter.tags {
                    groups.entry(tag.clone()).or_default().push(json!({
                        "path": spec.path,
                        "title": spec.title,
                        "status": spec.frontmatter.status.to_string(),
                    }));
                }
                continue;
            }
            "parent" => spec
                .frontmatter
                .parent
                .clone()
                .unwrap_or_else(|| "(no-parent)".to_string()),
            _ => "unknown".to_string(),
        };

        groups.entry(key).or_default().push(json!({
            "path": spec.path,
            "title": spec.title,
            "status": spec.frontmatter.status.to_string(),
        }));
    }

    let output: Vec<_> = groups
        .into_iter()
        .map(|(name, specs)| {
            json!({
                "name": name,
                "count": specs.len(),
                "specs": specs,
            })
        })
        .collect();

    serde_json::to_string_pretty(&json!({
        "groupBy": group_by,
        "total": specs.len(),
        "groups": output
    }))
    .map_err(|e| e.to_string())
}

pub(crate) fn get_definition() -> crate::protocol::ToolDefinition {
    crate::protocol::ToolDefinition {
        name: "board".to_string(),
        description: "Show project board view grouped by status, priority, or assignee".to_string(),
        input_schema: json!({
            "type": "object",
            "properties": {
                "groupBy": {
                    "type": "string",
                    "description": "Group by: status, priority, assignee, tag, parent (defaults to 'status')",
                    "enum": ["status", "priority", "assignee", "tag", "parent"]
                }
            }
        }),
    }
}
