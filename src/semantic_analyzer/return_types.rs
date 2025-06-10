use std::collections::HashMap;
use crate::ast_nodes::type_def::TypeDefNode;

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionInfo {
    pub name: String,
    pub arguments_types: Vec<(String,String)>,
    pub return_type: String,
}

impl FunctionInfo {
    pub fn new(name: String, arguments_types: Vec<(String,String)>, return_type: String) -> Self {
        FunctionInfo {
            name,
            arguments_types,
            return_type
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SemanticContext {
    pub symbols: HashMap<String, String>, // Maps variable names to their types
    pub declared_functions: HashMap<String, FunctionInfo>,
    pub declared_types: HashMap<String, TypeDefNode>,
    pub current_type: Option<String>,
    pub current_function: Option<String>
}