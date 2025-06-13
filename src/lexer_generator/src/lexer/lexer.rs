use std::{collections::HashMap, fmt::Debug, hash::Hash};

use crate::RegexParser;
use crate::automata::{
    bob_construye_nfa::NfaBuild,
    lexeme::Lexeme,
    lexer_dfa::LexerDFA,
    lexer_nfa::LexerNFA,
};
use super::token_spec::TokenSpec;

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