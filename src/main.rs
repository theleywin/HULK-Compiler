use crate::semantic_analyzer::semantic_analyzer::SemanticAnalyzer;
use lalrpop_util::lalrpop_mod;
pub mod ast_nodes;
mod parser_w_errors;
pub mod semantic_analyzer;
mod tokens;
pub mod types_tree;
pub mod visitor;

lalrpop_mod!(pub parser);

use crate::parser_w_errors::Parser;

fn main() {
    let input = r#"5/"#;

    let parser = Parser::new();
    match parser.parse(input) {
        Ok(mut expr) => {
            println!("\x1b[32mSyntactic Analyzer OK\x1b[0m");
            println!("\x1b[0m");
            let mut semantic_analyzer = SemanticAnalyzer::new();
            let result = semantic_analyzer.analyze(&mut expr);
            match result {
                Ok(_) => {
                    println!("\x1b[32mSemantic Analyzer OK\x1b[0m");
                }
                Err(errors) => {
                    println!("\x1b[31mSemantic Errors:");
                    for err in errors.iter() {
                        println!("{}", err.message());
                    }
                    println!("\x1b[0m");
                }
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
