mod ast;
mod codegen;
mod lexer;
mod parser;
mod colors;

use codegen::CodeGenerator;
use lexer::{Lexer, Result};
use parser::Parser;
use std::env;
use std::fs;
use std::path::Path;
use std::process;

fn is_main_file(filepath: &str) -> bool {
    Path::new(filepath)
        .file_name()
        .and_then(|name| name.to_str())
        .map_or(false, |name| name == "main.zed")
}

fn compile(source: &str, filename: &str) -> Result<String> {
    let lexer = Lexer::new(source, filename.to_string());
    let mut parser = Parser::new(lexer, Path::new(filename))?;
    let ast = parser.parse_program()?;

    let mut generator = CodeGenerator::new(is_main_file(filename));
    Ok(generator.generate(&ast))
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 || args[2] != "-o" {
        eprintln!("Usage: {} <input.zed> -o <output.asm>", args[0]);
        process::exit(1);
    }

    let input_path = &args[1];
    let output_path = &args[3];

    // Read input file
    let source = match fs::read_to_string(input_path) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("error: couldn't read {}: {}", input_path, e);
            process::exit(1);
        }
    };

    // Compile
    match compile(&source, input_path) {
        Ok(assembly) => {
            if let Err(e) = fs::write(output_path, assembly) {
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
