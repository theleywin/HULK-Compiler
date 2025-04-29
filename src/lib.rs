pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub mod ast;
pub mod tokens;
lalrpop_util::lalrpop_mod!(pub parser);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn calculator4() {
        let expr = parser::ExprsListParser::new()
            .parse("22 * 44 + 66 , 45 + 2")
            .unwrap();
        assert_eq!(&format!("{:?}", expr[1]), "Op(Op(Number(22), Mul, Number(44)), Add, Number(66))");
    }

}
