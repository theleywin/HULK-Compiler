use crate::tokens::OperatorToken;
use super::expression::Expression;

#[derive(Debug, PartialEq)]
pub struct UnaryOpNode {
    pub operator: OperatorToken,
    pub operand: Box<Expression>,
}

impl UnaryOpNode {
    pub fn new(operator: OperatorToken, operand: Expression) -> Self {
        UnaryOpNode {
            operator,
            operand: Box::new(operand),
        }
    }
}
