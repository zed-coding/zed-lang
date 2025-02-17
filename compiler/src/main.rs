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
use std::process;

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

fn compile(source: &str, filename: &str) -> Result<String> {
    let mut all_nodes = Vec::new();

    // Start with the main file
    let lexer = Lexer::new(&source, filename.to_string());
    let mut parser = Parser::new(lexer)?;
    let ast = parser.parse_program()?;

    // Add this file's AST nodes to our collection
    all_nodes.extend(ast);

    // Generate code from all collected nodes
    let mut generator = CodeGenerator::new();

    Ok(generator.generate(&all_nodes))
}
