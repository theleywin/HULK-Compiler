use lalrpop_util::lalrpop_mod;
mod ast;
mod tokens;

lalrpop_mod!(pub parser);

fn main() {
    let input = "-1 + 5;";
    
    let expr = parser::ExprsListParser::new()
            .parse(input)
            .unwrap();
    println!("{:?}", expr);
}