use crate::types_tree::tree_node::TypeNode;

use super::expression::Expression;

#[derive(Debug, PartialEq,Clone)]
pub struct Assignment { //TODO Add optional Signature Assignment
    pub identifier: String,
    pub expression: Box<Expression>,
}

impl Assignment {
    pub fn new(identifier: String, expression: Expression) -> Self {
        Assignment {
            identifier,
            expression: Box::new(expression),
        }
    }
}

#[derive(Debug, PartialEq,Clone)]
pub struct LetInNode {
    pub assignments: Vec<Assignment>,
    pub body: Box<Expression>,
    pub node_type: Option<TypeNode>,
}

impl LetInNode {
    pub fn new(assignments: Vec<Assignment>, body: Expression) -> Self {
        LetInNode {
            assignments,
            body: Box::new(body),
            node_type: None,
        }
    }
    pub fn set_type(&mut self, node_type: TypeNode) {
        self.node_type = Some(node_type);
    }
}
