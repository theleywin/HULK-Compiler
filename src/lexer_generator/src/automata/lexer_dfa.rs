use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
    hash::Hash,
};

use super::lexeme::Lexeme;
use super::lexer_nfa::LexerNFA;
use super::ndt::NDT;
use super::tracker::VisitTracker;
use super::utils::{to_set, to_str};

/// A deterministic finite automaton (DFA) lexer that tokenizes input strings
/// based on a provided nondeterministic finite automaton (NFA) with tagged tokens.
///
/// # Type Parameters
///
/// * `T`: The token kind type, which must implement `Clone`, `PartialEq`, `Eq`, `Hash`, and `Debug`.
///
/// # Fields
///
/// * `start_state`: The initial state of the DFA.
/// * `accepting_states`: A map from DFA states to token kinds indicating accepting states.
/// * `transitions`: A map representing DFA transitions from `(state, input_char)` to next state.
pub struct LexerDFA<T>
where
    T: Clone + PartialEq,
{
    pub start_state: usize,
    pub accepting_states: HashMap<usize, T>,
    pub transitions: HashMap<(usize, char), usize>,
}

impl<T> LexerDFA<T>
where
    T: Clone + PartialEq + Eq + Hash + Debug,
{
    /// Constructs a new `LexerDFA` from a given `LexerNFA`.
    ///
    /// # Arguments
    ///
    /// * `nfa` - A reference to the NFA with tagged token kinds.
    ///
    /// # Returns
    ///
    /// A `LexerDFA` representing the equivalent deterministic lexer.
    pub fn new(nfa: &LexerNFA<T>) -> Self {
        LexerDFA::from(nfa)
    }

    /// Scans the input string and produces a vector of lexemes (tokens).
    ///
    /// Attempts to match the longest possible lexemes according to the DFA transitions.
    /// Returns `Ok` with the lexemes if scanning is successful for the entire input,
    /// or `Err` with a list of lexical error messages if unrecognized characters are found.
    ///
    /// # Arguments
    ///
    /// * `input` - The input string slice to be tokenized.
    ///
    /// # Returns
    ///
    /// `Result<Vec<Lexeme<T>>, Vec<String>>` where the `Ok` variant contains the list of recognized tokens,
    /// and the `Err` variant contains lexical error messages.
    pub fn scan<'a>(&self, input: &'a str) -> Result<Vec<Lexeme<'a, T>>, Vec<String>> {
        let mut results = Vec::new();
        let mut issues = Vec::new();
        let source_chars: Vec<char> = input.chars().collect();
        let mut index = 0;
        let mut line_info = (0, 0);
        let total = source_chars.len();

        while index < total {
            let mut state = self.start_state;
            let mut last_match: Option<(usize, &T)> = None;
            let mut line_snapshot = line_info;
            let mut offset = index;

            while offset < total {
                let ch = source_chars[offset];
                if ch == '\n' {
                    line_snapshot.0 += 1;
                    line_snapshot.1 = offset;
                }

                if let Some(&next_state) = self.transitions.get(&(state, ch)) {
                    state = next_state;
                    if let Some(kind) = self.accepting_states.get(&state) {
                        last_match = Some((offset + 1, kind));
                    }
                    offset += 1;
                } else {
                    break;
                }
            }

            if let Some((end, kind)) = last_match {
                let fragment = &input[index..end];
                results.push(Lexeme::with(
                    kind.clone(),
                    fragment,
                    line_info.0,
                    index,
                    end,
                ));
                line_info = line_snapshot;
                index = end;
            } else {
                issues.push(format!(
                    "Lexical Error!: Unexpected character '{}' at line: {}, column: {}",
                    source_chars[index],
                    line_info.0,
                    index - line_info.1
                ));
                line_info = line_snapshot;
                index += 1;
            }
        }

        if issues.is_empty() {
            Ok(results)
        } else {
            Err(issues)
        }
    }
}

impl<T> From<&LexerNFA<T>> for LexerDFA<T>
where
    T: Clone + PartialEq + Eq + Hash,
{
    /// Converts a `LexerNFA` into an equivalent deterministic `LexerDFA`
    /// using the subset construction algorithm.
    ///
    /// The states of the DFA correspond to sets of NFA states,
    /// with transitions computed via epsilon-closure and moves.
    ///
    /// The accepting states are determined by the presence of any NFA accepting states
    /// in the corresponding subset, selecting the token with the highest priority (lowest number).
    ///
    /// # Arguments
    ///
    /// * `nfa` - A reference to the nondeterministic lexer NFA.
    ///
    /// # Returns
    ///
    /// An equivalent deterministic lexer DFA.
    fn from(nfa: &LexerNFA<T>) -> Self {
        let start_state = 0;
        let mut transitions = HashMap::new();
        let mut tracker = VisitTracker::new();

        let initial = nfa.e_closure(&HashSet::from([nfa.start_state]));
        let id = to_str(&initial);
        tracker.add_unseen(id.clone());

        while let Some(state_id) = tracker.pop_unseen() {
            let state_set = to_set(&state_id);
            for byte in 0..=255u8 {
                let ch = char::from(byte);
                let target_set = nfa.e_closure(&nfa.move_to(&state_set, &ch.into()));

                if target_set.is_empty() {
                    continue;
                }

                let target_id = to_str(&target_set);
                if !tracker.is_seen(&target_id) {
                    tracker.add_unseen(target_id.clone());
                }

                transitions.insert((tracker[&state_id], ch), tracker[&target_id]);
            }
        }

        let accepting_states = tracker
            .iter()
            .filter_map(|sid| {
                let set = to_set(sid);
                set.iter()
                    .filter_map(|q| nfa.accepting_states.get(q).cloned())
                    .min_by_key(|(_, priority)| *priority)
                    .map(|(kind, _)| (tracker[sid], kind))
            })
            .collect();

        LexerDFA {
            start_state,
            accepting_states,
            transitions,
        }
    }
}