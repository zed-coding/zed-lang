use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use colored::*;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use walkdir::WalkDir;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a new Zed project
    New {
        /// Name of the project
        name: String,
    },
    /// Build the current project
    Build {
        /// Enable release optimizations
        #[arg(long)]
        release: bool,
    },
    /// Run the current project
    Run {
        /// Enable release optimizations
        #[arg(long)]
        release: bool,
    },
    /// Clean the project
    Clean,
}

#[derive(Serialize, Deserialize)]
struct ZedConfig {
    name: String,
    version: String,
    target: String,
}

struct ZedProject {
    root: PathBuf,
    config: ZedConfig,
}

impl ZedProject {
    fn new(name: &str) -> Result<Self> {
        let root = PathBuf::from(name);
        let config = ZedConfig {
            name: name.to_string(),
            version: "0.1.0".to_string(),
            target: "main".to_string(),
        };
        Ok(Self { root, config })
    }

    fn create(&self) -> Result<()> {
        // Create project directories
        let dirs = ["src", "examples", "target"];
        for dir in dirs {
            fs::create_dir_all(self.root.join(dir))?;
        }

        // Create main.zed
        let main_content = r#"/* Main entry point for Zed program */

println("Hello from Zed!");
"#;
        fs::write(self.root.join("src").join("main.zed"), main_content)?;

        // Create zed.json config
        let config_content = serde_json::to_string_pretty(&self.config)?;
        fs::write(self.root.join("zed.json"), config_content)?;

        // Create .gitignore
        let gitignore_content = "target/\n*.o\n*.s\n";
        fs::write(self.root.join(".gitignore"), gitignore_content)?;

        println!("{} Project created successfully!", "Success:".green());
        Ok(())
    }

    fn load(path: &Path) -> Result<Self> {
        let config_path = path.join("zed.json");
        let config_content = fs::read_to_string(&config_path).context("Failed to read zed.json")?;
        let config: ZedConfig =
            serde_json::from_str(&config_content).context("Failed to parse zed.json")?;
        Ok(Self {
            root: path.to_path_buf(),
            config,
        })
    }

    fn build(&self, release: bool) -> Result<()> {
        let target_dir = self.root.join("target");
        let build_type = if release { "release" } else { "debug" };
        let build_dir = target_dir.join(build_type);
        fs::create_dir_all(&build_dir)?;

        // Find all .zed files
        let zed_files: Vec<_> = WalkDir::new(self.root.join("src"))
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.path().extension().map_or(false, |ext| ext == "zed"))
            .collect();

        for entry in zed_files {
            let source_path = entry.path();
            let asm_path = build_dir.join(
                source_path
                    .file_name()
                    .unwrap()
                    .to_string_lossy()
                    .replace(".zed", ".s"),
            );
            let obj_path = asm_path.with_extension("o");

            // Compile Zed to assembly
            println!("{} {}", "Compiling:".blue(), source_path.display());
            self.compile_to_asm(source_path, &asm_path)?;

            // Assemble
            println!("{} {}", "Assembling:".blue(), asm_path.display());
            self.assemble(&asm_path, &obj_path)?;
        }

        // Link
        let output_path = build_dir.join(&self.config.target);
        println!("{} {}", "Linking:".blue(), output_path.display());
        self.link(&build_dir, &output_path)?;

        println!("{} Build completed successfully!", "Success:".green());
        Ok(())
    }

    fn compile_to_asm(&self, source: &Path, output: &Path) -> Result<Output> {
        let output = Command::new("zedc")
            .arg(source)
            .arg("-o")
            .arg(output)
            .output()
            .context("Failed to execute zedc. Is it installed?")?;

        if !output.status.success() {
            anyhow::bail!(
                "Compilation failed:\n{}",
                String::from_utf8_lossy(&output.stderr)
                .red()
            );
        }
        Ok(output)
    }

    fn assemble(&self, source: &Path, output: &Path) -> Result<Output> {
        let output = Command::new("as")
            .arg(source)
            .arg("-o")
            .arg(output)
            .output()
            .context("Failed to execute as")?;

        if !output.status.success() {
            anyhow::bail!(
                "Assembly failed:\n{}",
                String::from_utf8_lossy(&output.stderr)
            );
        }
        Ok(output)
    }

    fn link(&self, obj_dir: &Path, output: &Path) -> Result<Output> {
        let obj_files: Vec<_> = fs::read_dir(obj_dir)?
            .filter_map(|e| e.ok())
            .filter(|e| e.path().extension().map_or(false, |ext| ext == "o"))
            .map(|e| e.path())
            .collect();

        let mut cmd = Command::new("ld");
        cmd.args(&obj_files).arg("-o").arg(output);

        let output = cmd.output().context("Failed to execute ld")?;

        if !output.status.success() {
            anyhow::bail!(
                "Linking failed:\n{}",
                String::from_utf8_lossy(&output.stderr)
            );
        }
        Ok(output)
    }

    fn run(&self, release: bool) -> Result<()> {
        self.build(release)?;

        let target_dir = self.root.join("target");
        let build_type = if release { "release" } else { "debug" };
        let executable = target_dir.join(build_type).join(&self.config.target);

        println!("{} {}", "Running:".blue(), executable.display());

        let output = Command::new(executable)
            .output()
            .context("Failed to execute program")?;

        println!("{}", String::from_utf8_lossy(&output.stdout));

        if !output.status.success() {
            anyhow::bail!(
                "Program exited with error:\n{}",
                String::from_utf8_lossy(&output.stderr)
            );
        }
        Ok(())
    }

    fn clean(&self) -> Result<()> {
        let target_dir = self.root.join("target");
        if target_dir.exists() {
            fs::remove_dir_all(target_dir)?;
            println!("{} Build artifacts cleaned", "Success:".green());
        }
        Ok(())
    }
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::New { name } => {
            let project = ZedProject::new(&name)?;
            project.create()?;
        }
        Commands::Build { release } => {
            let project = ZedProject::load(&std::env::current_dir()?)?;
            project.build(release)?;
        }
        Commands::Run { release } => {
            let project = ZedProject::load(&std::env::current_dir()?)?;
            project.run(release)?;
        }
        Commands::Clean => {
            let project = ZedProject::load(&std::env::current_dir()?)?;
            project.clean()?;
        }
    }

    Ok(())
}
