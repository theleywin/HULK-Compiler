use core::str;

use crate::types_tree::tree_node::TypeNode;

#[derive(Debug, PartialEq,Clone)]
pub struct NumberLiteralNode {
    pub value: f64,
    pub node_type: Option<TypeNode>,
}
impl NumberLiteralNode { 
    pub fn new(value: &str) -> Self {
        NumberLiteralNode {
            value: value.parse::<f64>().unwrap(),
            node_type: None,
        }
    }
    pub fn set_type(&mut self, node_type: TypeNode) {
        self.node_type = Some(node_type);
    }
}

#[derive(Debug, PartialEq,Clone)]
pub struct BooleanLiteralNode {
    pub value: bool,
    pub node_type: Option<TypeNode>,
}
impl BooleanLiteralNode {
    pub fn new(value: bool) -> Self {
        match value {
            true => BooleanLiteralNode { value: true, node_type: None },
            false => BooleanLiteralNode { value: false, node_type: None },
        }
    }
    pub fn set_type(&mut self, node_type: TypeNode) {
        self.node_type = Some(node_type);
    }
}

#[derive(Debug, PartialEq,Clone)]
pub struct StringLiteralNode {
    pub value: String,
    pub node_type: Option<TypeNode>,
}
impl StringLiteralNode {
    pub fn new(value: &str) -> Self {
        StringLiteralNode {
            value: value.to_string(),
            node_type: None,
        }
    }
    pub fn set_type(&mut self, node_type: TypeNode) {
        self.node_type = Some(node_type);
    }
}

#[derive(Debug, PartialEq,Clone)]
pub struct IdentifierNode {
    pub value: String,
    pub node_type: Option<TypeNode>,
}
impl IdentifierNode {
    pub fn new(value: &str) -> Self {
        IdentifierNode {
            value: value.to_string(),
            node_type: None,
        }
    }
    pub fn set_type(&mut self, node_type: TypeNode) {
        self.node_type = Some(node_type);
    }
}