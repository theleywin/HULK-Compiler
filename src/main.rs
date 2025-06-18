//! Main entry point for the Hulk compiler.
//!
//! This module coordinates the phases of compilation: parsing, semantic analysis,
//! code generation and final compilation via `clang`.
//!
//! It also includes a fun ASCII logo banner.

use crate::{parser_w_errors::Parser, semantic_analyzer::semantic_analyzer::SemanticAnalyzer};
use lalrpop_util::lalrpop_mod;

pub mod ast_nodes;
pub mod builtin;
pub mod codegen;
mod parser_w_errors;
pub mod semantic_analyzer;
mod tokens;
pub mod types_tree;
pub mod visitor;

// Load the LALRPOP parser module
lalrpop_mod!(pub parser);

use crate::builtin::FunctionInjector;
use codegen::CodeGenerator;
use std::path::Path;

/// Entry point of the Hulk compiler.
///
/// This function:
/// 1. Reads the input source file provided as a command-line argument.
/// 2. Injects built-in functions.
/// 3. Parses the code into an AST.
/// 4. Performs semantic analysis on the AST.
/// 5. Generates LLVM IR from the AST.
/// 6. Uses `clang` to compile the IR into a native executable.
fn main() {
    // Show ASCII art logo in green
    println!("\x1b[32m");
    println!(r#"
██╗  ██╗██╗   ██╗██╗     ██╗  ██╗
██║  ██║██║   ██║██║     ██║ ██╔╝
███████║██║   ██║██║     █████╔╝ 
██╔══██║██║   ██║██║     ██╔═██╗ 
██║  ██║╚██████╔╝███████╗██║  ██╗
╚═╝  ╚═╝ ╚═════╝ ╚══════╝╚═╝  ╚═╝
"#);
    println!("\x1b[0m"); // Reset color

    // Validate command-line arguments
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <input_file>", args[0]);
        std::process::exit(1);
    }

    // Read input source file
    let filename = &args[1];
    let raw_input = std::fs::read_to_string(filename)
        .expect(&format!("Failed to read input file: {}", filename));

    // Inject built-in functions
    let function_injector = FunctionInjector::new();
    let input = function_injector.inject_code(&raw_input);
    let missplacement = function_injector.get_builtin_functions_code_lines() as i32;

    // Parse input into AST
    let parser = Parser::new(missplacement);
    match parser.parse(&input) {
        Ok(mut expr) => {
            // Perform semantic analysis
            let mut semantic_analyzer = SemanticAnalyzer::new();
            let result = semantic_analyzer.analyze(&mut expr);
            match result {
                Ok(_) => {
                    // Continue to code generation
                }
                Err(errors) => {
                    println!("\x1b[31mSemantic Errors:");
                    for err in errors.iter() {
                        println!("{}", err.report(&input, missplacement));
                    }
                    println!("\x1b[0m");
                    std::process::exit(3);
                }
            }

            // Generate LLVM IR
            let mut codegen = CodeGenerator::new();
            let llvm_ir = codegen.generate(&mut expr);

            // Ensure output directory exists
            if !Path::new("hulk").exists() {
                std::fs::create_dir("hulk").expect("Failed to create hulk directory");
            }

            // Write IR to file
            let ir_path = "hulk/output.ll";
            std::fs::write(ir_path, &llvm_ir).expect("Failed to write LLVM IR");

            // Define output executable path depending on OS
            let executable = if cfg!(windows) {
                "hulk/output.exe"
            } else {
                "hulk/output"
            };

            // Compile using clang and link runtime
            let status = std::process::Command::new("clang")
                .args(&[ir_path, "runtime.c", "-o", executable, "-lm"]) 
                .status()
                .expect("Failed to compile with clang");

            if !status.success() {
                eprintln!("\x1b[31mCompilation failed\x1b[0m");
            }
        }
        Err(errors) => {
            // Report syntax errors
            println!("\x1b[31mSyntax Errors:");
            for err in errors.iter() {
                println!("{}", err);
            }   
            println!("\x1b[0m");
            std::process::exit(2);
        }
    }
}
