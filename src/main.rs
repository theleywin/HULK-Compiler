use lalrpop_util::lalrpop_mod;
pub mod ast_nodes;
mod tokens;

lalrpop_mod!(pub parser);

#[cfg(not(test))]
fn main() {
    let input = "function cot(x) => 1 / tan(x);";

    let expr = parser::ProgramParser::new().parse(input).unwrap();
    println!("{:?}", expr);
}
