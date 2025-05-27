use lalrpop_util::lalrpop_mod;
pub mod ast_nodes;
mod tokens;
pub mod visitor;
use crate::visitor::accept::Accept;
use crate::visitor::printer_visitor::PrinterVisitor;

lalrpop_mod!(pub parser);

fn main() {
    let input = "let a = 42 in if (a % 2 == 0 & a == 42) print(\"even\") else print(\"odd\");";

    let expr = parser::ProgramParser::new().parse(input).unwrap();
    let mut printer = PrinterVisitor;
    println!("");
    println!("{}", expr.accept(&mut printer));
}
