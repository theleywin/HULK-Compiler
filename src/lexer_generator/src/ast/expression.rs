use std::fmt::Display;

use crate::ast::{ atoms::regex_atom::MatchableAtom, bin_op::BinOp, un_op::UnOp };

pub enum Expression {
    Atom(MatchableAtom),
    BinOp(BinOp),
    UnOp(UnOp),
}

impl Expression {
    pub fn as_atom(&self) -> Option<&MatchableAtom> {
        if let Self::Atom(v) = self {
            Some(v)
        } else {
            None
        }
    }

pub fn as_bin_op(&self) -> Option<&BinOp> {
        if let Self::BinOp(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_un_op(&self) -> Option<&UnOp> {
        if let Self::UnOp(v) = self {
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
            Self::BinOp(bin_op) => write!(f, "{}", bin_op),
            Self::UnOp(un_op) => write!(f, "{}", un_op),
        }
    }
}