use std::{
    cmp::max,
    collections::{HashMap, HashSet},
    fmt::Debug,
};
use super::ndt::NDT;
use crate::ast::atoms::regex_atom::RegexAtom;
use super::nfa::NFA;

pub struct LexerNFA<T>
where
    T: Clone + PartialEq,
{
    pub start_state: usize,
    pub accepting_states: HashMap<usize, (T, usize)>,
    pub transitions: HashMap<(usize, RegexAtom), HashSet<usize>>,
}


impl<T> LexerNFA<T>
where
    T: Clone + PartialEq + Debug,
{
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
    fn get_transitions(&self) -> &HashMap<(usize, RegexAtom), HashSet<usize>> {
        &self.transitions
    }
}