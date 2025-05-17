use std::collections::HashMap;
use crate::tokens::TypeSignature;

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionInfo {
    pub name: String,
    pub arguments_types: Vec<TypeSignature>,
    pub return_type: TypeSignature
}

impl FunctionInfo {
    pub fn new(name: String, arguments_types: Vec<TypeSignature>, return_type: TypeSignature) -> Self {
        FunctionInfo {
            name,
            arguments_types,
            return_type
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SemanticContext {
    pub symbols: HashMap<String, TypeSignature>, 
    pub declared_functions: HashMap<String, FunctionInfo>
}