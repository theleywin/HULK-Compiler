use crate::{ast_nodes::expression::Expression, types_tree::tree_node::TypeNode};

#[derive(Debug, PartialEq,Clone)]
pub struct ForNode {
    pub variable: String,
    pub iterable: Box<Expression>,
    pub body: Box<Expression>,
    pub node_type: Option<TypeNode>,
}

impl ForNode {
    pub fn new(variable: String, iterable:Expression, body: Expression) -> Self {
        ForNode {
            variable,
            iterable: Box::new(iterable),
            body: Box::new(body),
            node_type: None,
        }
    }

    pub fn set_type(&mut self, node_type: TypeNode) {
        self.node_type = Some(node_type);
    }
}