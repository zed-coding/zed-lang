use anyhow::{Context, Result};
use clap::Parser;
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

mod parser;
mod generator;
mod templates;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Input file or directory
    input: String,

    /// Output directory
    #[arg(short, long)]
    output: String,

    /// Optional title for the documentation
    #[arg(short, long)]
    title: Option<String>,

    /// Include private functions in documentation
    #[arg(long)]
    private: bool,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let input_path = Path::new(&cli.input);
    let output_path = Path::new(&cli.output);

    // Create output directory if it doesn't exist
    fs::create_dir_all(output_path)
        .context("Failed to create output directory")?;

    // Process input
    if input_path.is_file() {
        process_file(input_path, output_path, &cli)?;
    } else if input_path.is_dir() {
        process_directory(input_path, output_path, &cli)?;
    } else {
        anyhow::bail!("Input path does not exist: {}", cli.input);
    }

    // Always generate index
    generator::generate_index(output_path, cli.title.as_deref())?;

    println!("Documentation generated successfully in {}", cli.output);
    Ok(())
}

fn process_file(input: &Path, output_dir: &Path, cli: &Cli) -> Result<()> {
    println!("Processing file: {}", input.display());

    let source = fs::read_to_string(input)
        .context("Failed to read input file")?;

    let doc = parser::parse_source(&source, cli.private)?;
    let html = generator::generate_html(&doc, cli.title.as_deref())?;

    let output_file = output_dir.join(
        input.file_name()
            .unwrap()
            .to_string_lossy()
            .replace(".zed", ".html")
    );

    fs::write(&output_file, html)
        .context("Failed to write output file")?;

    Ok(())
}

fn process_directory(input_dir: &Path, output_dir: &Path, cli: &Cli) -> Result<()> {
    let mut files = Vec::new();

    // Collect all .zed files
    for entry in WalkDir::new(input_dir) {
        let entry = entry?;
        if entry.path().extension().map_or(false, |ext| ext == "zed") {
            files.push(entry.path().to_owned());
        }
    }

    // Process each file
    for file in files {
        let relative_path = file.strip_prefix(input_dir)?;
        let output_path = output_dir.join(relative_path);

        // Create parent directories if needed
        if let Some(parent) = output_path.parent() {
            fs::create_dir_all(parent)?;
        }

        process_file(&file, output_dir, cli)?;
    }

    Ok(())
}
