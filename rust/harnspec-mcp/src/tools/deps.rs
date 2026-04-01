//! Deps tool — show dependency graph around a spec

use harnspec_core::SpecLoader;
use serde_json::{json, Value};
use std::collections::{HashMap, HashSet};

fn collect_upstream(
    path: &str,
    depth: usize,
    specs_map: &HashMap<String, harnspec_core::SpecInfo>,
    visited: &mut HashSet<String>,
    output: &mut Vec<String>,
) {
    if depth == 0 || !visited.insert(path.to_string()) {
        return;
    }

    if let Some(spec) = specs_map.get(path) {
        for dep in &spec.frontmatter.depends_on {
            if !output.contains(dep) {
                output.push(dep.clone());
            }
            collect_upstream(dep, depth - 1, specs_map, visited, output);
        }
    }
}

fn collect_downstream(
    path: &str,
    depth: usize,
    specs_map: &HashMap<String, harnspec_core::SpecInfo>,
    visited: &mut HashSet<String>,
    output: &mut Vec<String>,
) {
    if depth == 0 || !visited.insert(path.to_string()) {
        return;
    }

    for (candidate_path, candidate_spec) in specs_map {
        if candidate_spec
            .frontmatter
            .depends_on
            .iter()
            .any(|d| d == path)
        {
            if !output.contains(candidate_path) {
                output.push(candidate_path.clone());
            }
            collect_downstream(candidate_path, depth - 1, specs_map, visited, output);
        }
    }
}

pub(crate) fn tool_deps(specs_dir: &str, args: Value) -> Result<String, String> {
    let spec_path = args
        .get("specPath")
        .and_then(|v| v.as_str())
        .ok_or("Missing required parameter: specPath")?;

    let depth = args.get("depth").and_then(|v| v.as_u64()).unwrap_or(3) as usize;
    let upstream_only = args
        .get("upstream")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let downstream_only = args
        .get("downstream")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);

    let loader = SpecLoader::new(specs_dir);
    let root = loader
        .load(spec_path)
        .map_err(|e| e.to_string())?
        .ok_or_else(|| format!("Spec not found: {}", spec_path))?;

    let all_specs = loader.load_all_metadata().map_err(|e| e.to_string())?;
    let specs_map: HashMap<String, harnspec_core::SpecInfo> =
        all_specs.into_iter().map(|s| (s.path.clone(), s)).collect();

    let mut upstream_paths = Vec::new();
    let mut downstream_paths = Vec::new();

    if !downstream_only {
        collect_upstream(
            &root.path,
            depth,
            &specs_map,
            &mut HashSet::new(),
            &mut upstream_paths,
        );
    }

    if !upstream_only {
        collect_downstream(
            &root.path,
            depth,
            &specs_map,
            &mut HashSet::new(),
            &mut downstream_paths,
        );
    }

    let to_summary = |path: &String| {
        specs_map
            .get(path)
            .map(|s| {
                json!({
                    "path": s.path,
                    "title": s.title,
                    "status": s.frontmatter.status.to_string()
                })
            })
            .unwrap_or_else(|| json!({ "path": path }))
    };

    serde_json::to_string_pretty(&json!({
        "spec": {
            "path": root.path,
            "title": root.title,
            "status": root.frontmatter.status.to_string()
        },
        "depth": depth,
        "upstream": upstream_paths.iter().map(to_summary).collect::<Vec<_>>(),
        "downstream": downstream_paths.iter().map(to_summary).collect::<Vec<_>>()
    }))
    .map_err(|e| e.to_string())
}

pub(crate) fn get_definition() -> crate::protocol::ToolDefinition {
    crate::protocol::ToolDefinition {
        name: "deps".to_string(),
        description: "Show dependency graph around a spec (upstream/downstream)".to_string(),
        input_schema: json!({
            "type": "object",
            "properties": {
                "specPath": {
                    "type": "string",
                    "description": "Spec path or number"
                },
                "depth": {
                    "type": "number",
                    "description": "Maximum depth to traverse (defaults to 3)"
                },
                "upstream": {
                    "type": "boolean",
                    "description": "Show upstream dependencies only"
                },
                "downstream": {
                    "type": "boolean",
                    "description": "Show downstream dependents only"
                }
            },
            "required": ["specPath"]
        }),
    }
}
