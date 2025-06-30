use std::fmt::Display;
use crate::regex::ast::expressions::RegexExp;

/// Regex operators module
/// 
/// This module contains binary and unary operator definitions for regex AST.

/// Represents a binary operator in a regular expression, such as concatenation or union.
/// This enum is used to define operations that can be applied to two regular expression operands.
/// # Variants
/// - `Concat`: Represents the concatenation operator, which combines two regular expressions in sequence.
/// - `Union`: Represents the union operator, which matches either of the two regular expressions.
pub enum BinaryOperator {
    Concat,
    Union,
}

/// Represents a binary operation in a regular expression, which consists of two operands and a binary operator.
/// This struct is used to encapsulate the operation applied to two regular expression operands.
/// # Fields
/// - `left`: A boxed `RegexExp` representing the left operand of the binary operation.
/// - `right`: A boxed `RegexExp` representing the right operand of the binary operation.
/// - `op`: A `BinaryOperator` that specifies the type of binary operation to be performed on the operands.
pub struct BinOp {
    pub left: Box<RegexExp>,
    pub right: Box<RegexExp>,
    pub op: BinaryOperator,
}

impl Display for BinOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.op {
            BinaryOperator::Concat => write!(f, "({}{})", self.left, self.right),
            BinaryOperator::Union => write!(f, "({}|{})", self.left, self.right),
        }
    }
}

/// Represents a unary operator in a regular expression, such as Kleene star, plus, or optional.
/// This enum is used to define operations that can be applied to a regular expression operand.
/// # Variants
/// - `KleeneStar`: Represents the Kleene star operator (`*`), which matches zero or more occurrences of the operand.
/// - `Plus`: Represents the plus operator (`+`), which matches one or more occurrences of the operand.
/// - `Optional`: Represents the optional operator (`?`), which matches zero or one occurrence of the operand.
pub enum UnaryOperator {
    KleeneStar,
    Plus,
    Optional,
}

impl From<char> for UnaryOperator {
    fn from(c: char) -> Self {
        match c {
            '*' => UnaryOperator::KleeneStar,
            '+' => UnaryOperator::Plus,
            '?' => UnaryOperator::Optional,
            _ => panic!("Invalid unary operator character"),
        }
    }
}

/// Represents a unary operation in a regular expression, which consists of an operand and a unary operator.
/// This struct is used to encapsulate the operation applied to a regular expression operand.
/// # Fields
/// - `operand`: A boxed `RegexExp` representing the operand of the unary operation.
/// - `op`: A `UnaryOperator` that specifies the type of unary operation to be performed on the operand.
pub struct UnOp {
    pub operand: Box<RegexExp>,
    pub op: UnaryOperator,
}

impl Display for UnOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.op {
            UnaryOperator::KleeneStar => write!(f, "{}*", self.operand),
            UnaryOperator::Plus => write!(f, "{}+", self.operand),
            UnaryOperator::Optional => write!(f, "{}?", self.operand),
        }
    }
}
