use std::fmt::Display;

use crate::ast::expression::Expression;

pub enum UnaryOperator {
    KleeneStar,
    Plus,
    QuestionMark,
}

impl From<char> for UnaryOperator {
    fn from(c: char) -> Self {
        match c {
            '*' => UnaryOperator::KleeneStar,
            '+' => UnaryOperator::Plus,
            '?' => UnaryOperator::QuestionMark,
            _ => panic!("Invalid unary operator character"),
        }
    }
}

pub struct UnOp {
    pub operand: Box<Expression>,
    pub op: UnaryOperator,
}

impl Display for UnOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.op {
            UnaryOperator::KleeneStar => write!(f, "{}*", self.operand),
            UnaryOperator::Plus => write!(f, "{}+", self.operand),
            UnaryOperator::QuestionMark => write!(f, "{}?", self.operand),
        }
    }
}
