//! HarnSpec MCP Server Library
//!
//! Model Context Protocol server for HarnSpec spec management.
//! This library provides the protocol and tool implementations.

pub mod protocol;
pub mod tools;

pub use protocol::{handle_request, McpError, McpRequest, McpResponse, ToolDefinition};
pub use tools::{call_tool, get_tool_definitions};
