use super::Expression;


pub struct IfElse {
    pub condition: Box<Expression>,
    pub then_expression: Box<Expression>,
    pub else_expression: Box<Expression>,
}

impl IfElse {
    pub fn new(
        condition: Expression,
        then_expression: Expression,
        else_expression: Expression,
    ) -> Self {
        IfElse {
            condition: Box::new(condition),
            then_expression: Box::new(then_expression),
            else_expression: Box::new(else_expression),
        }
    }
}
