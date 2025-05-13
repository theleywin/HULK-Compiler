use super::expression::Expression;

#[derive(Debug, PartialEq)]
pub struct ExpressionList {
    pub expressions: Box<Vec<Expression>>,
}

impl ExpressionList {
    pub fn new(expressions: Vec<Expression>) -> Self {
        ExpressionList {
            expressions: Box::new(expressions),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct BlockNode {
    pub expression_list: Box<ExpressionList>,
}

impl BlockNode {
    pub fn new(expression_list: ExpressionList) -> Self {
        BlockNode {
            expression_list: Box::new(expression_list)
        }
    }
}

