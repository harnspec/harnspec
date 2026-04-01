//! Children tool — list direct child specs for a parent

use harnspec_core::SpecLoader;
use serde_json::{json, Value};
use std::collections::HashMap;

pub(crate) fn tool_children(specs_dir: &str, args: Value) -> Result<String, String> {
    let spec_path = args
        .get("specPath")
        .and_then(|v| v.as_str())
        .ok_or("Missing required parameter: specPath")?;

    let loader = SpecLoader::new(specs_dir);
    let parent = loader
        .load(spec_path)
        .map_err(|e| e.to_string())?
        .ok_or_else(|| format!("Spec not found: {}", spec_path))?;

    let all_specs = loader.load_all_metadata().map_err(|e| e.to_string())?;
    let spec_map: HashMap<String, &harnspec_core::SpecInfo> =
        all_specs.iter().map(|s| (s.path.clone(), s)).collect();
    let relationship_index = loader
        .load_relationship_index()
        .map_err(|e| e.to_string())?;
    let children: Vec<_> = relationship_index
        .children_by_parent
        .get(&parent.path)
        .cloned()
        .unwrap_or_default()
        .into_iter()
        .map(|path| {
            if let Some(spec) = spec_map.get(&path) {
                json!({
                    "path": spec.path,
                    "title": spec.title,
                    "status": spec.frontmatter.status.to_string()
                })
            } else {
                json!({"path": path})
            }
        })
        .collect();

    serde_json::to_string_pretty(&json!({
        "parent": {
            "path": parent.path,
            "title": parent.title,
            "status": parent.frontmatter.status.to_string()
        },
        "children": children,
        "count": children.len()
    }))
    .map_err(|e| e.to_string())
}

pub(crate) fn get_definition() -> crate::protocol::ToolDefinition {
    crate::protocol::ToolDefinition {
        name: "children".to_string(),
        description: "List direct child specs for a parent spec".to_string(),
        input_schema: json!({
            "type": "object",
            "properties": {
                "specPath": {
                    "type": "string",
                    "description": "Parent spec path or number"
                }
            },
            "required": ["specPath"]
        }),
    }
}
