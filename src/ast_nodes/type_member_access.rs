use crate::{ast_nodes::{expression::Expression, function_call::FunctionCallNode}, types_tree::tree_node::TypeNode};

#[derive(Debug, PartialEq,Clone)]
pub struct TypePropAccessNode {
    pub object: Box<Expression>,
    pub member: Box<String>,
    pub node_type: Option<TypeNode>
    
}

impl TypePropAccessNode {
    pub fn new(object: Expression, member: String) -> Self {
        TypePropAccessNode {
            object: Box::new(object),
            member: Box::new(member),
            node_type: None,
        }
    }
    pub fn set_type(&mut self, node_type: TypeNode) {
        self.node_type = Some(node_type);
    }
}

#[derive(Debug, PartialEq,Clone)]
pub struct TypeFunctionAccessNode {
    pub object: Box<Expression>,
    pub member: Box<FunctionCallNode>,
    pub node_type: Option<TypeNode>
}

impl TypeFunctionAccessNode {
    pub fn new(object: Expression, member: FunctionCallNode) -> Self {
        TypeFunctionAccessNode {
            object: Box::new(object),
            member: Box::new(member),
            node_type: None,
        }
    }
    pub fn set_type(&mut self, node_type: TypeNode) {
        self.node_type = Some(node_type);
    }
}
