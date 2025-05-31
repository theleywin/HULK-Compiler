use super::expression::Expression;

#[derive(Debug, PartialEq,Clone)]
pub struct FunctionParams {
    pub name: String,
    pub signature: String,
}

impl FunctionParams {
    pub fn new(name: String, signature: String) -> Self {
        FunctionParams {
            name,
            signature,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct FunctionDefNode {
    pub name: String,
    pub params: Vec<FunctionParams>,
    pub return_type: String,
    pub body: Expression,
}

impl FunctionDefNode {
    pub fn new_expr(name: String, params: Vec<FunctionParams>, return_type: String, expr: Expression) -> Self {
        FunctionDefNode {
            name,
            params,
            return_type,
            body: expr,
        }
    }
}