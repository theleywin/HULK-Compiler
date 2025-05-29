use std::collections::HashMap;
use super::return_types::{FunctionInfo, SemanticContext};
use super::semantic_errors::SemanticError;
use crate::ast_nodes::program::Program;
use crate::ast_nodes::binary_op::BinaryOpNode;
use crate::ast_nodes::function_call::FunctionCallNode;
use crate::ast_nodes::unary_op::UnaryOpNode;
use crate::ast_nodes::if_else::IfElseNode;
use crate::ast_nodes::literals::{NumberLiteralNode,BooleanLiteralNode,StringLiteralNode,IdentifierNode};
use crate::ast_nodes::while_loop::WhileNode;
use crate::ast_nodes::block::BlockNode;
use crate::ast_nodes::let_in::LetInNode;
use crate::ast_nodes::for_loop::ForNode;
use crate::ast_nodes::destructive_assign::DestructiveAssignNode;
use crate::ast_nodes::function_def::FunctionDefNode;
use crate::visitor::visitor_trait::Visitor;
use crate::visitor::accept::Accept;
use crate::tokens::{OperatorToken,TypeSignature};

pub struct SemanticAnalyzer {
    context: SemanticContext,
    scopes: Vec<SemanticContext>,
    errors: Vec<SemanticError>  
}

impl SemanticAnalyzer {
    pub fn new() -> Self {
        Self {
            context: SemanticContext {
                symbols: HashMap::new(),
                declared_functions: HashMap::new(),
            },
            scopes: Vec::new(),
            errors: Vec::new(),
        }
    }

    fn enter_scope(&mut self) {
        self.scopes.push(self.context.clone());
    }

    fn exit_scope(&mut self) {
        self.context = self.scopes.pop().unwrap();
    }

    fn new_error(&mut self, error: SemanticError) {
        self.errors.push(error);
    }

    pub fn analyze(&mut self, node: &Program) -> Result<(), Vec<SemanticError>> {
        for statement in &node.statements {
            statement.accept(self);
        }
        if self.errors.is_empty() {
            Ok(())
        } else {
            Err(self.errors.clone())
        }
    }
}

impl Visitor<TypeSignature> for SemanticAnalyzer {

    fn visit_for_loop(&mut self, node: &ForNode) -> TypeSignature {
        self.enter_scope();
        self.context.symbols.insert(node.variable.clone(), TypeSignature::NumberType);
        let return_type = node.body.accept(self);
        self.exit_scope();
        return_type
    }

    fn visit_destructive_assign(&mut self, node: &DestructiveAssignNode) -> TypeSignature {
        if self.context.symbols.contains_key(&node.identifier) {
            let new_type = node.expression.accept(self);
            self.context.symbols.insert(node.identifier.clone(), new_type.clone());
            new_type
        }
        else {
            self.new_error(SemanticError::UndefinedIdentifier(node.identifier.clone()));
            TypeSignature::UnknownType
        }
    }
    fn visit_function_def(&mut self, node: &FunctionDefNode) -> TypeSignature {
        self.enter_scope();
        let func_return= node.return_type.clone();
        let mut arg_types: Vec<TypeSignature> = vec![];
        for param in &node.params { 
            self.context.symbols.insert(param.name.clone(), param.signature.clone());
            arg_types.push(param.signature.clone());
        }
        if self.context.declared_functions.contains_key(&node.name) {
            self.new_error(SemanticError::RedefinitionOfFunction(node.name.clone()));
        } else {
            self.context.declared_functions.insert(node.name.clone(), FunctionInfo::new(node.name.clone(), arg_types.clone(),node.return_type.clone()));
        }
        let body_type = node.body.accept(self);
        if body_type != func_return.clone() {
            self.new_error(SemanticError::InvalidFunctionReturn(body_type,func_return.clone(),node.name.clone()));
        }
        self.exit_scope();
        self.context.declared_functions.insert(node.name.clone(), FunctionInfo::new(node.name.clone(), arg_types,node.return_type.clone()));
        func_return
    }
    fn visit_literal_number(&mut self, _node: &NumberLiteralNode) -> TypeSignature {
        TypeSignature::NumberType
    }
    fn visit_literal_boolean(&mut self, _node: &BooleanLiteralNode) -> TypeSignature {
        TypeSignature::BooleanType
    }
    fn visit_literal_string(&mut self, _node: &StringLiteralNode) -> TypeSignature {
        TypeSignature::StringType
    }
    fn visit_identifier(&mut self, node: &IdentifierNode) -> TypeSignature {
        if let Some(return_type) = self.context.symbols.get(&node.value) {
            return_type.clone()
        } else {
            self.new_error(SemanticError::UndefinedIdentifier(node.value.clone()));
            TypeSignature::UnknownType
        }
    }
    fn visit_function_call(&mut self, node: &FunctionCallNode) -> TypeSignature {
        if let Some(func_info) = self.context.declared_functions.get(&node.function_name) {
            let arguments_types = func_info.arguments_types.clone();
            let func_name = func_info.name.clone();
            let func_type = func_info.return_type.clone();
            if node.arguments.len() != arguments_types.len() {
                self.new_error(SemanticError::InvalidArgumentsCount(node.arguments.len(), arguments_types.len(), node.function_name.clone()));
            }
            else {
                for (index, arg) in node.arguments.iter().enumerate() {
                    let arg_type = arg.accept(self);
                    if arg_type != arguments_types[index] {
                        self.new_error(SemanticError::InvalidTypeArgument(arg_type, arguments_types[index].clone(), index, func_name.clone()));
                    }
                }
            }
            func_type
        } else {
            self.new_error(SemanticError::UndeclaredFunction(node.function_name.clone()));
            TypeSignature::UnknownType 
        }
    }
    fn visit_while_loop(&mut self, node: &WhileNode) -> TypeSignature {
        let condition_type = node.condition.accept(self);
        if condition_type != TypeSignature::BooleanType {
            self.new_error(SemanticError::InvalidConditionType(condition_type));
        }
        let body_type = node.body.accept(self);
        return body_type;
    }
    fn visit_code_block(&mut self, node: &BlockNode) -> TypeSignature {
        self.enter_scope();
        let mut last_type = TypeSignature::UnknownType;
        for expr in node.expression_list.expressions.iter() {
            last_type = expr.accept(self);
        }
        self.exit_scope();
        last_type
    }
    fn visit_binary_op(&mut self, node: &BinaryOpNode) -> TypeSignature {
        let left_type = node.left.accept(self);
        let right_type = node.right.accept(self);
        
        match node.operator {
            OperatorToken::PLUS | 
            OperatorToken::MINUS |
            OperatorToken::MUL |
            OperatorToken::DIV |
            OperatorToken::MOD |
            OperatorToken::POW |
            OperatorToken::GT |
            OperatorToken::GTE |
            OperatorToken::LT |
            OperatorToken::LTE |
            OperatorToken::EQ |
            OperatorToken::NEG => {
                if left_type == TypeSignature::NumberType && right_type == TypeSignature::NumberType {
                    TypeSignature::NumberType
                } else {
                    self.new_error(SemanticError::InvalidBinaryOperation(left_type, right_type,node.operator.clone()));
                    TypeSignature::UnknownType
                }
            },
            OperatorToken::CONCAT => {
                if left_type == TypeSignature::StringType || left_type == TypeSignature::BooleanType || left_type == TypeSignature::NumberType && right_type == TypeSignature::StringType || right_type == TypeSignature::BooleanType || right_type == TypeSignature::NumberType {
                    TypeSignature::StringType
                } else {
                    self.new_error(SemanticError::InvalidBinaryOperation(left_type, right_type,node.operator.clone()));
                    TypeSignature::UnknownType
                }
                
            },
            OperatorToken::AND |
            OperatorToken::OR => {
                if left_type == TypeSignature::BooleanType && right_type == TypeSignature::BooleanType {
                    TypeSignature::BooleanType
                } else {
                    self.new_error(SemanticError::InvalidBinaryOperation(left_type, right_type,node.operator.clone()));
                    TypeSignature::UnknownType
                }
            },
            _ => {
                self.new_error(SemanticError::UnknownError(format!("Operator ( {} ) not supported in binary operation",node.operator)));
                TypeSignature::UnknownType
            }
        }
    }
    fn visit_unary_op(&mut self, node: &UnaryOpNode) -> TypeSignature {
        let operand_type = node.operand.accept(self);
        
        match node.operator {
            OperatorToken::NEG => {
                if operand_type == TypeSignature::NumberType {
                    TypeSignature::NumberType
                } else {
                    self.new_error(SemanticError::InvalidUnaryOperation(operand_type, node.operator.clone()));
                    TypeSignature::UnknownType
                }
            },
            OperatorToken::NOT => {
                if operand_type == TypeSignature::BooleanType {
                    TypeSignature::BooleanType
                } else {
                    self.new_error(SemanticError::InvalidUnaryOperation(operand_type, node.operator.clone()));
                    TypeSignature::UnknownType
                }
            },
            _ => {
                self.new_error(SemanticError::UnknownError(format!("Operator ( {} ) not supported in unary operation",node.operator.clone())));
                TypeSignature::UnknownType
            }
        }
    }
    fn visit_if_else(&mut self, node: &IfElseNode) -> TypeSignature {
        let condition_type = node.condition.accept(self);
        if condition_type != TypeSignature::BooleanType {
            self.new_error(SemanticError::InvalidConditionType(condition_type));
        }
        let then_type = node.then_expression.accept(self);
        let else_type = node.else_expression.accept(self);
        
        if then_type == else_type {
            then_type
        } else {
            self.new_error(SemanticError::UnknownError("Then and Else branches must return the same type".to_string()));
            TypeSignature::UnknownType
        }
    }
    fn visit_let_in(&mut self, node: &LetInNode) -> TypeSignature {
        self.enter_scope();
        for assig in node.assignments.iter() {
            let expr_type = assig.expression.accept(self);
            if let Some(_) = self.context.symbols.get(&assig.identifier) {
                self.new_error(SemanticError::RedefinitionOfVariable(assig.identifier.clone()));
            } else {
                self.context.symbols.insert(assig.identifier.clone(), expr_type);
            }
        }
        let return_type = node.body.accept(self);
        self.exit_scope();
        return_type
    }
}