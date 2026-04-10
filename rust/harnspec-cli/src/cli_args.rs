use clap::{ArgAction, Parser, Subcommand};

#[derive(Parser)]
#[command(name = "harnspec")]
#[command(
    author,
    version,
    about = "Lightweight spec methodology for AI-powered development"
)]
#[command(propagate_version = true)]
pub(crate) struct Cli {
    #[command(subcommand)]
    pub(crate) command: Commands,

    /// Specs directory path (default: ./specs)
    #[arg(short = 'd', long, global = true)]
    pub(crate) specs_dir: Option<String>,

    /// Output format: text, json
    #[arg(short = 'o', long, global = true, default_value = "text")]
    pub(crate) output: String,

    /// Suppress non-essential output
    #[arg(short, long, global = true)]
    pub(crate) quiet: bool,
}

#[derive(Subcommand)]
pub(crate) enum Commands {
    /// Dispatch specs to AI coding agents
    Agent(Box<AgentParams>),

    /// Analyze spec complexity and structure
    Analyze(Box<AnalyzeParams>),

    /// Archive spec(s) by setting status to archived
    Archive(Box<ArchiveParams>),

    /// Backfill timestamps from git history
    Backfill(Box<BackfillParams>),

    /// Show project board view
    Board(Box<BoardParams>),

    /// Check for sequence conflicts
    Check(Box<CheckParams>),

    /// List child specs for a parent
    Children(Box<ChildrenParams>),

    /// Remove specified line ranges from spec
    Compact(Box<CompactParams>),

    /// Create a new spec
    Create(Box<CreateParams>),

    /// List example projects
    Examples,

    /// Manage spec relationships (hierarchy and dependencies)
    Rel(Box<RelParams>),

    /// List files in a spec directory
    Files(Box<FilesParams>),

    /// Manage Git repository integration
    Git {
        #[command(subcommand)]
        action: Box<GitSubcommand>,
    },

    /// Show timeline with dependencies
    Gantt(Box<GanttParams>),

    /// Initialize HarnSpec in current directory
    Init(Box<InitParams>),

    /// Run a configured runner from the current project
    Run(Box<RunParams>),

    /// List all specs with optional filtering
    List(Box<ListParams>),

    /// Show spec dependency graph
    Deps(Box<DepsParams>),

    /// Migrate specs from other SDD tools
    Migrate(Box<MigrateParams>),

    /// Migrate specs from archived/ folder to status-based archiving
    MigrateArchived(Box<MigrateArchivedParams>),

    /// Open spec in editor
    Open(Box<OpenParams>),

    /// Search specs
    Search(Box<SearchParams>),

    /// Split spec into multiple files
    Split(Box<SplitParams>),

    /// Show spec statistics
    Stats(Box<StatsParams>),

    /// Manage spec templates
    Templates(Box<TemplatesParams>),

    /// Show creation/completion timeline
    Timeline(Box<TimelineParams>),

    /// Count tokens in a spec or any file
    Tokens(Box<TokensParams>),

    /// Interactive terminal UI for spec management
    Tui(Box<TuiParams>),

    /// Start local web UI for spec management
    #[command(name = "ui", alias = "web")]
    Ui(Box<UiParams>),

    /// Update a spec's frontmatter
    Update(Box<UpdateParams>),

    /// Validate specs for issues
    Validate(Box<ValidateParams>),

    /// View a spec's details
    View(Box<ViewParams>),

    /// Manage AI coding sessions
    Session {
        #[command(subcommand)]
        action: Box<SessionSubcommand>,
    },

    /// Manage AI agent skills
    Skills {
        #[command(subcommand)]
        action: SkillSubcommand,
    },

    /// Manage AI runners
    Runner {
        #[command(subcommand)]
        action: Box<RunnerSubcommand>,
    },
}

#[derive(Parser)]
pub(crate) struct AgentParams {
    /// Action: run, list, status, config
    #[arg(default_value = "help")]
    pub(crate) action: String,
    /// Specs to dispatch (for run action)
    pub(crate) specs: Option<Vec<String>>,
    /// Agent type (claude, copilot, aider, gemini, cursor, continue)
    #[arg(long, default_value = "claude")]
    pub(crate) agent: Option<String>,
    /// Create worktrees for parallel implementation
    #[arg(long)]
    pub(crate) parallel: bool,
    /// Do not update spec status to in-progress
    #[arg(long)]
    pub(crate) no_status_update: bool,
    /// Preview without making changes
    #[arg(long)]
    pub(crate) dry_run: bool,
}

#[derive(Parser)]
pub(crate) struct AnalyzeParams {
    /// Spec path or number
    pub(crate) spec: String,
}

#[derive(Parser)]
pub(crate) struct ArchiveParams {
    /// Spec paths or numbers (supports batch operations)
    #[arg(required = true)]
    pub(crate) specs: Vec<String>,
    /// Preview changes without applying
    #[arg(long)]
    pub(crate) dry_run: bool,
}

#[derive(Parser)]
pub(crate) struct BackfillParams {
    /// Specific specs to backfill
    pub(crate) specs: Option<Vec<String>>,
    /// Preview without making changes
    #[arg(long)]
    pub(crate) dry_run: bool,
    /// Overwrite existing values
    #[arg(long)]
    pub(crate) force: bool,
    /// Include assignee from git author
    #[arg(long)]
    pub(crate) assignee: bool,
    /// Include status transitions
    #[arg(long)]
    pub(crate) transitions: bool,
    /// Include all optional fields
    #[arg(long)]
    pub(crate) all: bool,
    /// Create frontmatter for files without it
    #[arg(long)]
    pub(crate) bootstrap: bool,
}

#[derive(Parser)]
pub(crate) struct BoardParams {
    /// Group by: status, priority, assignee, tag, parent
    #[arg(short, long, default_value = "status")]
    pub(crate) group_by: String,
}

#[derive(Parser)]
pub(crate) struct CheckParams {
    /// Attempt to fix conflicts
    #[arg(long)]
    pub(crate) fix: bool,
}

#[derive(Parser)]
pub(crate) struct ChildrenParams {
    /// Spec path or number
    pub(crate) spec: String,
}

#[derive(Parser)]
pub(crate) struct CompactParams {
    /// Spec to compact
    pub(crate) spec: String,
    /// Line range to remove (e.g., 145-153)
    #[arg(long = "remove")]
    pub(crate) removes: Vec<String>,
    /// Preview without making changes
    #[arg(long)]
    pub(crate) dry_run: bool,
}

#[derive(Parser)]
pub(crate) struct CreateParams {
    /// Spec name (e.g., "my-feature")
    pub(crate) name: String,
    /// Spec title
    #[arg(short, long)]
    pub(crate) title: Option<String>,
    /// Template to use
    #[arg(short = 'T', long)]
    pub(crate) template: Option<String>,
    /// Initial status
    #[arg(short, long)]
    pub(crate) status: Option<String>,
    /// Priority level
    #[arg(short, long, default_value = "medium")]
    pub(crate) priority: String,
    /// Tags (comma-separated)
    #[arg(long)]
    pub(crate) tags: Option<String>,
    /// Parent umbrella spec path or number
    #[arg(long)]
    pub(crate) parent: Option<String>,
    /// Spec(s) this new spec depends on
    #[arg(long = "depends-on", num_args = 1..)]
    pub(crate) depends_on: Vec<String>,
    /// Full markdown content for the spec body (may include frontmatter)
    #[arg(long, allow_hyphen_values = true)]
    pub(crate) content: Option<String>,
    /// Read spec content from a file path (takes precedence over --content)
    #[arg(short, long)]
    pub(crate) file: Option<String>,
    /// Assignee for the spec
    #[arg(short, long)]
    pub(crate) assignee: Option<String>,
    /// Short description (inserted into template body under the title)
    #[arg(long)]
    pub(crate) description: Option<String>,
}

#[derive(Parser)]
pub(crate) struct RelParams {
    /// Arguments: <spec> or <action> <spec>
    #[arg(required = true, num_args = 1..=2)]
    pub(crate) args: Vec<String>,
    /// Set or clear parent relationship
    #[arg(long, num_args = 0..=1, default_missing_value = "")]
    pub(crate) parent: Option<String>,
    /// Add or remove child relationships
    #[arg(long = "child", num_args = 1..)]
    pub(crate) child: Vec<String>,
    /// Add or remove dependency relationships
    #[arg(long = "depends-on", num_args = 1..)]
    pub(crate) depends_on: Vec<String>,
}

#[derive(Parser)]
pub(crate) struct FilesParams {
    /// Spec path or number
    pub(crate) spec: String,
    /// Show file sizes
    #[arg(short, long)]
    pub(crate) size: bool,
}

#[derive(Parser)]
pub(crate) struct GanttParams {
    /// Filter by status
    #[arg(short, long)]
    pub(crate) status: Option<String>,
}

#[derive(Parser)]
pub(crate) struct InitParams {
    /// Skip prompts and use defaults
    #[arg(short, long)]
    pub(crate) yes: bool,
    /// Initialize an example project
    #[arg(long)]
    pub(crate) example: Option<String>,
    /// Skip AI tool configuration (symlinks)
    #[arg(long)]
    pub(crate) no_ai_tools: bool,
    /// Install HarnSpec agent skills (project-level default)
    #[arg(long)]
    pub(crate) skill: bool,
    /// Install skills to .github/skills/
    #[arg(long)]
    pub(crate) skill_github: bool,
    /// Install skills to .claude/skills/
    #[arg(long)]
    pub(crate) skill_claude: bool,
    /// Install skills to .cursor/skills/
    #[arg(long)]
    pub(crate) skill_cursor: bool,
    /// Install skills to .codex/skills/
    #[arg(long)]
    pub(crate) skill_codex: bool,
    /// Install skills to .gemini/skills/
    #[arg(long)]
    pub(crate) skill_gemini: bool,
    /// Install skills to .vscode/skills/
    #[arg(long)]
    pub(crate) skill_vscode: bool,
    /// Install skills to user-level directories (e.g., ~/.copilot/skills)
    #[arg(long)]
    pub(crate) skill_user: bool,
    /// Skip skill installation entirely
    #[arg(long)]
    pub(crate) no_skill: bool,
}

#[derive(Parser)]
pub(crate) struct RunParams {
    /// Optional project path (defaults to current directory)
    #[arg(long)]
    pub(crate) project_path: Option<String>,

    /// Inline prompt to send to the runner
    #[arg(short = 'p', long)]
    pub(crate) prompt: Option<String>,
    /// Spec IDs to attach as context (repeatable: --spec 028 --spec 320)
    #[arg(long, action = clap::ArgAction::Append)]
    pub(crate) spec: Vec<String>,
    /// Runner ID to use (defaults to configured default runner)
    #[arg(long)]
    pub(crate) runner: Option<String>,
    /// Override the runner model if supported
    #[arg(long)]
    pub(crate) model: Option<String>,
    /// Show the composed command without executing it
    #[arg(long)]
    pub(crate) dry_run: bool,
    /// Force ACP protocol for this invocation
    #[arg(long)]
    pub(crate) acp: bool,
    /// Run the session inside a dedicated git worktree
    #[arg(long)]
    pub(crate) worktree: bool,
    /// Run each provided spec in parallel worktrees
    #[arg(long)]
    pub(crate) parallel: bool,
    /// Merge strategy to use for worktree sessions
    #[arg(long)]
    pub(crate) merge_strategy: Option<String>,
}

#[derive(Parser)]
pub(crate) struct ListParams {
    /// Filter by status: draft, planned, in-progress, complete, archived
    #[arg(short, long)]
    pub(crate) status: Option<String>,
    /// Filter by tag
    #[arg(short, long)]
    pub(crate) tag: Option<Vec<String>>,
    /// Filter by priority: low, medium, high, critical
    #[arg(short, long)]
    pub(crate) priority: Option<String>,
    /// Filter by assignee
    #[arg(short, long)]
    pub(crate) assignee: Option<String>,
    /// Show compact output
    #[arg(short, long)]
    pub(crate) compact: bool,
    /// Show parent-child hierarchy tree
    #[arg(long)]
    pub(crate) hierarchy: bool,
}

#[derive(Parser)]
pub(crate) struct DepsParams {
    /// Spec path or number
    pub(crate) spec: String,
    /// Maximum depth to traverse
    #[arg(short = 'D', long, default_value = "3")]
    pub(crate) depth: usize,
    /// Show upstream dependencies only
    #[arg(long)]
    pub(crate) upstream: bool,
    /// Show downstream dependents only
    #[arg(long)]
    pub(crate) downstream: bool,
}

#[derive(Parser)]
pub(crate) struct MigrateParams {
    /// Path to directory containing specs to migrate
    pub(crate) input_path: String,
    /// Automatic migration
    #[arg(long)]
    pub(crate) auto: bool,
    /// AI-assisted migration (copilot, claude, gemini)
    #[arg(long = "with")]
    pub(crate) ai_provider: Option<String>,
    /// Preview without making changes
    #[arg(long)]
    pub(crate) dry_run: bool,
    /// Process N docs at a time
    #[arg(long)]
    pub(crate) batch_size: Option<usize>,
    /// Don't validate after migration
    #[arg(long)]
    pub(crate) skip_validation: bool,
    /// Auto-run backfill after migration
    #[arg(long)]
    pub(crate) backfill: bool,
}

#[derive(Parser)]
pub(crate) struct MigrateArchivedParams {
    /// Preview without making changes
    #[arg(long)]
    pub(crate) dry_run: bool,
}

#[derive(Parser)]
pub(crate) struct OpenParams {
    /// Spec path or number
    pub(crate) spec: String,
    /// Editor to use (default: $EDITOR or platform default)
    #[arg(short, long)]
    pub(crate) editor: Option<String>,
}

#[derive(Parser)]
pub(crate) struct SearchParams {
    pub(crate) query: String,
    #[arg(short, long, default_value = "10")]
    pub(crate) limit: usize,
}

#[derive(Parser)]
pub(crate) struct SplitParams {
    pub(crate) spec: String,
    #[arg(long = "to")]
    pub(crate) outputs: Vec<String>,
    #[arg(long)]
    pub(crate) update_refs: bool,
    #[arg(long)]
    pub(crate) dry_run: bool,
}

#[derive(Parser)]
pub(crate) struct StatsParams {
    #[arg(long)]
    pub(crate) detailed: bool,
}

#[derive(Parser)]
pub(crate) struct TemplatesParams {
    #[arg(short, long)]
    pub(crate) action: Option<String>,
    pub(crate) name: Option<String>,
}

#[derive(Parser)]
pub(crate) struct TimelineParams {
    #[arg(short, long, default_value = "6")]
    pub(crate) months: usize,
}

#[derive(Parser)]
pub(crate) struct TokensParams {
    pub(crate) path: Option<String>,
    #[arg(short, long)]
    pub(crate) verbose: bool,
}

#[derive(Parser)]
pub(crate) struct TuiParams {
    #[arg(long, default_value = "board")]
    pub(crate) view: String,
    #[arg(long)]
    pub(crate) project: Option<String>,
    #[arg(long)]
    pub(crate) headless: Option<String>,
}

#[derive(Parser)]
pub(crate) struct UiParams {
    /// Port to listen on
    #[arg(short, long, default_value = "3000")]
    pub(crate) port: String,

    /// Do not auto-open browser
    #[arg(long)]
    pub(crate) no_open: bool,

    /// Enable multi-project mode
    #[arg(long)]
    pub(crate) multi_project: bool,

    /// Development mode (monorepo only)
    #[arg(long)]
    pub(crate) dev: bool,

    /// Preview command without running
    #[arg(long)]
    pub(crate) dry_run: bool,

    /// Shut down the running UI server
    #[arg(short, long)]
    pub(crate) quit: bool,
}

#[derive(Parser)]
pub(crate) struct UpdateParams {
    /// Spec path(s) or number(s)
    #[arg(required = true, num_args = 1..)]
    pub(crate) specs: Vec<String>,
    /// New status
    #[arg(short, long)]
    pub(crate) status: Option<String>,
    /// New priority
    #[arg(short, long)]
    pub(crate) priority: Option<String>,
    /// New assignee
    #[arg(short, long)]
    pub(crate) assignee: Option<String>,
    /// Add tags
    #[arg(long)]
    pub(crate) add_tags: Option<String>,
    /// Remove tags
    #[arg(long)]
    pub(crate) remove_tags: Option<String>,
    /// Replace text (repeatable: --replace "old" "new")
    #[arg(long = "replace", num_args = 2, value_names = ["OLD", "NEW"], action = ArgAction::Append)]
    pub(crate) replacements: Vec<String>,
    /// Replace all matches (applies to all --replace entries)
    #[arg(long, conflicts_with = "match_first")]
    pub(crate) match_all: bool,
    /// Replace first match only (applies to all --replace entries)
    #[arg(long, conflicts_with = "match_all")]
    pub(crate) match_first: bool,
    /// Check checklist item (repeatable)
    #[arg(long, action = ArgAction::Append)]
    pub(crate) check: Vec<String>,
    /// Uncheck checklist item (repeatable)
    #[arg(long, action = ArgAction::Append)]
    pub(crate) uncheck: Vec<String>,
    /// Section heading to update
    #[arg(long)]
    pub(crate) section: Option<String>,
    /// Replace content for section
    #[arg(long, conflicts_with_all = ["append", "prepend"])]
    pub(crate) section_content: Option<String>,
    /// Append content to section
    #[arg(long, conflicts_with = "section_content")]
    pub(crate) append: Option<String>,
    /// Prepend content to section
    #[arg(long, conflicts_with = "section_content")]
    pub(crate) prepend: Option<String>,
    /// Replace full body content (frontmatter preserved)
    #[arg(long)]
    pub(crate) content: Option<String>,
    /// Skip completion verification or stage skipping guard (draft -> in-progress/complete)
    #[arg(short, long)]
    pub(crate) force: bool,
    /// Expected content hash for optimistic concurrency (fails if content changed)
    #[arg(long = "expected-hash")]
    pub(crate) expected_hash: Option<String>,
}

#[derive(Parser)]
pub(crate) struct ValidateParams {
    pub(crate) spec: Option<String>,
    #[arg(long)]
    pub(crate) check_deps: bool,
    #[arg(long)]
    pub(crate) strict: bool,
    #[arg(long)]
    pub(crate) warnings_only: bool,
}

#[derive(Parser)]
pub(crate) struct ViewParams {
    pub(crate) spec: String,
    #[arg(long)]
    pub(crate) raw: bool,
}

#[derive(Subcommand)]
pub(crate) enum SkillSubcommand {
    /// Install official HarnSpec skills to the current project
    Install {
        /// Agents to install to (e.g. claude, copilot, cursor)
        #[arg(long, action = ArgAction::Append)]
        agent: Vec<String>,

        /// Skip confirmation prompts
        #[arg(short, long)]
        yes: bool,
    },
}

#[derive(Subcommand)]
pub(crate) enum GitSubcommand {
    /// Detect specs in a Git repository
    Detect {
        /// Repository (owner/repo or git URL)
        repo: String,

        /// Branch to check (default: repo's default branch)
        #[arg(short, long)]
        branch: Option<String>,
    },

    /// Import a Git repo as a HarnSpec project
    Import {
        /// Repository (owner/repo or git URL)
        repo: String,

        /// Branch to track (default: repo's default branch)
        #[arg(short, long)]
        branch: Option<String>,

        /// Display name for the project
        #[arg(short, long)]
        name: Option<String>,
    },
}

#[derive(Subcommand)]
pub(crate) enum SessionSubcommand {
    Create {
        #[arg(long)]
        project_path: Option<String>,

        /// Spec IDs to attach as context (repeatable: --spec 028 --spec 320)
        #[arg(long, action = clap::ArgAction::Append)]
        spec: Vec<String>,

        /// Optional custom prompt/instructions for the session
        #[arg(long)]
        prompt: Option<String>,

        #[arg(long)]
        runner: Option<String>,

        #[arg(long)]
        model: Option<String>,

        #[arg(long)]
        acp: bool,

        #[arg(long)]
        worktree: bool,

        #[arg(long)]
        merge_strategy: Option<String>,

        #[arg(long, default_value = "autonomous")]
        mode: String,
    },
    Run {
        #[arg(long)]
        project_path: Option<String>,

        /// Spec IDs to attach as context (repeatable: --spec 028 --spec 320)
        #[arg(long, action = clap::ArgAction::Append)]
        spec: Vec<String>,

        /// Optional custom prompt/instructions for the session
        #[arg(long)]
        prompt: Option<String>,

        #[arg(long)]
        runner: Option<String>,

        #[arg(long)]
        model: Option<String>,

        #[arg(long)]
        acp: bool,

        #[arg(long)]
        worktree: bool,

        #[arg(long)]
        parallel: bool,

        #[arg(long)]
        merge_strategy: Option<String>,

        #[arg(long, default_value = "autonomous")]
        mode: String,
    },
    Start {
        session_id: String,
    },
    Pause {
        session_id: String,
    },
    Resume {
        session_id: String,
    },
    Stop {
        session_id: String,
    },
    Archive {
        session_id: String,

        #[arg(long)]
        output_dir: Option<String>,

        #[arg(long, default_value_t = false)]
        compress: bool,
    },
    RotateLogs {
        session_id: String,

        #[arg(long, default_value_t = 10_000)]
        keep: usize,
    },
    Delete {
        session_id: String,
    },
    View {
        session_id: String,
    },
    List {
        #[arg(long)]
        spec: Option<String>,
        #[arg(long)]
        status: Option<String>,
        #[arg(long)]
        runner: Option<String>,
    },
    Logs {
        session_id: String,
    },
    Worktrees {
        #[arg(long)]
        all: bool,
    },
    Merge {
        session_id: String,

        #[arg(long)]
        strategy: Option<String>,

        #[arg(long)]
        resolve: bool,
    },
    Cleanup {
        session_id: String,

        #[arg(long)]
        keep_branch: bool,
    },
    Gc,
}

#[derive(Subcommand)]
pub(crate) enum RunnerSubcommand {
    /// List configured runners
    List {
        /// Optional project path (defaults to current directory)
        #[arg(long)]
        project_path: Option<String>,
    },
    /// Show a runner configuration
    Show {
        runner_id: String,

        /// Optional project path (defaults to current directory)
        #[arg(long)]
        project_path: Option<String>,
    },
    /// Validate runners by checking command availability
    Validate {
        runner_id: Option<String>,

        /// Optional project path (defaults to current directory)
        #[arg(long)]
        project_path: Option<String>,
    },
    /// Open runners config file
    Config {
        /// Use global config instead of project config
        #[arg(long)]
        global: bool,

        /// Optional project path (defaults to current directory)
        #[arg(long)]
        project_path: Option<String>,
    },
}
