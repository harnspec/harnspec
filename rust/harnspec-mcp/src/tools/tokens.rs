//! Tokens tool — count tokens in specs or any file

use harnspec_core::{SpecLoader, TokenCounter};
use serde_json::{json, Value};
use std::path::Path;

pub(crate) fn tool_tokens(specs_dir: &str, args: Value) -> Result<String, String> {
    let loader = SpecLoader::new(specs_dir);
    let counter = TokenCounter::new();

    // Check if filePath is provided (for generic files)
    if let Some(file_path) = args.get("filePath").and_then(|v| v.as_str()) {
        let path = Path::new(file_path);

        if !path.exists() {
            return Err(format!("File not found: {}", file_path));
        }

        if !path.is_file() {
            return Err(format!("Not a file: {}", file_path));
        }

        let content = std::fs::read_to_string(path).map_err(|e| e.to_string())?;
        let result = counter.count_file(&content);

        return serde_json::to_string_pretty(&json!({
            "path": file_path,
            "total": result.total,
            "status": format!("{:?}", result.status),
        }))
        .map_err(|e| e.to_string());
    }

    // Otherwise, handle as spec (existing behavior)
    if let Some(spec_path) = args.get("specPath").and_then(|v| v.as_str()) {
        let spec = loader
            .load(spec_path)
            .map_err(|e| e.to_string())?
            .ok_or_else(|| format!("Spec not found: {}", spec_path))?;

        let content = std::fs::read_to_string(&spec.file_path).map_err(|e| e.to_string())?;
        let result = counter.count_spec(&content);

        Ok(serde_json::to_string_pretty(&json!({
            "spec": spec.path,
            "total": result.total,
            "frontmatter": result.frontmatter,
            "content": result.content,
            "status": format!("{:?}", result.status),
        }))
        .map_err(|e| e.to_string())?)
    } else {
        let specs = loader.load_all().map_err(|e| e.to_string())?;

        let results: Vec<_> = specs
            .iter()
            .filter_map(|spec| {
                let content = std::fs::read_to_string(&spec.file_path).ok()?;
                let result = counter.count_spec(&content);
                Some(json!({
                    "path": spec.path,
                    "title": spec.title,
                    "total": result.total,
                    "status": format!("{:?}", result.status),
                }))
            })
            .collect();

        let total_tokens: usize = results
            .iter()
            .filter_map(|r| r.get("total").and_then(|v| v.as_u64()))
            .map(|v| v as usize)
            .sum();

        Ok(serde_json::to_string_pretty(&json!({
            "count": results.len(),
            "totalTokens": total_tokens,
            "averageTokens": if results.is_empty() { 0 } else { total_tokens / results.len() },
            "specs": results,
        }))
        .map_err(|e| e.to_string())?)
    }
}

pub(crate) fn get_definition() -> crate::protocol::ToolDefinition {
    crate::protocol::ToolDefinition {
        name: "tokens".to_string(),
        description: "Count tokens in spec(s) or any file for context economy".to_string(),
        input_schema: json!({
            "type": "object",
            "properties": {
                "specPath": {
                    "type": "string",
                    "description": "Specific spec to count (counts all specs if not provided)"
                },
                "filePath": {
                    "type": "string",
                    "description": "Path to any file (markdown, code, text) to count tokens"
                }
            }
        }),
    }
}
