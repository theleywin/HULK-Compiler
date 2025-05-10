use super::Expression;

pub struct ExpressionList {
    pub expressions: Vec<Expression>,
}

impl ExpressionList {
    pub fn new(expressions: Vec<Expression>, multiple_semicolon_terminated: bool) -> Self {
        ExpressionList {
            expressions,
        }
    }
}

pub struct Block {
    pub expression_list: ExpressionList,
}

impl Block {
    pub fn new(
        expression_list: ExpressionList,
    ) -> Self {
        Block {
            expression_list
        }
    }
}

