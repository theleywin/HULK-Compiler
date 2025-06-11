use lalrpop_util::lalrpop_mod;

lalrpop_mod!(parser);

pub use parser::RegexParser;

pub mod ast;

#[cfg(test)]
pub mod test {
    pub mod char_match;
    pub mod test_expression;
    pub mod test_parsing;
}
