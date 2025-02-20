use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use colored::*;
use flate2::read::GzDecoder;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};
use tar::Archive;
use tempfile::TempDir;

const REGISTRY_URL: &str = "https://zed-pkg.vercel.app/api/packages";

#[derive(Parser)]
#[command(name = "zed-pkg")]
#[command(about = "Zed Package Manager")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Install a package
    Install {
        /// Package name
        package: String,

        /// Optional specific version
        #[arg(short, long)]
        version: Option<String>,
    },
    /// List installed packages
    List,
    /// Remove a package
    Remove {
        /// Package name
        package: String,
    },
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct PackageMetadata {
    name: String,
    version: String,
    description: Option<String>,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    // Ensure pkg directory exists in src/
    let pkg_dir = std::env::current_dir()?.join("src/pkg");
    fs::create_dir_all(&pkg_dir)?;

    match cli.command {
        Commands::Install { package, version } => install_package(&package, version)?,
        Commands::List => list_packages()?,
        Commands::Remove { package } => remove_package(&package)?,
    }

    Ok(())
}

fn install_package(package: &str, version: Option<String>) -> Result<()> {
    let client = Client::new();

    // Construct URL for package metadata
    let url = match version {
        Some(v) => format!("{}/{}/{}", REGISTRY_URL, package, v),
        None => format!("{}/{}", REGISTRY_URL, package),
    };

    // Fetch package metadata
    let response = client.get(&url)
        .send()
        .context("Failed to fetch package metadata")?;

    if !response.status().is_success() {
        println!("{} Package not found", "✗".red());
        return Ok(());
    }

    let package_info: PackageMetadata = response.json()?;

    // Prepare pkg directory
    let pkg_dir = std::env::current_dir()?.join("src/pkg");
    let package_file = pkg_dir.join(format!("{}.zed", package_info.name));

    // Create temporary directory for download
    let temp_dir = TempDir::new()?;
    let tarball_path = temp_dir.path().join(format!("{}.tar.gz", package));

    // Download package tarball
    let mut file = File::create(&tarball_path)?;
    let download_url = format!("{}/download", url);
    let content = client.get(&download_url)
        .send()?
        .text()?;
    file.write_all(content.as_bytes())?;

    // Remove existing package file if it exists
    if package_file.exists() {
        fs::remove_file(&package_file)?;
    }

    // Extract tarball and find .zed file
    let tar_gz = File::open(&tarball_path)?;
    let tar = GzDecoder::new(tar_gz);
    let mut archive = Archive::new(tar);

    // Extract to a temporary directory
    let extract_dir = temp_dir.path().join("extracted");
    fs::create_dir_all(&extract_dir)?;
    archive.unpack(&extract_dir)?;

    // Find .zed file in extracted contents
    let zed_file = find_zed_file(&extract_dir)?;

    // Copy .zed file to src/pkg
    fs::copy(zed_file, &package_file)?;

    println!(
        "{} Installed {} v{} at {}",
        "✓".green(),
        package_info.name.bright_blue(),
        package_info.version.bright_blue(),
        package_file.display()
    );

    // Create a metadata file to track the package
    let metadata_file = pkg_dir.join(format!("{}.json", package_info.name));
    let metadata = serde_json::json!({
        "name": package_info.name,
        "version": package_info.version,
        "description": package_info.description,
        "installed_at": chrono::Utc::now().to_rfc3339()
    });

    fs::write(
        metadata_file,
        serde_json::to_string_pretty(&metadata)?
    )?;

    Ok(())
}

fn find_zed_file(dir: &Path) -> Result<PathBuf> {
    for entry in walkdir::WalkDir::new(dir) {
        let entry = entry?;
        if entry.file_type().is_file() {
            let path = entry.path();
            if path.extension().map_or(false, |ext| ext == "zed") {
                return Ok(path.to_path_buf());
            }
        }
    }

    anyhow::bail!("No .zed file found in package")
}

fn list_packages() -> Result<()> {
    let pkg_dir = std::env::current_dir()?.join("src/pkg");

    if !pkg_dir.exists() {
        println!("No packages installed. src/pkg/ directory is empty.");
        return Ok(());
    }

    let mut package_found = false;
    for entry in fs::read_dir(&pkg_dir)? {
        let entry = entry?;
        let path = entry.path();

        // Look for .json metadata files
        if path.extension().map_or(false, |ext| ext == "json") {
            package_found = true;

            // Read metadata
            let metadata_content = fs::read_to_string(&path)?;
            let metadata: serde_json::Value = serde_json::from_str(&metadata_content)?;

            println!(
                "{} v{}",
                metadata["name"].as_str().unwrap_or("Unknown").bright_green(),
                metadata["version"].as_str().unwrap_or("0.0.0").bright_blue()
            );

            if let Some(desc) = metadata["description"].as_str() {
                println!("  {}", desc.dimmed());
            }
        }
    }

    if !package_found {
        println!("No packages found in src/pkg/ directory.");
    }

    Ok(())
}

fn remove_package(package: &str) -> Result<()> {
    let pkg_dir = std::env::current_dir()?.join("src/pkg");
    let zed_file = pkg_dir.join(format!("{}.zed", package));
    let json_file = pkg_dir.join(format!("{}.json", package));

    if !zed_file.exists() {
        println!("{} Package {} not found in src/pkg/", "✗".red(), package);
        return Ok(());
    }

    // Remove .zed and .json files
    fs::remove_file(&zed_file)?;
    if json_file.exists() {
        fs::remove_file(&json_file)?;
    }

    println!("{} Removed package {} from src/pkg/", "✓".green(), package.bright_blue());

    Ok(())
}
