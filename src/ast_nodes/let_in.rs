use super::expression::Expression;

#[derive(Debug, PartialEq)]
pub struct Assignment { //TODO Add optional Signature Assignment
    pub identifier: String,
    pub expression: Box<Expression>,
}

impl Assignment {
    pub fn new(identifier: String, expression: Expression) -> Self {
        Assignment {
            identifier,
            expression: Box::new(expression),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct LetInNode {
    pub assignments: Vec<Assignment>,
    pub body: Box<Expression>,
}

impl LetInNode {
    pub fn new(assignments: Vec<Assignment>, body: Expression) -> Self {
        LetInNode {
            assignments,
            body: Box::new(body),
        }
    }
}
