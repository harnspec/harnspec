//! MCP Tool implementations
//!
//! Each tool has its own module file:
//! - `list`, `view`, `create`, `update`, `search`: Spec management
//! - `relationships`, `children`, `deps`: Dependency and hierarchy management
//! - `validate`, `tokens`: Validation and token counting
//! - `board`, `stats`: Board view and statistics
//! - `helpers`: Shared utility functions

mod board;
mod children;
mod create;
mod deps;
mod helpers;
mod list;
mod relationships;
mod search;
mod stats;
mod tokens;
mod update;
mod validate;
mod view;

use crate::protocol::ToolDefinition;
use helpers::get_specs_dir;
use serde_json::Value;

// Re-export the test helper
pub use helpers::set_test_specs_dir;

/// Get all tool definitions
pub fn get_tool_definitions() -> Vec<ToolDefinition> {
    vec![
        list::get_definition(),
        view::get_definition(),
        create::get_definition(),
        update::get_definition(),
        search::get_definition(),
        relationships::get_definition(),
        children::get_definition(),
        deps::get_definition(),
        validate::get_definition(),
        tokens::get_definition(),
        board::get_definition(),
        stats::get_definition(),
    ]
}

/// Call a tool with arguments
pub async fn call_tool(name: &str, args: Value) -> Result<String, String> {
    let specs_dir = get_specs_dir();

    match name {
        "list" => list::tool_list(&specs_dir, args),
        "view" => view::tool_view(&specs_dir, args),
        "create" => create::tool_create(&specs_dir, args),
        "update" => update::tool_update(&specs_dir, args),
        "search" => search::tool_search(&specs_dir, args),
        "validate" => validate::tool_validate(&specs_dir, args),
        "tokens" => tokens::tool_tokens(&specs_dir, args),
        "board" => board::tool_board(&specs_dir, args),
        "stats" => stats::tool_stats(&specs_dir),
        "relationships" => relationships::tool_relationships(&specs_dir, args),
        "children" => children::tool_children(&specs_dir, args),
        "deps" => deps::tool_deps(&specs_dir, args),
        _ => Err(format!("Unknown tool: {}", name)),
    }
}
