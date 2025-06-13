use lalrpop_util::lalrpop_mod;

lalrpop_mod!(parser);

pub use parser::RegexParser;

pub mod ast;
pub mod automata;
pub mod lexer;

#[cfg(test)]
pub mod test {
    pub mod test_char_match;
    pub mod test_expression;
    pub mod test_parsing;
    pub mod test_nfa;
    pub mod test_lexer;
}
