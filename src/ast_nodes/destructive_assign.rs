use super::expression::Expression;

#[derive(Debug, PartialEq)]
pub struct DestructiveAssignNode {
    pub identifier: String,
    pub expression: Box<Expression>,
}

impl DestructiveAssignNode {
    pub fn new(identifier: String, expression: Expression) -> Self {
        Self {
            identifier,
            expression: Box::new(expression),
        }
    }
}