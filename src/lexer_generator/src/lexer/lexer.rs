use std::{collections::HashMap, fmt::Debug, hash::Hash};

use crate::RegexParser;
use crate::automata::{
    bob_construye_nfa::NfaBuild,
    lexeme::Lexeme,
    lexer_dfa::LexerDFA,
    lexer_nfa::LexerNFA,
};
use super::token_spec::TokenSpec;

/// A generic lexer that uses a set of token specifications to tokenize input strings.
/// 
/// # Type Parameters
/// 
/// * `T` - The token kind type. Must implement `Clone`, `PartialEq`, `Eq`, `Hash`, and `Debug`.
///
/// # Description
///
/// The `Lexer` builds a composite nondeterministic finite automaton (NFA) from
/// token regex patterns, then determinizes it into a deterministic finite automaton (DFA)
/// for efficient token scanning. It supports ignoring tokens such as whitespace.
pub struct Lexer<T>
where
    T: Clone + PartialEq + Eq + Hash + Debug,
{
    ruleset: HashMap<T, TokenSpec<T>>,
    matcher: LexerDFA<T>,
}

impl<T> Lexer<T>
where
    T: Clone + PartialEq + Eq + Hash + Debug,
{
    /// Constructs a new `Lexer` from a list of token specifications.
    ///
    /// # Arguments
    ///
    /// * `specs` - A vector of `TokenSpec`s describing token patterns and kinds.
    ///
    /// # Panics
    ///
    /// Panics if any regex pattern fails to parse.
    pub fn new(specs: Vec<TokenSpec<T>>) -> Self {
        let parser = RegexParser::new();
        let tagged_automata = specs
            .iter()
            .map(|spec| {
                let pattern = parser.parse(&spec.patt).unwrap();
                let mut builder = NfaBuild::new();
                let nfa = builder.build_from_regex(&pattern);
                (nfa, spec.kind.clone())
            })
            .collect::<Vec<_>>();

        let composite_nfa = LexerNFA::new(&tagged_automata);
        let matcher = LexerDFA::new(&composite_nfa);
        let ruleset = specs
            .into_iter()
            .map(|spec| (spec.kind.clone(), spec))
            .collect::<HashMap<_, _>>();

        Lexer { ruleset, matcher }
    }

    /// Splits the input string into a sequence of lexemes (tokens).
    ///
    /// # Arguments
    ///
    /// * `input` - The input string to tokenize.
    ///
    /// # Returns
    ///
    /// * `Ok(Vec<Lexeme<T>>)` containing the tokens if lexing is successful.
    /// * `Err(Vec<String>)` containing error messages if lexing fails.
    ///
    /// Tokens flagged as `ignore` in their `TokenSpec` will be omitted from the output.
    pub fn split<'a>(&self, input: &'a str) -> Result<Vec<Lexeme<'a, T>>, Vec<String>> {
        let scan_result = self.matcher.scan(input);
        let Ok(captures) = scan_result else {
            return Err(scan_result.err().unwrap());
        };

        let filtered = captures
            .into_iter()
            .filter(|lexeme| {
                self.ruleset
                    .get(&lexeme.kind)
                    .map_or(false, |spec| !spec.ignore)
            })
            .collect();

        Ok(filtered)
    }
}