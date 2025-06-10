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
    type Point(x : Number, y : Number) {
        x = x;
        y = y;

        getX() : Number => self.x;
        getY() : Number => self.y;

        setX(x: Number) : Number =>  self.x := x ;
        setY(y: Number) : Number => self.y := y ;
    }; 

    let x = new Point(4,5) in ( x.getX() + x.getY() ) ;

    type PolarPoint(rho: Number, phi: Number, lol: Number) inherits Point(rho * phi, rho * phi) {
        rho() : Number => self.getX() ^ 2 + self.getY() ^ 2;
    };

    let x = new PolarPoint(4,5,7) in ( x.getX() + x.getY() ) ;
    let x = new Point(4,5) in ( x.getX() + x.getY() ) ;

    type Person (name : String , lastname: String) {
        name = name;
        lastname = lastname;

        name() : String => name @ lastname ;
        
    } ;

    type Knight inherits Person {
        name() : String => \"Sir\" @ \" \" @ base();
    } ;

    let p = new Knight(\"Diego\", \"Viera\") in p.name() ;

    ";

    let mut expr = parser::ProgramParser::new().parse(input).unwrap();
    let mut printer = PrinterVisitor;
    let mut semantic_analyzer = SemanticAnalyzer::new();
    println!("");
    let result = semantic_analyzer.analyze(&mut expr);
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
    println!("\x1b[34m{}\x1b[0m", printer.print_program(&mut expr));
}