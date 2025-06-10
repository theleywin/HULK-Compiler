use std::fmt::Display;

use crate::ast::expression::Expression;

pub enum BinaryOperator {
    Concat,
    Union,
}


pub struct BinOp {
    pub left: Box<Expression>,
    pub right: Box<Expression>,
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
