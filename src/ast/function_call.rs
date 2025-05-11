use super::Expression;

pub struct FunctionCall {
    pub function: String,             
    pub arguments: Vec<Expression>,
}

impl FunctionCall {
    pub fn new(function: String, arguments: Vec<Expression>) -> Self {
        FunctionCall { function, arguments }
    }
}
