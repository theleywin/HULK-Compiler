use core::str;

#[derive(Debug, PartialEq)]
pub struct NumberLiteralNode {
    pub value: f64,
}
impl NumberLiteralNode { 
    pub fn new(value: &str) -> Self {
        NumberLiteralNode {
            value: value.parse::<f64>().unwrap(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct BooleanLiteralNode {
    pub value: bool
}
impl BooleanLiteralNode {
    pub fn new(value: bool) -> Self {
        match value {
            true => BooleanLiteralNode { value: true },
            false => BooleanLiteralNode { value: false },
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct StringLiteralNode {
    pub value: String,
}
impl StringLiteralNode {
    pub fn new(value: &str) -> Self {
        StringLiteralNode {
            value: value.to_string(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct IdentifierNode {
    pub value: String,
}
impl IdentifierNode {
    pub fn new(value: &str) -> Self {
        IdentifierNode {
            value: value.to_string(),
        }
    }
}