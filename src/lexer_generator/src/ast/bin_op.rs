use std::fmt::Display;

use crate::ast::expression::Expression;

/// Represents a binary operator in a regular expression AST.
///
/// Supported operators include:
/// - `Concat`: Concatenation of two expressions.
/// - `Union`: Alternation (e.g., `a|b`).
pub enum BinaryOperator {
    /// Concatenation operator.
    Concat,
    /// Union (alternation) operator.
    Union,
}

/// Represents a binary operation node in a regular expression abstract syntax tree (AST).
///
/// A binary operation is composed of:
/// - a left-hand side expression,
/// - a right-hand side expression,
/// - and a binary operator (either concatenation or union).
pub struct BinOp {
    /// The left-hand side expression.
    pub left: Box<Expression>,
    /// The right-hand side expression.
    pub right: Box<Expression>,
    /// The binary operator applied between the two expressions.
    pub op: BinaryOperator,
}

impl Display for BinOp {
    /// Formats the binary operation as a string representation of the regular expression.
    ///
    /// Concatenation is displayed as `(left right)` and union as `(left|right)`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.op {
            BinaryOperator::Concat => write!(f, "({}{})", self.left, self.right),
            BinaryOperator::Union => write!(f, "({}|{})", self.left, self.right),
        }
    }
}