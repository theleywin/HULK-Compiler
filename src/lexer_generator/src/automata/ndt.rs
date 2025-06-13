use std::collections::{HashMap, HashSet};

use crate::ast::atoms::regex_atom::RegexAtom;

/// Trait that provides common operations for Non-deterministic Finite Automata (NFA).
///
/// This trait abstracts the transition table and provides methods for computing
/// epsilon closures and transitions under a given input symbol.
pub trait NDT {
    /// Returns a reference to the transition map of the NFA.
    ///
    /// The transition map is a `HashMap` where:
    /// - keys are pairs `(state, symbol)` (using `RegexAtom`),
    /// - values are `HashSet` of destination states.
    fn get_transitions(&self) -> &HashMap<(usize, RegexAtom), HashSet<usize>>;

    /// Computes the epsilon-closure of a given set of states.
    ///
    /// The epsilon-closure of a state set includes all states that can be reached
    /// by zero or more epsilon (`ε`) transitions.
    ///
    /// # Arguments
    ///
    /// * `state_set` - A set of states to start the closure computation from.
    ///
    /// # Returns
    ///
    /// A new `HashSet<usize>` containing the states in the epsilon-closure.
    fn e_closure(&self, state_set: &HashSet<usize>) -> HashSet<usize> {
        let mut closure = state_set.clone();
        let mut stack: Vec<usize> = state_set.iter().cloned().collect();

        while let Some(state) = stack.pop() {
            if let Some(next_states) = self.get_transitions().get(&(state, RegexAtom::Epsilon)) {
                for &next_state in next_states {
                    // Only push and insert if the state hasn't already been seen
                    if closure.insert(next_state) {
                        stack.push(next_state);
                    }
                }
            }
        }

        closure
    }

    /// Computes the set of states reachable from the given state set via a specific symbol.
    ///
    /// This corresponds to the classical `move` operation in automata theory.
    ///
    /// # Arguments
    ///
    /// * `state_set` - A set of current NFA states.
    /// * `symbol` - The symbol under which transitions are considered.
    ///
    /// # Returns
    ///
    /// A `HashSet<usize>` of all reachable states using the given symbol.
    fn move_to(&self, state_set: &HashSet<usize>, symbol: &RegexAtom) -> HashSet<usize> {
        let mut reachable = HashSet::new();

        for &state in state_set {
            if let Some(next_states) = self.get_transitions().get(&(state, symbol.clone())) {
                reachable.extend(next_states);
            }
        }

        reachable
    }
}