//! Session management module
//!
//! Provides session types, database persistence, and runner registry.

pub mod database;
pub mod runner;
pub mod types;
pub mod worktree;

pub mod manager;

pub use database::SessionDatabase;
pub use manager::{ArchiveOptions, CreateSessionOptions, SessionManager};
pub use runner::{
    global_runners_path, project_runners_path, DetectionConfig, DetectionResult, RunnerDefinition,
    RunnerProtocol, RunnerRegistry, RunnersFile,
};
pub use types::{
    EventType, LogLevel, Session, SessionConfig, SessionEvent, SessionLog, SessionMode,
    SessionStatus,
};
pub use worktree::{
    worktree_enabled, GcResult, GitWorktreeManager, MergeOutcome, MergeStrategy, WorktreeSession,
    WorktreeStatus, WORKTREE_AUTO_MERGE_KEY, WORKTREE_BASE_BRANCH_KEY, WORKTREE_BASE_COMMIT_KEY,
    WORKTREE_BRANCH_KEY, WORKTREE_CLEANED_AT_KEY, WORKTREE_CONFLICT_FILES_KEY,
    WORKTREE_ENABLED_KEY, WORKTREE_MERGE_STRATEGY_KEY, WORKTREE_PATH_KEY, WORKTREE_STATUS_KEY,
};
