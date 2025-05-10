use super::Expression;
use crate::tokens::OperatorToken;

pub struct UnaryOp {
    pub operator: OperatorToken,
    pub operand: Box<Expression>,
}

impl UnaryOp {
    pub fn new(operator: OperatorToken, operand: Expression) -> Self {
        UnaryOp {
            operator,
            operand: Box::new(operand),
        }
    }
}
