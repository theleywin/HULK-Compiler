use lalrpop_util::lalrpop_mod;
pub mod ast_nodes;
mod tokens;

lalrpop_mod!(pub parser);

fn main() {
    let input = "let x = 5 in x + x ;";    
    
    let expr = parser::ProgramParser::new()
            .parse(input)
            .unwrap();
    println!("{:?}", expr);
}