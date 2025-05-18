use lalrpop_util::lalrpop_mod;
pub mod ast_nodes;
pub mod visitor;
mod tokens;
use crate::visitor::printer_visitor::PrinterVisitor;
use crate::visitor::accept::Accept;


lalrpop_mod!(pub parser);

fn main() {
    let input = "let x = 5 in x + x ; while ( 10 > 0 ) { let y = 10 in y + y ; }";    
    
    let expr = parser::ProgramParser::new()
            .parse(input)
            .unwrap();
    let mut printer = PrinterVisitor;
    println!("");
    println!("{}", expr.accept(&mut printer));
}
