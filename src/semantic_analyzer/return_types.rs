use std::collections::HashMap;
use crate::types_tree::tree_node::TypeNode;

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionInfo {
    pub name: String,
    pub arguments_types: Vec<(String,TypeNode)>,
    pub return_type: TypeNode
}

impl FunctionInfo {
    pub fn new(name: String, arguments_types: Vec<(String,TypeNode)>, return_type: TypeNode) -> Self {
        FunctionInfo {
            name,
            arguments_types,
            return_type
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SemanticContext {
    pub symbols: HashMap<String, TypeNode>, 
    pub declared_functions: HashMap<String, FunctionInfo>
}