use crate::semantic_analyzer::semantic_analyzer::SemanticAnalyzer;
use lalrpop_util::lalrpop_mod;
pub mod ast_nodes;
mod tokens;
pub mod visitor;
pub mod semantic_analyzer;
use crate::visitor::printer_visitor::PrinterVisitor;

lalrpop_mod!(pub parser);

fn main() {
    let input = "function hello(name : String) : String { 5 + 5 } ; ";

    let expr = parser::ProgramParser::new().parse(input).unwrap();
    let mut printer = PrinterVisitor;
    let mut semantic_analyzer = SemanticAnalyzer::new();
    println!("");
    let result = semantic_analyzer.analyze(&expr);
    match result {
        Ok(_) => {
            println!("Semantic Analyzer OK");
        },
        Err(errors) => {
            println!("Errors:");
            for err in errors.iter() {
                println!("{}", err.message());
            }
        }
    }
    println!("");
    println!("{}", printer.print_program(&expr));
}