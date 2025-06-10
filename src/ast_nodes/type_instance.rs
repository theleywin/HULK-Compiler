use crate::types_tree::tree_node::TypeNode;

use super::expression::Expression;

#[derive(Debug, PartialEq,Clone)]
pub struct TypeInstanceNode {
    pub type_name: String,             
    pub arguments: Vec<Expression>,
    pub node_type: Option<TypeNode>,
}

impl TypeInstanceNode {
    pub fn new(type_name: String, arguments: Vec<Expression>) -> Self {
        TypeInstanceNode { type_name, arguments, node_type: None }
    }
    pub fn set_type(&mut self, node_type: TypeNode) {
        self.node_type = Some(node_type);
    }
}
