use crate::tokens::TypeSignature;
use super::expression::Expression;

#[derive(Debug, PartialEq,Clone)]
pub struct FunctionParams {
    pub name: String,
    pub signature: TypeSignature,
}

impl FunctionParams {
    pub fn new(name: String, signature: TypeSignature) -> Self {
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
    pub return_type: TypeSignature,
    pub body: Expression,
}

impl FunctionDefNode {
    pub fn new_expr(name: String, params: Vec<FunctionParams>, return_type: TypeSignature, expr: Expression) -> Self {
        FunctionDefNode {
            name,
            params,
            return_type,
            body: expr,
        }
    }
}