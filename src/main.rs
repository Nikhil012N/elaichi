pub mod ast;
pub mod bifurcator;
pub mod bridge;
pub mod lexer;
pub mod parser;
pub mod runtime;
pub mod static_engine;
pub mod vm;

#[cfg(test)]
mod tests;

use bifurcator::Bifurcator;
use lexer::Lexer;
use parser::Parser;
use runtime::Runtime;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_usage();
        return;
    }

    match args[1].as_str() {
        "run" => {
            if args.len() < 3 {
                eprintln!("Error: missing source file to run.");
                return;
            }
            let filename = &args[2];
            run_file(filename);
        }
        "dump-ast" => {
            if args.len() < 3 {
                eprintln!("Error: missing source file.");
                return;
            }
            dump_ast(&args[2]);
        }
        "version" | "--version" | "-v" => {
            println!("Elaichi v0.1.0-dev (Dual-Stack Engine)");
        }
        _ => {
            print_usage();
        }
    }
}

fn print_usage() {
    println!("Elaichi Programming Language CLI");
    println!("Usage:");
    println!("  elaichi run <file.el>       Execute program on the dual-stack runtime");
    println!("  elaichi dump-ast <file.el>  Parse and dump AST bifurcation classification");
    println!("  elaichi --version           Print version");
}

fn run_file(path: &str) {
    let source = match fs::read_to_string(path) {
        Ok(s) => s,
        Err(err) => {
            eprintln!("Error reading file '{}': {}", path, err);
            return;
        }
    };

    let mut lexer = Lexer::new(&source);
    let tokens = match lexer.tokenize() {
        Ok(toks) => toks,
        Err(err) => {
            eprintln!("Lexer error: {}", err);
            return;
        }
    };

    let mut parser = Parser::new(tokens);
    let program = match parser.parse_program() {
        Ok(p) => p,
        Err(err) => {
            eprintln!("Parser error: {}", err);
            return;
        }
    };

    let mut bifurcator = Bifurcator::new();
    if let Err(err) = bifurcator.compile_program(program) {
        eprintln!("Bifurcation error: {}", err);
        return;
    }

    let mut runtime = Runtime::new(bifurcator);
    if let Err(err) = runtime.execute("main") {
        eprintln!("Runtime execution error: {}", err);
    }
}

fn dump_ast(path: &str) {
    let source = match fs::read_to_string(path) {
        Ok(s) => s,
        Err(err) => {
            eprintln!("Error reading file: {}", err);
            return;
        }
    };

    let mut lexer = Lexer::new(&source);
    let tokens = lexer.tokenize().expect("Lexer error");
    let mut parser = Parser::new(tokens);
    let program = parser.parse_program().expect("Parser error");

    println!("=== Elaichi AST Bifurcation ===");
    for func in &program.functions {
        let kind = if func.is_static {
            "[STATIC STACK]"
        } else {
            "[DYNAMIC STACK]"
        };
        println!("{} fn {} (params: {:?}) -> {:?}", kind, func.name, func.params, func.return_ty);
    }
}
