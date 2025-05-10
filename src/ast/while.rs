use super::Expression;

pub struct While {
    pub condition: Box<Expression>,
    pub body: Box<Expression>,
}

impl While {
    pub fn new(while_token: KeywordToken, condition: Expression, body: Expression) -> Self {
        While {
            condition: Box::new(condition),
            body: Box::new(body),
        }
    }
}
