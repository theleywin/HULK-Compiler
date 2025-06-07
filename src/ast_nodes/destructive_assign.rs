use super::expression::Expression;

#[derive(Debug, PartialEq)]
pub struct DestructiveAssignNode {
    pub identifier: Box<Expression>,
    pub expression: Box<Expression>,
}

impl DestructiveAssignNode {
    pub fn new(identifier: Expression, expression: Expression) -> Self {
        Self {
            identifier: Box::new(identifier),
            expression: Box::new(expression),
        }
    }
}