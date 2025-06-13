use std::fmt::Display;

use crate::ast::expression::Expression;

/// Represents a unary operator in a regular expression AST.
///
/// Supported unary operators include:
/// - `KleeneStar`: Matches zero or more repetitions (e.g., `a*`)
/// - `Plus`: Matches one or more repetitions (e.g., `a+`)
/// - `QuestionMark`: Matches zero or one occurrence (e.g., `a?`)
pub enum UnaryOperator {
    /// The Kleene star operator (`*`)
    KleeneStar,
    /// The plus operator (`+`)
    Plus,
    /// The question mark operator (`?`)
    QuestionMark,
}

impl From<char> for UnaryOperator {
    /// Converts a character into a corresponding `UnaryOperator`.
    ///
    /// # Panics
    ///
    /// Panics if the character is not one of `*`, `+`, or `?`.
    fn from(c: char) -> Self {
        match c {
            '*' => UnaryOperator::KleeneStar,
            '+' => UnaryOperator::Plus,
            '?' => UnaryOperator::QuestionMark,
            _ => panic!("Invalid unary operator character"),
        }
    }
}

/// Represents a unary operation node in a regular expression abstract syntax tree (AST).
///
/// A unary operation consists of:
/// - an operand (sub-expression),
/// - and a unary operator applied to it.
pub struct UnOp {
    /// The operand expression to which the unary operator is applied.
    pub operand: Box<Expression>,
    /// The unary operator.
    pub op: UnaryOperator,
}

impl Display for UnOp {
    /// Formats the unary operation as a string representation of the regular expression.
    ///
    /// Examples:
    /// - `a*`, `b+`, or `c?` depending on the operator applied.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.op {
            UnaryOperator::KleeneStar => write!(f, "{}*", self.operand),
            UnaryOperator::Plus => write!(f, "{}+", self.operand),
            UnaryOperator::QuestionMark => write!(f, "{}?", self.operand),
        }
    }
}