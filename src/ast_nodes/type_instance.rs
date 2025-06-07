use super::expression::Expression;

#[derive(Debug, PartialEq)]
pub struct TypeInstanceNode {
    pub type_name: String,             
    pub arguments: Vec<Expression>,
}

impl TypeInstanceNode {
    pub fn new(type_name: String, arguments: Vec<Expression>) -> Self {
        TypeInstanceNode { type_name, arguments }
    }
}
