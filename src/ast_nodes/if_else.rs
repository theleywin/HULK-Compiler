use crate::types_tree::tree_node::TypeNode;

use super::expression::Expression;


#[derive(Debug, PartialEq,Clone)]
pub struct IfElseNode {
    pub condition: Box<Expression>,
    pub if_expression: Box<Expression>,
    pub elifs: Vec<(Option<Expression>,Expression)>,
    pub node_type: Option<TypeNode>,
}

impl IfElseNode {
    pub fn new(condition: Expression,if_expression: Expression,elifs: Vec<(Option<Expression>,Expression)>) -> Self {
        IfElseNode {
            condition: Box::new(condition),
            if_expression: Box::new(if_expression),
            elifs,
            node_type: None,
        }
    }
    pub fn set_type(&mut self, node_type: TypeNode) {
        self.node_type = Some(node_type);
    }
}
