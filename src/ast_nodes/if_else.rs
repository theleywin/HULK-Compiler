use crate::types_tree::tree_node::TypeNode;

use super::expression::Expression;


#[derive(Debug, PartialEq)]
pub struct IfElseNode {
    pub condition: Box<Expression>,
    pub then_expression: Box<Expression>,
    pub else_expression: Box<Expression>,
    pub node_type: Option<TypeNode>,
}

impl IfElseNode {
    pub fn new(condition: Expression,then_expression: Expression,else_expression: Expression) -> Self {
        IfElseNode {
            condition: Box::new(condition),
            then_expression: Box::new(then_expression),
            else_expression: Box::new(else_expression),
            node_type: None,
        }
    }
    pub fn set_type(&mut self, node_type: TypeNode) {
        self.node_type = Some(node_type);
    }
}
