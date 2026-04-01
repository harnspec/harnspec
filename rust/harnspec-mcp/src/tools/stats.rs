//! Stats tool — show spec statistics and insights

use harnspec_core::{SpecLoader, SpecStats};
use serde_json::json;

pub(crate) fn tool_stats(specs_dir: &str) -> Result<String, String> {
    let loader = SpecLoader::new(specs_dir);
    let specs = loader.load_all_metadata().map_err(|e| e.to_string())?;

    let stats = SpecStats::compute(&specs);

    serde_json::to_string_pretty(&json!({
        "total": stats.total,
        "byStatus": stats.by_status.iter().map(|(k, v)| (k.to_string(), *v)).collect::<std::collections::HashMap<_, _>>(),
        "byPriority": stats.by_priority.iter().map(|(k, v)| (k.to_string(), *v)).collect::<std::collections::HashMap<_, _>>(),
        "completionPercentage": stats.completion_percentage(),
        "activeCount": stats.active_count(),
        "withDependencies": stats.with_dependencies,
        "totalDependencies": stats.total_dependencies,
        "subSpecs": stats.sub_specs,
        "topTags": stats.top_tags(10).iter().map(|(k, v)| json!({ "tag": k, "count": v })).collect::<Vec<_>>(),
    }))
    .map_err(|e| e.to_string())
}

pub(crate) fn get_definition() -> crate::protocol::ToolDefinition {
    crate::protocol::ToolDefinition {
        name: "stats".to_string(),
        description: "Show spec statistics and insights".to_string(),
        input_schema: json!({
            "type": "object",
            "properties": {}
        }),
    }
}
