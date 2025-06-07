use crate::{ast_nodes::expression::Expression, types_tree::tree_node::TypeNode};

#[derive(Debug, PartialEq)]
pub struct WhileNode {
    pub condition: Box<Expression>,
    pub body: Box<Expression>,
    pub node_type: Option<TypeNode>,
}

impl WhileNode {
    pub fn new(condition: Expression, body: Expression) -> Self {
        WhileNode {
            condition: Box::new(condition),
            body: Box::new(body),
            node_type: None,
        }
    }
    pub fn set_type(&mut self, node_type: TypeNode) {
        self.node_type = Some(node_type);
    }
}
