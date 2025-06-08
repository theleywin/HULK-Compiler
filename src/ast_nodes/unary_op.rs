use crate::{tokens::OperatorToken, types_tree::tree_node::TypeNode};
use super::expression::Expression;

#[derive(Debug, PartialEq)]
pub struct UnaryOpNode {
    pub operator: OperatorToken,
    pub operand: Box<Expression>,
    pub node_type: Option<TypeNode>,
}

impl UnaryOpNode {
    pub fn new(operator: OperatorToken, operand: Expression) -> Self {
        UnaryOpNode {
            operator,
            operand: Box::new(operand),
            node_type: None,
        }
    }
    pub fn set_type(&mut self, node_type: TypeNode) {
        self.node_type = Some(node_type);
    }
}
