/// Automata module
/// 
/// This module contains all automata-related functionality, including
/// basic finite automata, extended super automata for lexing, and utilities.

pub mod finite_automata;
pub mod super_automata;
pub mod utils;

// Re-export commonly used types for convenience
pub use finite_automata::{nfa::NFA, dfa::DFA, nfa_builder::NFABuilder};
pub use super_automata::{super_nfa::SuperNFA, super_dfa::SuperDFA};
pub use utils::{transitionable::NDTransitionable, marked_queue::MarkedQueue};
