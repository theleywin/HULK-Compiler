use super::Expression;
use crate::tokens::OperatorToken;

pub struct BinaryOp {
    pub left: Box<Expression>,
    pub operator: OperatorToken,
    pub right: Box<Expression>,
}

impl BinaryOp {
    pub fn new(left: Expression, operator: OperatorToken, right: Expression) -> Self {
        BinaryOp {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        }
    }
}