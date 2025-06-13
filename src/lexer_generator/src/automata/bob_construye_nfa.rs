use std::collections::{HashMap, HashSet};
use crate::automata::nfa::NFA;
use crate::ast::bin_op::{BinOp, BinaryOperator};
use crate::ast::un_op::{UnOp, UnaryOperator};
use crate::ast::expression::Expression;
use crate::ast::atoms::regex_atom::{RegexAtom, MatchableAtom};

/// Builder for constructing an NFA from a regular expression AST.
///
/// The builder keeps track of unique state IDs to create distinct states
/// for the resulting NFA.
pub struct NfaBuild {
    state_counter: usize,
}

impl NfaBuild {
    /// Creates a new `NfaBuild` instance with the state counter initialized to zero.
    pub fn new() -> Self {
        NfaBuild { state_counter: 0 }
    }

    /// Builds an NFA from a given regular expression AST (`Expression`).
    ///
    /// This method recursively converts the regex expression into an NFA.
    ///
    /// # Arguments
    ///
    /// * `regex` - The reference to the regular expression AST node.
    ///
    /// # Returns
    ///
    /// An NFA representing the given regex.
    pub fn build_from_regex(&mut self, regex: &Expression) -> NFA {
        match regex {
            Expression::Atom(symbol) => self.build_symbol(symbol),
            Expression::BinOp(binary_op) => self.build_binary_op(binary_op),
            Expression::UnOp(unary_op) => self.build_unary_op(unary_op),
        }
    }

    /// Builds a simple NFA from a single symbol (atom).
    ///
    /// # Arguments
    ///
    /// * `symbol` - The atom symbol to build the NFA for.
    ///
    /// # Returns
    ///
    /// An NFA that accepts exactly the given symbol.
    fn build_symbol(&mut self, symbol: &MatchableAtom) -> NFA {
        let start_state = self.state_counter;
        let accept_state = self.state_counter + 1;
        self.state_counter += 2;

        let mut transitions = HashMap::new();

        match symbol {
            MatchableAtom::AtomSet(char_set) => {
                for byte in 0u8..=255u8 {
                    let character = byte as char;
                    if *char_set == character {
                        transitions.insert((start_state, RegexAtom::Char(character)), HashSet::from([accept_state]));
                    }
                }
            }
            MatchableAtom::Atom(sym) => {
                transitions.insert((start_state, sym.clone()), HashSet::from([accept_state]));
            }
        }
        NFA { start_state, accept_state, transitions }
    }

    /// Builds an NFA for a binary operation (concatenation or union).
    ///
    /// # Arguments
    ///
    /// * `binary_op` - The binary operation AST node.
    ///
    /// # Returns
    ///
    /// An NFA representing the binary operation applied to its operands.
    fn build_binary_op(&mut self, binary_op: &BinOp) -> NFA {
        let operator = &binary_op.op;
        let left_nfa = self.build_from_regex(&binary_op.left);
        let right_nfa = self.build_from_regex(&binary_op.right);

        match operator {
            BinaryOperator::Concat => self.concat(&left_nfa, &right_nfa),
            BinaryOperator::Union => self.union(&left_nfa, &right_nfa),
        }
    }

    /// Builds an NFA for a unary operation (Kleene star, plus, or question mark).
    ///
    /// # Arguments
    ///
    /// * `unary_op` - The unary operation AST node.
    ///
    /// # Returns
    ///
    /// An NFA representing the unary operation applied to its operand.
    fn build_unary_op(&mut self, unary_op: &UnOp) -> NFA {
        let operator = &unary_op.op;
        let operand_nfa = self.build_from_regex(&unary_op.operand);

        match operator {
            UnaryOperator::KleeneStar => self.kleene_star(&operand_nfa),
            UnaryOperator::Plus => self.one_or_more(&operand_nfa),
            UnaryOperator::QuestionMark => self.optional(&operand_nfa),
        }
    }

    /// Builds the concatenation of two NFAs.
    ///
    /// The accept state of the first NFA is connected to the start state
    /// of the second NFA via epsilon transitions.
    ///
    /// # Arguments
    ///
    /// * `nfa1` - The left operand NFA.
    /// * `nfa2` - The right operand NFA.
    ///
    /// # Returns
    ///
    /// A new NFA representing the concatenation.
    fn concat(&mut self, nfa1: &NFA, nfa2: &NFA) -> NFA {
        let start_state = nfa1.start_state;
        let accept_state = nfa2.accept_state;

        let mut transitions = nfa1.transitions.clone();
        for ((state, symbol), targets) in nfa2.transitions.iter() {
            if *state == nfa2.start_state {
                transitions.insert((nfa1.accept_state, symbol.clone()), targets.clone());
            } else {
                transitions.insert((*state, symbol.clone()), targets.clone());
            }
        }

        NFA { start_state, accept_state, transitions }
    }

    /// Builds the union (alternation) of two NFAs.
    ///
    /// A new start state and accept state are created with epsilon transitions
    /// connecting them appropriately to the operand NFAs.
    ///
    /// # Arguments
    ///
    /// * `nfa1` - The first operand NFA.
    /// * `nfa2` - The second operand NFA.
    ///
    /// # Returns
    ///
    /// A new NFA representing the union.
    fn union(&mut self, nfa1: &NFA, nfa2: &NFA) -> NFA {
        let start_state = self.state_counter;
        let accept_state = self.state_counter + 1;
        self.state_counter += 2;

        let mut transitions = HashMap::new();
        transitions.insert((start_state, RegexAtom::Epsilon), HashSet::from([nfa1.start_state, nfa2.start_state]));
        transitions.insert((nfa1.accept_state, RegexAtom::Epsilon), HashSet::from([accept_state]));
        transitions.insert((nfa2.accept_state, RegexAtom::Epsilon), HashSet::from([accept_state]));

        for ((state, symbol), targets) in nfa1.transitions.iter() {
            transitions.insert((*state, symbol.clone()), targets.clone());
        }
        for ((state, symbol), targets) in nfa2.transitions.iter() {
            transitions.insert((*state, symbol.clone()), targets.clone());
        }

        NFA { start_state, accept_state, transitions }
    }

    /// Builds the Kleene star (*) operation on an NFA.
    ///
    /// Adds new start and accept states with epsilon transitions to implement zero or more repetitions.
    ///
    /// # Arguments
    ///
    /// * `nfa` - The operand NFA.
    ///
    /// # Returns
    ///
    /// A new NFA representing the Kleene star of the operand.
    fn kleene_star(&mut self, nfa: &NFA) -> NFA {
        let start_state = self.state_counter;
        let accept_state = self.state_counter + 1;
        self.state_counter += 2;

        let mut transitions = HashMap::new();
        transitions.insert((start_state, RegexAtom::Epsilon), HashSet::from([nfa.start_state, accept_state]));
        transitions.insert((nfa.accept_state, RegexAtom::Epsilon), HashSet::from([nfa.start_state, accept_state]));

        for ((state, symbol), targets) in nfa.transitions.iter() {
            transitions.insert((*state, symbol.clone()), targets.clone());
        }

        NFA { start_state, accept_state, transitions }
    }

    /// Builds the one-or-more (+) operation on an NFA.
    ///
    /// Similar to Kleene star but requires at least one occurrence.
    ///
    /// # Arguments
    ///
    /// * `nfa` - The operand NFA.
    ///
    /// # Returns
    ///
    /// A new NFA representing one or more repetitions.
    fn one_or_more(&mut self, nfa: &NFA) -> NFA {
        let start_state = self.state_counter;
        let accept_state = self.state_counter + 1;
        self.state_counter += 2;

        let mut transitions = HashMap::new();
        transitions.insert((start_state, RegexAtom::Epsilon), HashSet::from([nfa.start_state]));
        transitions.insert((nfa.accept_state, RegexAtom::Epsilon), HashSet::from([nfa.start_state, accept_state]));

        for ((state, symbol), targets) in nfa.transitions.iter() {
            transitions.insert((*state, symbol.clone()), targets.clone());
        }

        NFA { start_state, accept_state, transitions }
    }

    /// Builds the optional (?) operation on an NFA.
    ///
    /// Adds an epsilon transition from start to accept state, allowing zero or one occurrence.
    ///
    /// # Arguments
    ///
    /// * `nfa` - The operand NFA.
    ///
    /// # Returns
    ///
    /// A new NFA representing an optional occurrence.
    fn optional(&mut self, nfa: &NFA) -> NFA {
        let start_state = nfa.start_state;
        let accept_state = nfa.accept_state;
        let mut transitions = nfa.transitions.clone();

        let epsilon_targets = transitions.get_mut(&(start_state, RegexAtom::Epsilon));
        if let Some(targets) = epsilon_targets {
            targets.insert(accept_state);
        } else {
            transitions.insert((start_state, RegexAtom::Epsilon), HashSet::from([accept_state]));
        }

        NFA { start_state, accept_state, transitions }
    }
}