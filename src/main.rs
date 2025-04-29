use lalrpop_util::lalrpop_mod;
mod ast;
mod tokens;

lalrpop_mod!(pub parser);

fn main() {
    let input = "5 * 3 + (2 - 1) / 4 ; 4 * 4 + 2";
    
    let expr = parser::ExprsListParser::new()
            .parse(input)
            .unwrap();
    println!("{:?}", expr);
}