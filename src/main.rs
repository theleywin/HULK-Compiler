use lalrpop_util::lalrpop_mod;
mod ast;
mod tokens;

lalrpop_mod!(pub parser);

fn main() {
    let input = "4*2/5+(1^2^3^4)-34";
    
    let expr = parser::ExprsListParser::new()
            .parse(input)
            .unwrap();
    println!("{:?}", expr);
}