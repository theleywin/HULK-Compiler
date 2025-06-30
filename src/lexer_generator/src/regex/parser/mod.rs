use lalrpop_util::lalrpop_mod;

lalrpop_mod!(grammar);

pub use grammar::RegexParser;
