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

lalrpop_mod!(pub parser);

use crate::builtin::FunctionInjector;
use codegen::CodeGenerator;
use std::path::Path;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <input_file>", args[0]);
        std::process::exit(1);
    }

    let filename = &args[1];
    let raw_input = std::fs::read_to_string(filename)
        .expect(&format!("Failed to read input file: {}", filename));

    let function_injector = FunctionInjector::new();
    let input = function_injector.inject_code(&raw_input);
    let missplacement = function_injector.get_builtin_functions_code_lines() as i32;

    let parser = Parser::new(missplacement);
    match parser.parse(&input) {
        Ok(mut expr) => {
            println!("Syntactic Analyzer OK");
            let mut semantic_analyzer = SemanticAnalyzer::new();
            let result = semantic_analyzer.analyze(&mut expr);
            match result {
                Ok(_) => {
                    println!("Semantic Analyzer OK");
                }
                Err(errors) => {
                    println!("\x1b[31mSemantic Errors:");
                    for err in errors.iter() {
                        println!("{}", err.message());
                    }
                    println!("\x1b[0m");
                }
            }

            println!("\nGenerating LLVM IR...");
            let mut codegen = CodeGenerator::new();
            let llvm_ir = codegen.generate(&mut expr);

            println!("\x1b[32mGenerated LLVM IR:\n{}\x1b[0m", llvm_ir);

            // Create hulk directory if it doesn't exist
            if !Path::new("hulk").exists() {
                std::fs::create_dir("hulk").expect("Failed to create hulk directory");
            }
            
            // Write IR to hulk/output.ll
            let ir_path = "hulk/output.ll";
            std::fs::write(ir_path, &llvm_ir).expect("Failed to write LLVM IR");
            println!("LLVM IR written to {}", ir_path);

            println!("\nCompiling to native executable...");
            let executable = if cfg!(windows) {
                "hulk/output.exe"
            } else {
                "hulk/output"
            };
            
            let status = std::process::Command::new("clang")
                .args(&[ir_path, "runtime.c", "-o", executable])
                .status()
                .expect("Failed to compile with clang");

            if status.success() {
                println!("Executable built at {}", executable);
            } else {
                eprintln!("\x1b[31mCompilation failed\x1b[0m");
            }
        }
        Err(errors) => {
            println!("\x1b[31mSyntax Errors:");
            for err in errors.iter() {
                println!("{}", err);
            }
            println!("\x1b[0m");
        }
    }
}