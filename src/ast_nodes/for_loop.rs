use crate::ast_nodes::expression::Expression;

#[derive(Debug, PartialEq)]
pub struct ForNode {
    pub variable: String,
    pub start: Box<Expression>,
    pub end: Box<Expression>,
    pub body: Box<Expression>,
}

impl ForNode {
    pub fn new(variable: String, start: Expression, end: Expression, body: Expression) -> Self {
        ForNode {
            variable,
            start: Box::new(start),
            end: Box::new(end),
            body: Box::new(body),
        }
    }
}