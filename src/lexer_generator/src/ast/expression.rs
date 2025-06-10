use std::fmt::Display;

use crate::ast::{ atoms::regex_atom::MatchableAtom};

pub enum Expression {
    Atom(MatchableSymbol),
}

impl Expression {
    pub fn as_atom(&self) -> Option<&MatchableAtom> {
        if let Self::Atom(v) = self {
            Some(v)
        } else {
            None
        }
    }

}

impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Atom(atom) => write!(f, "{}", atom),
        }
    }
}