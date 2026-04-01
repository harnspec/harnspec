//! Stats command implementation

use colored::Colorize;
use harnspec_core::{Insights, SpecLoader, SpecPriority, SpecStats, SpecStatus};
use std::error::Error;

pub fn run(specs_dir: &str, detailed: bool, output_format: &str) -> Result<(), Box<dyn Error>> {
    let loader = SpecLoader::new(specs_dir);
    let specs = loader.load_all()?;

    let stats = SpecStats::compute(&specs);
    let insights = Insights::generate(&specs, &stats);

    if output_format == "json" {
        #[derive(serde::Serialize)]
        struct StatsOutput {
            total: usize,
            by_status: std::collections::HashMap<String, usize>,
            by_priority: std::collections::HashMap<String, usize>,
            completion_percentage: f64,
            active_count: usize,
            with_dependencies: usize,
            sub_specs: usize,
            insights: Vec<InsightOutput>,
        }

        #[derive(serde::Serialize)]
        struct InsightOutput {
            severity: String,
            message: String,
            related_specs: Vec<String>,
        }

        let output = StatsOutput {
            total: stats.total,
            by_status: stats
                .by_status
                .iter()
                .map(|(k, v)| (k.to_string(), *v))
                .collect(),
            by_priority: stats
                .by_priority
                .iter()
                .map(|(k, v)| (k.to_string(), *v))
                .collect(),
            completion_percentage: stats.completion_percentage(),
            active_count: stats.active_count(),
            with_dependencies: stats.with_dependencies,
            sub_specs: stats.sub_specs,
            insights: insights
                .messages
                .iter()
                .map(|i| InsightOutput {
                    severity: format!("{:?}", i.severity),
                    message: i.message.clone(),
                    related_specs: i.related_specs.clone(),
                })
                .collect(),
        };

        println!("{}", serde_json::to_string_pretty(&output)?);
        return Ok(());
    }

    // Text output
    println!();
    println!("{}", "═".repeat(60).dimmed());
    println!("{}", " SPEC STATISTICS ".bold().cyan());
    println!("{}", "═".repeat(60).dimmed());
    println!();

    // Summary
    println!(
        "{} {} specs",
        "📊".bold(),
        stats.total.to_string().green().bold()
    );
    println!();

    // By status
    println!("{}", "By Status".bold());
    println!("{}", "─".repeat(30).dimmed());
    let status_order = [
        (SpecStatus::InProgress, "⏳"),
        (SpecStatus::Planned, "📅"),
        (SpecStatus::Complete, "✅"),
        (SpecStatus::Archived, "📦"),
    ];
    for (status, emoji) in status_order {
        let count = stats.by_status.get(&status).copied().unwrap_or(0);
        if count > 0 || detailed {
            let bar_len = (count as f64 / stats.total as f64 * 30.0) as usize;
            let bar = "█".repeat(bar_len);
            println!(
                "  {} {:12} {:>4} {}",
                emoji,
                format!("{:?}", status),
                count,
                bar.cyan()
            );
        }
    }
    println!();

    // Completion
    let completion = stats.completion_percentage();
    let completion_bar_len = (completion / 100.0 * 30.0) as usize;
    let completion_bar = "█".repeat(completion_bar_len);
    let completion_empty = "░".repeat(30 - completion_bar_len);
    println!(
        "{} {:.1}% {}{}",
        "Progress:".bold(),
        completion,
        completion_bar.green(),
        completion_empty.dimmed()
    );
    println!();

    // By priority
    println!("{}", "By Priority".bold());
    println!("{}", "─".repeat(30).dimmed());
    let priority_order = [
        (Some(SpecPriority::Critical), "🔴"),
        (Some(SpecPriority::High), "🟠"),
        (Some(SpecPriority::Medium), "🟡"),
        (Some(SpecPriority::Low), "🟢"),
    ];
    for (priority, emoji) in priority_order {
        if let Some(p) = priority {
            let count = stats.by_priority.get(&p).copied().unwrap_or(0);
            if count > 0 || detailed {
                println!("  {} {:12} {:>4}", emoji, format!("{:?}", p), count);
            }
        }
    }
    if stats.no_priority > 0 {
        println!("  ⚪ {:12} {:>4}", "None", stats.no_priority);
    }
    println!();

    // Top tags
    if !stats.by_tag.is_empty() {
        println!("{}", "Top Tags".bold());
        println!("{}", "─".repeat(30).dimmed());
        for (tag, count) in stats.top_tags(10) {
            println!("  🏷️  {:20} {:>4}", tag, count);
        }
        println!();
    }

    // Dependencies
    if detailed {
        println!("{}", "Dependencies".bold());
        println!("{}", "─".repeat(30).dimmed());
        println!("  Specs with dependencies: {}", stats.with_dependencies);
        println!("  Total dependency links:  {}", stats.total_dependencies);
        println!("  Sub-specs:               {}", stats.sub_specs);
        println!();
    }

    // Insights
    if !insights.messages.is_empty() {
        println!("{}", "═".repeat(60).dimmed());
        println!("{}", " INSIGHTS ".bold().yellow());
        println!("{}", "═".repeat(60).dimmed());
        println!();

        for insight in &insights.messages {
            println!("  {} {}", insight.severity, insight.message);

            if detailed && !insight.related_specs.is_empty() {
                for spec in insight.related_specs.iter().take(5) {
                    println!("      └─ {}", spec.dimmed());
                }
                if insight.related_specs.len() > 5 {
                    println!("      └─ ... and {} more", insight.related_specs.len() - 5);
                }
            }
        }
        println!();
    }

    Ok(())
}
