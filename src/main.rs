mod lexer;
mod ast;
mod parser;
mod evaluator;

use lexer::tokenize;
use parser::Parser;
use evaluator::Interpreter;
use std::env;
use std::fs;
use std::process;

fn main() {
    // Collect command-line arguments
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("MiniLang v1.0");
        eprintln!("Usage: minilang <file.ml>");
        eprintln!("Example: minilang hello.ml");
        process::exit(1);
    }

    let filename = &args[1];

    // Read the source file
    let source = match fs::read_to_string(filename) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error: Could not read file '{}': {}", filename, e);
            process::exit(1);
        }
    };

    // Run the pipeline
    let tokens = tokenize(&source);

    let mut parser = Parser::new(tokens);
    let ast = parser.parse_program();

    let mut interpreter = Interpreter::new();
    interpreter.run(&ast);
}