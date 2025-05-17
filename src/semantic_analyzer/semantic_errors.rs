use crate::tokens::{OperatorToken, TypeSignature};

#[derive(Debug, Clone, PartialEq)]
pub enum SemanticError {
    DivisionByZero,
    UndefinedIdentifier(String),
    InvalidConditionType(TypeSignature),
    InvalidBinaryOperation(TypeSignature, TypeSignature, OperatorToken),
    InvalidUnaryOperation(TypeSignature, OperatorToken),
    RedefinitionOfFunction(String),
    UndeclaredFunction(String),
    UnknownError(String),
    InvalidArgumentsCount(usize,usize,String),
    InvalidTypeArgument(TypeSignature,TypeSignature,usize,String),
    InvalidFunctionReturn(TypeSignature,TypeSignature,String)
}

impl SemanticError {
    pub fn message(&self) -> String {
        match self {
            SemanticError::DivisionByZero => "Error: Division by zero is not allowed".to_string(),
            SemanticError::UndefinedIdentifier(identifier) => {
                format!("Error: Undefined identifier: {}.", identifier)
            }
            SemanticError::InvalidConditionType(return_type) => {
                format!("Error: Invalid condition type: {:?}.", return_type)
            }
            SemanticError::InvalidBinaryOperation(left, right, op) => {
                format!(
                    "Error: Invalid binary operation between types {:?} and {:?} with operator {:?}.",
                    left, right, op
                )
            }
            SemanticError::InvalidUnaryOperation(return_type, op) => {
                format!(
                    "Error: Invalid unary operation on type {:?} with operator {:?}.",
                    return_type, op
                )
            }
            SemanticError::UnknownError(message) => {
                format!("Error: {}", message)
            }
            SemanticError::RedefinitionOfFunction(function_name) => {
                format!("Error: function with name {} already exist.", function_name)
            }
            SemanticError::UndeclaredFunction(function_name) => {
                format!("Error: function {} is not declared in this context.", function_name)
            }
            SemanticError::InvalidArgumentsCount(curr_arg_count,func_arg_count,func_name) => {
                format!("Error: function call to {}, expected {} arguments, found {}.",func_name,func_arg_count,curr_arg_count)
            }
            SemanticError::InvalidTypeArgument(curr_type,func_arg_type,arg_pos ,func_name ) => {
                format!("Error: function {} receive {} on position {} but {} was found.",func_name,func_arg_type,arg_pos,curr_type)
            }
            SemanticError::InvalidFunctionReturn(body_type,func_return ,func_name ) => {
                format!("Error: function {} returns {} but function's body returns {}",func_name,func_return,body_type)
            }
        }
    }
}