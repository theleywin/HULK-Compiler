use std::collections::{HashMap, HashSet};
use super::ndt::NDT;
use crate::ast::atoms::regex_atom::RegexAtom;

/// Represents a Non-deterministic Finite Automaton (NFA).
///
/// The NFA consists of:
/// - a start state,
/// - an accept (final) state,
/// - and a transition map associating `(state, symbol)` pairs to sets of next states.
///
/// This struct supports simulation of input strings to determine acceptance.
pub struct NFA {
    /// The start state of the NFA.
    pub start_state: usize,
    /// The accept (final) state of the NFA.
    pub accept_state: usize,
    /// The transition function represented as a map from `(state, symbol)` to next states.
    pub transitions: HashMap<(usize, RegexAtom), HashSet<usize>>,
}

impl NFA {
    /// Constructs a new NFA with the given start state, accept state, and transition map.
    ///
    /// # Arguments
    ///
    /// * `start_state` - The initial state of the NFA.
    /// * `accept_state` - The final accepting state of the NFA.
    /// * `transitions` - The transition table mapping `(state, symbol)` to sets of next states.
    ///
    /// # Returns
    ///
    /// A new `NFA` instance.
    pub fn new(
        start_state: usize,
        accept_state: usize,
        transitions: HashMap<(usize, RegexAtom), HashSet<usize>>,
    ) -> Self {
        NFA {
            start_state,
            accept_state,
            transitions,
        }
    }

    /// Simulates the NFA on a given input string.
    ///
    /// It computes reachable states using epsilon-closures and transitions for each character,
    /// and returns whether the NFA accepts the input.
    ///
    /// # Arguments
    ///
    /// * `input` - A vector of characters representing the input string to simulate.
    ///
    /// # Returns
    ///
    /// `true` if the NFA accepts the input string, `false` otherwise.
    pub fn simulate(&self, input: Vec<char>) -> bool {
        let mut current_states = self.e_closure(&HashSet::from([self.start_state]));

        for &ch in input.iter() {
            let symbol = RegexAtom::from(ch);
            let reachable = self.move_to(&current_states, &symbol);
            current_states = self.e_closure(&reachable);
        }

        current_states.contains(&self.accept_state)
    }
}

impl NDT for NFA {
    /// Returns the transitions of the NFA.
    fn get_transitions(&self) -> &HashMap<(usize, RegexAtom), HashSet<usize>> {
        &self.transitions
    }
}

/// Prints the transition table of the given NFA in a human-readable format.
///
/// # Arguments
///
/// * `nfa` - The NFA whose transitions and states will be printed.
pub fn print_transition_table(nfa: &NFA) {
    println!("Transition Table:");

    let mut sorted_transitions: Vec<_> = nfa.transitions.iter().collect();
    sorted_transitions.sort_by_key(|((state, symbol), _)| (*state, symbol.clone()));

    for ((state, symbol), destinations) in sorted_transitions {
        println!(
            "  From state {} on symbol {:?} → {:?}",
            state, symbol, destinations
        );
    }

    println!("Start State: {}", nfa.start_state);
    println!("Accept State: {}", nfa.accept_state);
    println!();
}