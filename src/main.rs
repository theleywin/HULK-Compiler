use crate::semantic_analyzer::semantic_analyzer::SemanticAnalyzer;
use lalrpop_util::lalrpop_mod;
pub mod ast_nodes;
mod tokens;
pub mod visitor;
pub mod semantic_analyzer;
pub mod types_tree;
use crate::visitor::printer_visitor::PrinterVisitor;

lalrpop_mod!(pub parser);

fn main() {
    let input = "
    function SumPro ( a: Number , b : Number ) : Number {
        if ( a > b ) {
            5 ;
        } else {
            SumLet( a, b ) ;
        }
    } ;
    for ( i in range(1,10) ) {
        if ( i > 5 ) {
            i;
        } else {
            \"hola\";
        }
    };
    let x = 5 in ( x + x ) ;
    let y = 4 , z = 3 in ( x + y + z ) ;
    while ( !(3 < 4) ) { 
        !\"hola\" ;
    };

    let x = SumLet( 5, 5) in x ;

    function SumLet (a: Number , b : Number) : Object {
        if ( a > b ) {
            5 ;
        } else {
            \"hola\" ;
        }
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
            println!("\x1b[31mErrors:");
            for err in errors.iter() {
            println!("{}", err.message());
            }
            println!("\x1b[0m");
        }
    }
    println!("");
    // Imprime el resultado en azul
    println!("\x1b[34m{}\x1b[0m", printer.print_program(&expr));
}