use crate::semantic_analyzer::semantic_analyzer::SemanticAnalyzer;
use lalrpop_util::lalrpop_mod;
pub mod ast_nodes;
mod tokens;
pub mod visitor;
pub mod semantic_analyzer;
use crate::visitor::printer_visitor::PrinterVisitor;

lalrpop_mod!(pub parser);

fn main() {
    let input = "for ( i  in range(1,10) ) { i + 5 ; } ;
    let x = 5 , y = 10 in (x + y) ;
    let x = 9 in (y := 10) ;
    function SumLet (a: Number , b : Number) : Number {
        let x = a , y = b in (x + y) ;
    } ;
    ";

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