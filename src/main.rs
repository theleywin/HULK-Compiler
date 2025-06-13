use crate::semantic_analyzer::semantic_analyzer::SemanticAnalyzer;
use lalrpop_util::lalrpop_mod;
mod parser_w_errors;
pub mod ast_nodes;
pub mod semantic_analyzer;
mod tokens;
pub mod types_tree;
pub mod visitor;

lalrpop_mod!(pub parser);

use crate::parser_w_errors::Parser;

fn main() {
    let input = "function while (a: String):Number => 5+1 ;";

    let parser = Parser::new();
    match parser.parse(input) {
        Ok(mut expr) => {
            let mut semantic_analyzer = SemanticAnalyzer::new();
            println!("");
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