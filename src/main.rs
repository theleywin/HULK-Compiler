use lalrpop_util::lalrpop_mod;
mod ast;
mod tokens;

lalrpop_mod!(pub parser);

fn main() {
    let input = "";    
    
    let expr = parser::ProgramParser::new()
            .parse(input)
            .unwrap();
    println!("{:?}", expr);
}