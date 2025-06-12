use std::collections::{HashMap, HashSet};

use crate::ast::atoms::regex_atom::RegexAtom;


pub trait NDT {
    fn get_transitions(&self) -> &HashMap<(usize, RegexAtom), HashSet<usize>>;

    fn e_closure(&self, state_set: &HashSet<usize>) -> HashSet<usize> {
        let mut closure = state_set.clone();
        let mut stack: Vec<usize> = state_set.iter().cloned().collect();

        while let Some(state) = stack.pop() {
            if let Some(next_states) = self.get_transitions().get(&(state, RegexAtom::Epsilon)) {
                for &next_state in next_states {
                    if closure.insert(next_state) {
                        stack.push(next_state);
                    }
                }
            }
        }

        closure
    }

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