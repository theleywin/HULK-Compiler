use lalrpop_util::lalrpop_mod;
mod ast;
mod tokens;

lalrpop_mod!(pub parser);

fn main() {
    let input = "function cot(x) => 1 / tan(x);function tan(x) => sin(x) / cos(x);print(tan(PI)^2 + cot(PI)^2);";    
    
    let expr = parser::ProgramParser::new()
            .parse(input)
            .unwrap();
    println!("{:?}", expr);
}