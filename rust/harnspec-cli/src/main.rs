//! HarnSpec CLI - Command-line interface for spec management

mod cli_args;
mod commands;

use clap::Parser;
use colored::Colorize;
use std::process::ExitCode;

use crate::cli_args::{Cli, Commands, GitSubcommand, RunnerSubcommand, SessionSubcommand};
use std::error::Error;
use std::path::PathBuf;

fn resolve_project_path(path: Option<String>) -> Result<String, Box<dyn Error>> {
    let path_buf = match path {
        Some(p) => PathBuf::from(p),
        None => std::env::current_dir()?,
    };
    let absolute = if path_buf.exists() {
        dunce::canonicalize(&path_buf)?
    } else {
        path_buf
    };
    Ok(absolute.to_string_lossy().to_string())
}

fn main() -> ExitCode {
    let cli = Cli::parse();

    // Determine specs directory (for non-TUI commands that use specs_dir directly).
    // We keep `cli.specs_dir` intact so TUI can detect whether it was explicitly set.
    let specs_dir = cli.specs_dir.clone().unwrap_or_else(|| "specs".to_string());

    let result = match cli.command {
        Commands::Agent(params) => commands::agent::run(
            &specs_dir,
            &params.action,
            params.specs.clone(),
            params.agent.clone(),
            params.parallel,
            params.no_status_update,
            params.dry_run,
            &cli.output,
        ),
        Commands::Analyze(params) => commands::analyze::run(&specs_dir, &params.spec, &cli.output),
        Commands::Archive(params) => {
            commands::archive::run(&specs_dir, &params.specs, params.dry_run)
        }
        Commands::Backfill(params) => commands::backfill::run(
            &specs_dir,
            params.specs.clone(),
            params.dry_run,
            params.force,
            params.assignee || params.all,
            params.transitions || params.all,
            params.bootstrap,
            &cli.output,
        ),
        Commands::Board(params) => commands::board::run(&specs_dir, &params.group_by, &cli.output),
        Commands::Check(params) => commands::check::run(&specs_dir, params.fix, &cli.output),
        Commands::Children(params) => {
            commands::children::run(&specs_dir, &params.spec, &cli.output)
        }
        Commands::Compact(params) => commands::compact::run(
            &specs_dir,
            &params.spec,
            params.removes.clone(),
            params.dry_run,
            &cli.output,
        ),
        Commands::Create(params) => commands::create::run(commands::create::CreateParams {
            specs_dir: specs_dir.clone(),
            name: params.name.clone(),
            title: params.title.clone(),
            template: params.template.clone(),
            status: params.status.clone(),
            priority: params.priority.clone(),
            tags: params.tags.clone(),
            parent: params.parent.clone(),
            depends_on: params.depends_on.clone(),
            content: params.content.clone(),
            file: params.file.clone(),
            assignee: params.assignee.clone(),
            description: params.description.clone(),
        }),
        Commands::Rel(params) => commands::rel::run(
            &specs_dir,
            commands::rel::RelArgs {
                args: params.args.clone(),
                parent: params.parent.clone(),
                children: params.child.clone(),
                depends_on: params.depends_on.clone(),
            },
            &cli.output,
        ),
        Commands::Examples => commands::examples::run(&cli.output),
        Commands::Deps(params) => commands::deps::run(
            &specs_dir,
            &params.spec,
            params.depth,
            params.upstream,
            params.downstream,
            &cli.output,
        ),
        Commands::Files(params) => {
            commands::files::run(&specs_dir, &params.spec, params.size, &cli.output)
        }
        Commands::Git { action } => {
            use commands::git_repo::GitRepoCommand as Cmd;
            let cmd = match *action {
                GitSubcommand::Detect {
                    ref repo,
                    ref branch,
                } => Cmd::Detect {
                    repo: repo.clone(),
                    branch: branch.clone(),
                },
                GitSubcommand::Import {
                    ref repo,
                    ref branch,
                    ref name,
                } => Cmd::Import {
                    repo: repo.clone(),
                    branch: branch.clone(),
                    name: name.clone(),
                },
            };
            commands::git_repo::run(cmd, &cli.output)
        }
        Commands::Gantt(params) => {
            commands::gantt::run(&specs_dir, params.status.clone(), &cli.output)
        }
        Commands::Init(params) => commands::init::run(
            &specs_dir,
            commands::init::InitOptions {
                yes: params.yes,
                example: params.example.clone(),
                no_ai_tools: params.no_ai_tools,
                skill: params.skill,
                skill_github: params.skill_github,
                skill_claude: params.skill_claude,
                skill_cursor: params.skill_cursor,
                skill_codex: params.skill_codex,
                skill_gemini: params.skill_gemini,
                skill_vscode: params.skill_vscode,
                skill_user: params.skill_user,
                no_skill: params.no_skill,
            },
        ),
        Commands::Run(params) => (|| {
            let project_path = resolve_project_path(params.project_path.clone())?;
            commands::session::run_direct(commands::session::RunDirectRequest {
                project_path,
                specs: params.spec.clone(),
                prompt: params.prompt.clone(),
                runner: params.runner.clone(),
                model: params.model.clone(),
                dry_run: params.dry_run,
                acp: params.acp,
                worktree: params.worktree,
                parallel: params.parallel,
                merge_strategy: params.merge_strategy.clone(),
            })
        })(),
        Commands::List(params) => commands::list::run(commands::list::ListParams {
            specs_dir: specs_dir.clone(),
            status: params.status.clone(),
            tags: params.tag.clone(),
            priority: params.priority.clone(),
            assignee: params.assignee.clone(),
            compact: params.compact,
            hierarchy: params.hierarchy,
            output_format: cli.output.clone(),
        }),
        Commands::Migrate(params) => commands::migrate::run(
            &specs_dir,
            &params.input_path,
            params.auto,
            params.ai_provider.clone(),
            params.dry_run,
            params.batch_size,
            params.skip_validation,
            params.backfill,
            &cli.output,
        ),
        Commands::MigrateArchived(params) => {
            commands::migrate_archived::run(&specs_dir, params.dry_run)
        }
        Commands::Open(params) => {
            commands::open::run(&specs_dir, &params.spec, params.editor.clone())
        }
        Commands::Search(params) => {
            commands::search::run(&specs_dir, &params.query, params.limit, &cli.output)
        }
        Commands::Split(params) => commands::split::run(
            &specs_dir,
            &params.spec,
            params.outputs.clone(),
            params.update_refs,
            params.dry_run,
            &cli.output,
        ),
        Commands::Stats(params) => commands::stats::run(&specs_dir, params.detailed, &cli.output),
        Commands::Templates(params) => commands::templates::run(
            &specs_dir,
            params.action.as_deref(),
            params.name.as_deref(),
            &cli.output,
        ),
        Commands::Timeline(params) => {
            commands::timeline::run(&specs_dir, params.months, &cli.output)
        }
        Commands::Tokens(params) => commands::tokens::run(
            &specs_dir,
            params.path.as_deref(),
            params.verbose,
            &cli.output,
        ),
        Commands::Tui(params) => commands::tui::run(
            // Pass None when --specs-dir not provided so TUI can use the project registry.
            cli.specs_dir.as_deref(),
            &params.view,
            params.project.as_deref(),
            params.headless.as_deref(),
        ),
        Commands::Ui(params) => commands::ui::run(
            &specs_dir,
            &params.port,
            params.no_open,
            true,
            params.dev,
            params.dry_run,
            params.quit,
        ),
        Commands::Update(params) => commands::update::run(
            &specs_dir,
            &params.specs,
            params.status.clone(),
            params.priority.clone(),
            params.assignee.clone(),
            params.add_tags.clone(),
            params.remove_tags.clone(),
            params.replacements.clone(),
            params.match_all,
            params.match_first,
            params.check.clone(),
            params.uncheck.clone(),
            params.section.clone(),
            params.section_content.clone(),
            params.append.clone(),
            params.prepend.clone(),
            params.content.clone(),
            params.force,
            params.expected_hash.clone(),
        ),
        Commands::Validate(params) => commands::validate::run(
            &specs_dir,
            params.spec.clone(),
            params.check_deps,
            params.strict,
            params.warnings_only,
            &cli.output,
        ),
        Commands::Session { action } => {
            use commands::session::SessionCommand as Cmd;
            (|| {
                let cmd = match *action {
                    SessionSubcommand::Create {
                        project_path,
                        spec,
                        prompt,
                        runner,
                        model,
                        acp,
                        worktree,
                        merge_strategy,
                        mode,
                    } => Cmd::Create {
                        project_path: resolve_project_path(project_path)?,
                        specs: spec,
                        prompt,
                        runner,
                        model,
                        acp,
                        worktree,
                        merge_strategy,
                        mode,
                    },
                    SessionSubcommand::Run {
                        project_path,
                        spec,
                        prompt,
                        runner,
                        model,
                        acp,
                        worktree,
                        parallel,
                        merge_strategy,
                        mode,
                    } => Cmd::Run {
                        project_path: resolve_project_path(project_path)?,
                        specs: spec,
                        prompt,
                        runner,
                        model,
                        acp,
                        worktree,
                        parallel,
                        merge_strategy,
                        mode,
                    },
                    SessionSubcommand::Start { ref session_id } => Cmd::Start {
                        session_id: session_id.clone(),
                    },
                    SessionSubcommand::Pause { ref session_id } => Cmd::Pause {
                        session_id: session_id.clone(),
                    },
                    SessionSubcommand::Resume { ref session_id } => Cmd::Resume {
                        session_id: session_id.clone(),
                    },
                    SessionSubcommand::Stop { ref session_id } => Cmd::Stop {
                        session_id: session_id.clone(),
                    },
                    SessionSubcommand::Archive {
                        ref session_id,
                        ref output_dir,
                        compress,
                    } => Cmd::Archive {
                        session_id: session_id.clone(),
                        output_dir: output_dir.clone(),
                        compress,
                    },
                    SessionSubcommand::RotateLogs {
                        ref session_id,
                        keep,
                    } => Cmd::RotateLogs {
                        session_id: session_id.clone(),
                        keep,
                    },
                    SessionSubcommand::Delete { ref session_id } => Cmd::Delete {
                        session_id: session_id.clone(),
                    },
                    SessionSubcommand::View { ref session_id } => Cmd::View {
                        session_id: session_id.clone(),
                    },
                    SessionSubcommand::List {
                        ref spec,
                        ref status,
                        ref runner,
                    } => Cmd::List {
                        spec: spec.clone(),
                        status: status.clone(),
                        runner: runner.clone(),
                    },
                    SessionSubcommand::Logs { ref session_id } => Cmd::Logs {
                        session_id: session_id.clone(),
                    },
                    SessionSubcommand::Worktrees { all } => Cmd::Worktrees { all },
                    SessionSubcommand::Merge {
                        ref session_id,
                        ref strategy,
                        resolve,
                    } => Cmd::Merge {
                        session_id: session_id.clone(),
                        strategy: strategy.clone(),
                        resolve,
                    },
                    SessionSubcommand::Cleanup {
                        ref session_id,
                        ref keep_branch,
                    } => Cmd::Cleanup {
                        session_id: session_id.clone(),
                        keep_branch: *keep_branch,
                    },
                    SessionSubcommand::Gc => Cmd::Gc,
                };
                commands::session::run(cmd)
            })()
        }
        Commands::Runner { action } => {
            use commands::runner::RunnerCommand as Cmd;
            let cmd = match *action {
                RunnerSubcommand::List { ref project_path } => Cmd::List {
                    project_path: project_path.clone(),
                },
                RunnerSubcommand::Show {
                    ref runner_id,
                    ref project_path,
                } => Cmd::Show {
                    runner_id: runner_id.clone(),
                    project_path: project_path.clone(),
                },
                RunnerSubcommand::Validate {
                    ref runner_id,
                    ref project_path,
                } => Cmd::Validate {
                    runner_id: runner_id.clone(),
                    project_path: project_path.clone(),
                },
                RunnerSubcommand::Config {
                    global,
                    ref project_path,
                } => Cmd::Config {
                    global,
                    project_path: project_path.clone(),
                },
            };
            commands::runner::run(cmd)
        }
        Commands::Skills { action } => match action {
            crate::cli_args::SkillSubcommand::Install { agent, yes } => {
                let agents = if agent.is_empty() {
                    None
                } else {
                    Some(&agent[..])
                };
                commands::skill::install(agents, yes)
            }
        },
        Commands::View(params) => {
            commands::view::run(&specs_dir, &params.spec, params.raw, &cli.output)
        }
    };

    match result {
        Ok(_) => ExitCode::SUCCESS,
        Err(e) => {
            if !cli.quiet {
                eprintln!("{} {}", "Error:".red().bold(), e);
            }
            ExitCode::FAILURE
        }
    }
}
