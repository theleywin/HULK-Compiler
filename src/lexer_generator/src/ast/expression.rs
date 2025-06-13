use std::fmt::Display;

use crate::ast::{ atoms::regex_atom::MatchableAtom, bin_op::BinOp, un_op::UnOp };

/// Represents a node in the abstract syntax tree (AST) for a regular expression.
///
/// An `Expression` can be one of:
/// - a single atom (character, epsilon, wildcard, or charset),
/// - a binary operation (concatenation or union),
/// - a unary operation (Kleene star, plus, or question mark).
pub enum Expression {
    /// A single atomic unit of the regular expression (e.g., `a`, `.`, `[a-z]`, `ε`).
    Atom(MatchableAtom),

    /// A binary operation combining two expressions (e.g., `ab`, `a|b`).
    BinOp(BinOp),

    /// A unary operation applied to a single expression (e.g., `a*`, `a+`, `a?`).
    UnOp(UnOp),
}

impl Expression {
    /// Returns a reference to the inner [`MatchableAtom`] if this expression is an `Atom`.
    ///
    /// # Returns
    ///
    /// `Some(&MatchableAtom)` if the expression is an atom, otherwise `None`.
    pub fn as_atom(&self) -> Option<&MatchableAtom> {
        if let Self::Atom(v) = self {
            Some(v)
        } else {
            None
        }
    }

    /// Returns a reference to the inner [`BinOp`] if this expression is a binary operation.
    ///
    /// # Returns
    ///
    /// `Some(&BinOp)` if the expression is a binary operation, otherwise `None`.
    pub fn as_bin_op(&self) -> Option<&BinOp> {
        if let Self::BinOp(v) = self {
            Some(v)
        } else {
            None
        }
    }

    /// Returns a reference to the inner [`UnOp`] if this expression is a unary operation.
    ///
    /// # Returns
    ///
    /// `Some(&UnOp)` if the expression is a unary operation, otherwise `None`.
    pub fn as_un_op(&self) -> Option<&UnOp> {
        if let Self::UnOp(v) = self {
            Some(v)
        } else {
            None
        }
    }
}

impl Display for Expression {
    /// Formats the expression as a string representing the regular expression syntax.
    ///
    /// Examples:
    /// - `"a"`, `"a|b"`, or `"a*"` depending on the expression node.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Atom(atom) => write!(f, "{}", atom),
            Self::BinOp(bin_op) => write!(f, "{}", bin_op),
            Self::UnOp(un_op) => write!(f, "{}", un_op),
        }
    }
}