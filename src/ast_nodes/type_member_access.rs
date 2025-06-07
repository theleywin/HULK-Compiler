use crate::ast_nodes::{expression::Expression, function_call::FunctionCallNode};

#[derive(Debug, PartialEq)]
pub struct TypePropAccessNode {
    pub object: Box<Expression>,
    pub member: Box<String>,
}

impl TypePropAccessNode {
    pub fn new(object: Expression, member: String) -> Self {
        TypePropAccessNode {
            object: Box::new(object),
            member: Box::new(member),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct TypeFunctionAccessNode {
    pub object: Box<Expression>,
    pub member: Box<FunctionCallNode>,
}

impl TypeFunctionAccessNode {
    pub fn new(object: Expression, member: FunctionCallNode) -> Self {
        TypeFunctionAccessNode {
            object: Box::new(object),
            member: Box::new(member),
        }
    }
}
