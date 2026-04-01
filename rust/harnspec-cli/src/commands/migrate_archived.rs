//! Migrate archived specs command implementation
//!
//! Migrates specs from legacy archived/ folder to status-based archiving.

use colored::Colorize;
use harnspec_core::SpecArchiver;
use std::error::Error;
use std::path::Path;

pub fn run(specs_dir: &str, dry_run: bool) -> Result<(), Box<dyn Error>> {
    let specs_path = Path::new(specs_dir);
    let archived_dir = specs_path.join("archived");

    if !archived_dir.exists() {
        println!(
            "{} No archived/ folder found - nothing to migrate",
            "✓".green()
        );
        return Ok(());
    }

    // Count specs in archived folder
    let archived_count = std::fs::read_dir(&archived_dir)?
        .filter_map(|e| e.ok())
        .filter(|e| e.path().is_dir())
        .count();

    if archived_count == 0 {
        println!(
            "{} archived/ folder is empty - nothing to migrate",
            "✓".green()
        );
        // Try to remove empty directory
        let _ = std::fs::remove_dir(&archived_dir);
        return Ok(());
    }

    println!();
    println!("{}", "═".repeat(60));
    println!("{}", "📦 Migrate Archived Specs".cyan().bold());
    println!("{}", "═".repeat(60));
    println!();
    println!(
        "Found {} spec(s) in archived/ folder to migrate.",
        archived_count.to_string().cyan()
    );
    println!();
    println!("This will:");
    println!(
        "  • Move specs from {} to {}",
        "specs/archived/".cyan(),
        "specs/".green()
    );
    println!(
        "  • Set {} on all migrated specs",
        "status: archived".green()
    );
    println!("  • Remove empty archived/ folder");
    println!();

    if dry_run {
        println!("{}", "⚠️  DRY RUN - No changes will be made".yellow());
        println!();

        // List specs that would be migrated
        for entry in std::fs::read_dir(&archived_dir)? {
            let entry = entry?;
            if entry.path().is_dir() {
                let name = entry.file_name().to_string_lossy().to_string();
                println!("  {} archived/{} → {}", "→".cyan(), name, name);
            }
        }
        println!();
        return Ok(());
    }

    let archiver = SpecArchiver::new(specs_dir);

    match archiver.migrate_archived() {
        Ok(migrated) => {
            println!();
            for name in &migrated {
                println!("{} Migrated: {}", "✓".green(), name.cyan());
            }
            println!();
            println!("{}", "═".repeat(60));
            println!(
                "{} Successfully migrated {} spec(s) to status-based archiving",
                "✓".green(),
                migrated.len()
            );
            println!("{}", "═".repeat(60));
            println!();
            println!(
                "Specs now use {} for archiving - no folder moves needed.",
                "status: archived".green()
            );
            Ok(())
        }
        Err(e) => Err(format!("Migration failed: {}", e).into()),
    }
}
