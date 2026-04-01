//! Search tool — search specs by query

use harnspec_core::SpecLoader;
use serde_json::{json, Value};

pub(crate) fn tool_search(specs_dir: &str, args: Value) -> Result<String, String> {
    use harnspec_core::{search_specs, validate_search_query};

    let query = args
        .get("query")
        .and_then(|v| v.as_str())
        .ok_or("Missing required parameter: query")?;

    let limit = args.get("limit").and_then(|v| v.as_u64()).unwrap_or(10) as usize;

    let loader = SpecLoader::new(specs_dir);
    // Search needs body content for full-text matching.
    let specs = loader.load_all().map_err(|e| e.to_string())?;

    // Check for empty query
    if query.trim().is_empty() {
        return serde_json::to_string_pretty(&json!({
            "query": query,
            "count": 0,
            "results": [],
            "error": "Empty search query"
        }))
        .map_err(|e| e.to_string());
    }

    if let Err(err) = validate_search_query(query) {
        return serde_json::to_string_pretty(&json!({
            "query": query,
            "count": 0,
            "results": [],
            "error": format!("Invalid search query: {}", err)
        }))
        .map_err(|e| e.to_string());
    }

    // Use core search module
    let results = search_specs(&specs, query, limit);

    serde_json::to_string_pretty(&json!({
        "query": query,
        "count": results.len(),
        "results": results
    }))
    .map_err(|e| e.to_string())
}

pub(crate) fn get_definition() -> crate::protocol::ToolDefinition {
    crate::protocol::ToolDefinition {
        name: "search".to_string(),
        description: "Search specs by query. Supports AND/OR/NOT, field filters (status:, tag:, priority:, title:, created:), quoted phrases, and fuzzy terms (~). Examples: \"api AND security\", \"tag:api status:planned\", \"created:>2025-11\", \"authetication~\".".to_string(),
        input_schema: json!({
            "type": "object",
            "properties": {
                "query": {
                    "type": "string",
                    "description": "Search query (e.g., \"api AND security\", \"tag:rust status:in-progress\", \"\\\"user authentication\\\"\", \"auth~2\")"
                },
                "limit": {
                    "type": "number",
                    "description": "Maximum results (defaults to 10)"
                }
            },
            "required": ["query"]
        }),
    }
}
