use super::expression::Expression;

#[derive(Debug, PartialEq)]
pub struct FunctionCallNode {
    pub function_name: String,             
    pub arguments: Vec<Expression>,
}

impl FunctionCallNode {
    pub fn new(function_name: String, arguments: Vec<Expression>) -> Self {
        FunctionCallNode { function_name, arguments }
    }
}
