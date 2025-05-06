use lalrpop_util::lalrpop_mod;
mod ast;
mod tokens;

lalrpop_mod!(pub parser);

fn main() {
    let input = "let x = 5 , y = 4 in print(x + y);";    
    
    let expr = parser::ExprsListParser::new()
            .parse(input)
            .unwrap();
    println!("{:?}", expr);
}