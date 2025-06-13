use crate::types_tree::tree_node::TypeNode;

use super::expression::Expression;

#[derive(Debug, PartialEq,Clone)]
pub struct DestructiveAssignNode {
    pub identifier: Box<Expression>,
    pub expression: Box<Expression>,
    pub node_type: Option<TypeNode>, 
}

impl DestructiveAssignNode {
    pub fn new(identifier: Expression, expression: Expression) -> Self {
        Self {
            identifier: Box::new(identifier),
            expression: Box::new(expression),
            node_type: None,
        }
    }
    pub fn set_type(&mut self, node_type: TypeNode) {
        self.node_type = Some(node_type);
    }
}