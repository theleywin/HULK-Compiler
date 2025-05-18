use super::expression::Expression;

#[derive(Debug, PartialEq)]
pub struct FunctionDefNode {
    pub name: String,
    pub params: Vec<String>,
    pub body: Expression,
}

impl FunctionDefNode {
    pub fn new_expr(name: String, params: Vec<String>, expr: Expression) -> Self {
        FunctionDefNode {
            name,
            params,
            body: expr,
        }
    }
}