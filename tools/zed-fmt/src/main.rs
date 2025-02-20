use anyhow::{Context, Result};
use clap::Parser;
use colored::*;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

mod formatter;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Files or directories to format
    #[arg(required = true)]
    paths: Vec<String>,

    /// Write changes to files (instead of printing diffs)
    #[arg(short, long)]
    write: bool,

    /// Check if files are formatted without making changes
    #[arg(short, long)]
    check: bool,

    /// Number of spaces per indentation level
    #[arg(short, long, default_value = "4")]
    indent: usize,

    /// Maximum line length
    #[arg(long, default_value = "100")]
    max_width: usize,
}

fn format_file(path: &Path, config: &formatter::Config, check_only: bool, write: bool) -> Result<bool> {
    let source = fs::read_to_string(path)
        .with_context(|| format!("Failed to read {}", path.display()))?;

    let formatted = formatter::format_source(&source, config)?;

    if source != formatted {
        if check_only {
            println!("{} {}", "Needs formatting:".red(), path.display());
            return Ok(false);
        }

        if write {
            fs::write(path, formatted)
                .with_context(|| format!("Failed to write to {}", path.display()))?;
            println!("{} {}", "Formatted:".green(), path.display());
        } else {
            // Print diff
            println!("{} {}", "Would format:".yellow(), path.display());
            // TODO: Implement diff display
        }
    } else if !check_only {
        println!("{} {}", "Already formatted:".green(), path.display());
    }

    Ok(true)
}

fn process_paths(paths: &[String], config: &formatter::Config, check: bool, write: bool) -> Result<bool> {
    let mut all_formatted = true;

    for path_str in paths {
        let path = PathBuf::from(path_str);
        if path.is_file() {
            if !format_file(&path, config, check, write)? {
                all_formatted = false;
            }
        } else if path.is_dir() {
            for entry in WalkDir::new(&path)
                .into_iter()
                .filter_map(|e| e.ok())
                .filter(|e| e.path().extension().map_or(false, |ext| ext == "zed"))
            {
                if !format_file(entry.path(), config, check, write)? {
                    all_formatted = false;
                }
            }
        } else {
            eprintln!("{} Path does not exist: {}", "Error:".red(), path_str);
            all_formatted = false;
        }
    }

    Ok(all_formatted)
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    if cli.check && cli.write {
        anyhow::bail!("Cannot use both --check and --write");
    }

    let config = formatter::Config {
        indent_spaces: cli.indent,
        max_width: cli.max_width,
    };

    let all_formatted = process_paths(&cli.paths, &config, cli.check, cli.write)?;

    if cli.check && !all_formatted {
        std::process::exit(1);
    }

    Ok(())
}
