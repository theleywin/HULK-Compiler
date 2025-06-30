use crate::regex::parser::RegexParser;
use crate::automata::finite_automata::{dfa::DFA, nfa::NFA, nfa_builder::NFABuilder};

/// # Description
/// This module defines regex matchers that use finite automata for pattern matching.

/// # DFAMatcher
/// This struct is used to match strings against a regular expression
/// using a DFA.
pub struct DFAMatcher {
    dfa: DFA,
}

impl DFAMatcher {
    /// Constructs a new `DFAMatcher` from a regular expression string.
    /// # Arguments
    /// * `regex`: A string slice that contains the regular expression to be matched.
    pub fn new(regex: &str) -> Self {
        let parser = RegexParser::new();
        let regex_ast = parser.parse(regex).expect("Failed to parse regex");
        let mut builder = NFABuilder::new();
        let nfa = builder.build_from_regex(&regex_ast);
        let dfa = DFA::from(nfa);
        DFAMatcher { dfa }
    }

    /// Matches the input string against the DFA.
    /// # Arguments
    /// * `input`: A string slice that contains the input to be matched.
    /// # Returns
    /// A boolean indicating whether the input string matches the DFA.
    pub fn matches(&self, input: &str) -> bool {
        self.dfa.simulate(input.chars().collect())
    }
}

/// # Description
/// This struct is used to match strings against a regular expression
/// using a Non-deterministic Finite Automaton (NFA).
pub struct NFAMatcher {
    nfa: NFA,
}

impl NFAMatcher {
    /// Constructs a new `NFAMatcher` from a regular expression string.
    /// # Arguments
    /// * `regex`: A string slice that contains the regular expression to be matched.   
    pub fn new(regex: &str) -> Self {
        let parser = RegexParser::new();
        let regex_ast = parser.parse(regex).expect("Failed to parse regex");
        let mut builder = NFABuilder::new();
        let nfa = builder.build_from_regex(&regex_ast);
        NFAMatcher { nfa }
    }

    /// Matches the input string against the NFA.
    /// # Arguments
    /// * `input`: A string slice that contains the input to be matched.
    /// # Returns
    /// A boolean indicating whether the input string matches the NFA.
    pub fn matches(&self, input: &str) -> bool {
        self.nfa.simulate(input.chars().collect())
    }
}
