//! View tool — view a spec's full content and metadata

use harnspec_core::SpecLoader;
use serde_json::{json, Value};

pub(crate) fn tool_view(specs_dir: &str, args: Value) -> Result<String, String> {
    let spec_path = args
        .get("specPath")
        .and_then(|v| v.as_str())
        .ok_or("Missing required parameter: specPath")?;

    let loader = SpecLoader::new(specs_dir);
    let spec = loader
        .load(spec_path)
        .map_err(|e| e.to_string())?
        .ok_or_else(|| format!("Spec not found: {}", spec_path))?;

    let relationship_index = loader
        .load_relationship_index()
        .map_err(|e| e.to_string())?;
    let children = relationship_index
        .children_by_parent
        .get(&spec.path)
        .cloned()
        .unwrap_or_default();
    let required_by = relationship_index
        .required_by
        .get(&spec.path)
        .cloned()
        .unwrap_or_default();

    let output = json!({
        "path": spec.path,
        "title": spec.title,
        "status": spec.frontmatter.status.to_string(),
        "created": spec.frontmatter.created,
        "priority": spec.frontmatter.priority.map(|p| p.to_string()),
        "tags": spec.frontmatter.tags,
        "depends_on": spec.frontmatter.depends_on,
        "assignee": spec.frontmatter.assignee,
        "parent": spec.frontmatter.parent,
        "children": children,
        "required_by": required_by,
        "content": spec.content,
    });

    serde_json::to_string_pretty(&output).map_err(|e| e.to_string())
}

pub(crate) fn get_definition() -> crate::protocol::ToolDefinition {
    crate::protocol::ToolDefinition {
        name: "view".to_string(),
        description: "View a spec's full content and metadata".to_string(),
        input_schema: json!({
            "type": "object",
            "properties": {
                "specPath": {
                    "type": "string",
                    "description": "Spec path or number (e.g., '170' or '170-cli-mcp')"
                }
            },
            "required": ["specPath"]
        }),
    }
}
