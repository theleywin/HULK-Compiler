// Main modules
pub mod core;
pub mod automata;
pub mod regex;

#[cfg(test)]
pub mod tests;

// Re-export main public API
pub use core::{Lexer, Rule, LexerChunk, Token, LexerResult};
pub use regex::{DFAMatcher, NFAMatcher};
pub use regex::parser::RegexParser;
pub use automata::{NFA, DFA, NFABuilder, SuperNFA, SuperDFA};
