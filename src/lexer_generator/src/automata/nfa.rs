use std::collections::{HashMap, HashSet};
use super::ndt::NDT;
use crate::ast::atoms::regex_atom::RegexAtom;

pub struct NFA {
    pub start_state: usize,
    pub accept_state: usize,
    pub transitions: HashMap<(usize, RegexAtom), HashSet<usize>>,
}

impl NFA {

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
    fn get_transitions(&self) -> &HashMap<(usize, RegexAtom), HashSet<usize>> {
        &self.transitions
    }
}

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