use crate::ast_nodes::expression::Expression;

#[derive(Debug, PartialEq)]
pub struct WhileNode {
    pub condition: Box<Expression>,
    pub body: Box<Expression>,
}

impl WhileNode {
    pub fn new(condition: Expression, body: Expression) -> Self {
        WhileNode {
            condition: Box::new(condition),
            body: Box::new(body),
        }
    }
}
