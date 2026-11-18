pub mod compiler;
pub mod frontend;
pub mod runtime;
pub mod tests;

use frontend::{execute_file, Lexer, Parser};
use std::env;
use std::fs;
use std::path::Path;

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
        "lsp" => {
            if let Err(e) = compiler::LspServer::run() {
                eprintln!("LSP server error: {}", e);
            }
        }
        "emit-llvm" => {
            if args.len() < 3 {
                eprintln!("Error: missing source file.");
                return;
            }
            emit_llvm(&args[2]);
        }
        "emit-c" => {
            if args.len() < 3 {
                eprintln!("Error: missing source file.");
                return;
            }
            emit_c(&args[2]);
        }
        "test" => {
            println!("=== Running Elaichi Engine Verification Suite ===");
            match tests::run_all_self_tests() {
                Ok(_) => println!("All tests completed successfully."),
                Err(err) => eprintln!("Test failure: {}", err),
            }
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
    println!("  elaichi lsp                 Start Language Server Protocol (LSP) daemon");
    println!("  elaichi emit-llvm <file.el> Compile program to SSA LLVM IR");
    println!("  elaichi emit-c <file.el>    Compile program to optimized SSA C99 code");
    println!("  elaichi test                Execute built-in engine verification suite");
    println!("  elaichi dump-ast <file.el>  Parse and dump AST bifurcation classification");
    println!("  elaichi --version           Print version");
}

fn run_file(path: &str) {
    let p = Path::new(path);
    if let Err(err) = execute_file(p) {
        eprintln!("Error: {}", err);
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

fn emit_llvm(path: &str) {
    let source = match fs::read_to_string(path) {
        Ok(s) => s,
        Err(err) => {
            eprintln!("Error reading file: {}", err);
            return;
        }
    };

    let mut lexer = Lexer::new(&source);
    let tokens = match lexer.tokenize() {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Lexer error: {}", e);
            return;
        }
    };
    let mut parser = Parser::new(tokens);
    let program = match parser.parse_program() {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Parser error: {}", e);
            return;
        }
    };

    let env = compiler::TypeEnv::new();
    let mut module = compiler::MirModule::default();

    for func in &program.functions {
        match compiler::compile_function_to_mir(func, &env) {
            Ok(mir_func) => module.functions.push(mir_func),
            Err(e) => {
                eprintln!("MIR compilation error for {}: {}", func.name, e);
                return;
            }
        }
    }

    compiler::MirOptimizer::optimize_module(&mut module);
    let llvm_ir = compiler::LlvmIrEmitter::emit_module(&module);
    println!("{}", llvm_ir);
}

fn emit_c(path: &str) {
    let source = match fs::read_to_string(path) {
        Ok(s) => s,
        Err(err) => {
            eprintln!("Error reading file: {}", err);
            return;
        }
    };

    let mut lexer = Lexer::new(&source);
    let tokens = match lexer.tokenize() {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Lexer error: {}", e);
            return;
        }
    };
    let mut parser = Parser::new(tokens);
    let program = match parser.parse_program() {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Parser error: {}", e);
            return;
        }
    };

    let env = compiler::TypeEnv::new();
    let mut module = compiler::MirModule::default();

    for func in &program.functions {
        match compiler::compile_function_to_mir(func, &env) {
            Ok(mir_func) => module.functions.push(mir_func),
            Err(e) => {
                eprintln!("MIR compilation error for {}: {}", func.name, e);
                return;
            }
        }
    }

    // Apply full SSA optimization pipeline (constant folding, DCE, escape analysis)
    compiler::MirOptimizer::optimize_module(&mut module);

    // Emit target C99 code
    let c_code = compiler::MirCTranspiler::compile_module_to_c(&module);
    println!("{}", c_code);
}
