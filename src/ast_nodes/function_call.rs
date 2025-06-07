use crate::types_tree::tree_node::TypeNode;

use super::expression::Expression;

#[derive(Debug, PartialEq)]
pub struct FunctionCallNode {
    pub function_name: String,             
    pub arguments: Vec<Expression>,
    pub node_type: Option<TypeNode>, 
}

impl FunctionCallNode {
    pub fn new(function_name: String, arguments: Vec<Expression>) -> Self {
        FunctionCallNode { function_name, arguments, node_type: None }
    }
    pub fn set_type(&mut self, node_type: TypeNode) {
        self.node_type = Some(node_type);
    }
}
