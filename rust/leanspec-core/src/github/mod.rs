//! GitHub integration for LeanSpec
//!
//! Detect, display, and manage specs from GitHub repositories.

pub mod client;
pub mod types;

pub use client::GitHubClient;
pub use types::*;
