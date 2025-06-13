use std::{
    cmp::max,
    collections::{HashMap, HashSet},
    fmt::Debug,
};
use super::ndt::NDT;
use crate::ast::atoms::regex_atom::RegexAtom;
use super::nfa::NFA;

/// A Lexer NFA that combines multiple tagged NFAs into a single NFA with prioritized accepting states.
///
/// # Type Parameters
///
/// * `T`: The type used to tag accepting states, representing token kinds or similar metadata.  
///        Must implement `Clone`, `PartialEq`, and optionally `Debug`.
pub struct LexerNFA<T>
where
    T: Clone + PartialEq,
{
    /// The start state of the combined NFA.
    pub start_state: usize,

    /// Mapping from accepting state indices to a tuple of token kind (`T`) and priority.
    /// Priority is used to resolve conflicts when multiple NFAs accept the same input.
    pub accepting_states: HashMap<usize, (T, usize)>,

    /// Transitions of the combined NFA: keys are (state, input symbol) pairs,
    /// values are sets of reachable states.
    pub transitions: HashMap<(usize, RegexAtom), HashSet<usize>>,
}

impl<T> LexerNFA<T>
where
    T: Clone + PartialEq + Debug,
{
    /// Constructs a new `LexerNFA` by combining multiple NFAs tagged with token kinds.
    ///
    /// Each NFA is offset in states to avoid collisions, and epsilon transitions
    /// connect a new start state to each individual NFA's start state.
    ///
    /// # Arguments
    ///
    /// * `tagged_nfas` - A vector of tuples `(NFA, T)` where each NFA is tagged with a token kind.
    ///
    /// # Returns
    ///
    /// A combined `LexerNFA` representing all input NFAs with unique start and accepting states.
    ///
    pub fn new(tagged_nfas: &Vec<(NFA, T)>) -> Self {
        let start_state = 0;
        let mut current_max_state = start_state;

        let mut accepting_states = HashMap::new();
        let mut transitions: HashMap<(usize, RegexAtom), HashSet<usize>> = HashMap::new();
        transitions.insert((start_state, RegexAtom::Epsilon), HashSet::new());

        for (priority, (nfa, token_kind)) in tagged_nfas.iter().enumerate() {
            let state_offset = current_max_state + 1;

            transitions
                .get_mut(&(start_state, RegexAtom::Epsilon))
                .unwrap()
                .insert(nfa.start_state + state_offset);

            for ((origin_state, symbol), target_states) in &nfa.transitions {
                let shifted_targets: HashSet<usize> =
                    target_states.iter().map(|&s| s + state_offset).collect();

                let shifted_origin = origin_state + state_offset;
                current_max_state = max(current_max_state, shifted_origin);
                current_max_state = max(
                    current_max_state,
                    *shifted_targets.iter().max().unwrap_or(&shifted_origin),
                );

                transitions
                    .entry((shifted_origin, symbol.clone()))
                    .or_insert_with(HashSet::new)
                    .extend(shifted_targets);
            }
            accepting_states.insert(nfa.accept_state + state_offset, (token_kind.clone(), priority));
        }

        LexerNFA {
            start_state,
            accepting_states,
            transitions,
        }
    }
}

impl<T> NDT for LexerNFA<T>
where
    T: Clone + PartialEq,
{
    /// Returns a reference to the transitions map.
    ///
    /// This is used by the NDT trait to query transitions of the automaton.
    fn get_transitions(&self) -> &HashMap<(usize, RegexAtom), HashSet<usize>> {
        &self.transitions
    }
}