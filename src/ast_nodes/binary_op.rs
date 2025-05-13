use super::expression::Expression;
use crate::tokens::OperatorToken;

#[derive(Debug, PartialEq)]
pub struct BinaryOpNode {
    pub left: Box<Expression>,
    pub operator: OperatorToken,
    pub right: Box<Expression>,
}

impl BinaryOpNode {
    pub fn new(left: Expression, operator: OperatorToken, right: Expression) -> Self {
        BinaryOpNode {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        }
    }
}