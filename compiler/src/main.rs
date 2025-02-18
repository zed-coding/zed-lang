mod ast;
mod codegen;
mod colors;
mod lexer;
mod parser;

use codegen::CodeGenerator;
use lexer::{Lexer, Result};
use parser::Parser;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process;

fn is_main_file(filepath: &str) -> bool {
    Path::new(filepath)
        .file_name()
        .and_then(|name| name.to_str())
        .map_or(false, |name| name == "main.zed")
}

fn compile(source: &str, filename: &str, stdlib_path: Option<PathBuf>) -> Result<String> {
    let lexer = Lexer::new(source, filename.to_string());
    let mut parser = Parser::new(lexer, Path::new(filename))?;

    if let Some(path) = stdlib_path {
        parser.set_stdlib_path(path);
    }

    let ast = parser.parse_program()?;
    let mut generator = CodeGenerator::new(is_main_file(filename));
    Ok(generator.generate(&ast))
}

fn print_usage(program: &str) {
    eprintln!(
        "Usage: {} <input.zed> -o <output.asm> [--stdlib-path <path>]",
        program
    );
    process::exit(1);
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 4 {
        print_usage(&args[0]);
    }

    let mut input_path = None;
    let mut output_path = None;
    let mut stdlib_path = None;
    let mut i = 1;

    while i < args.len() {
        match args[i].as_str() {
            "-o" => {
                if i + 1 < args.len() {
                    output_path = Some(args[i + 1].clone());
                    i += 2;
                } else {
                    eprintln!("error: -o requires an output path");
                    process::exit(1);
                }
            }
            "--stdlib-path" => {
                if i + 1 < args.len() {
                    stdlib_path = Some(PathBuf::from(&args[i + 1]));
                    i += 2;
                } else {
                    eprintln!("error: --stdlib-path requires a path");
                    process::exit(1);
                }
            }
            _ => {
                if input_path.is_none() {
                    input_path = Some(args[i].clone());
                    i += 1;
                } else {
                    print_usage(&args[0]);
                }
            }
        }
    }

    let input_path = input_path.unwrap_or_else(|| {
        eprintln!("error: no input file specified");
        process::exit(1);
    });

    let output_path = output_path.unwrap_or_else(|| {
        eprintln!("error: no output file specified (-o flag missing)");
        process::exit(1);
    });

    // Check stdlib path
    if stdlib_path.is_none() {
        stdlib_path = Parser::get_default_stdlib_path().ok();
    }

    if let Some(path) = &stdlib_path {
        if !path.exists() {
            eprintln!("error: Standard library not found at {}", path.display());
            eprintln!(
                "Please install the standard library or specify correct path with --stdlib-path"
            );
            process::exit(1);
        }
    }

    // Read input file
    let source = match fs::read_to_string(&input_path) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("error: couldn't read {}: {}", input_path, e);
            process::exit(1);
        }
    };

    // Compile
    match compile(&source, &input_path, stdlib_path) {
        Ok(assembly) => {
            if let Err(e) = fs::write(&output_path, assembly) {
                eprintln!("error: couldn't write to {}: {}", output_path, e);
                process::exit(1);
            }
        }
        Err(e) => {
            eprintln!("{}", e.format_error());
            process::exit(1);
        }
    }

    Ok(())
}
