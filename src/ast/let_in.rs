use super::Expression;

pub struct Assignment {
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

pub struct LetIn {
    pub assignments: Vec<Assignment>,
    pub body: Box<Expression>,
}

impl LetIn {
    pub fn new(assignments: Vec<Assignment>, body: Expression) -> Self {
        LetIn {
            assignments,
            body: Box::new(body),
        }
    }
}
