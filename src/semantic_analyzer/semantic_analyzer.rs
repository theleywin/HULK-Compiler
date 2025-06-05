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
use crate::types_tree::types_tree::{TypeTree,BuiltInTypes};
use crate::visitor::visitor_trait::Visitor;
use crate::visitor::accept::Accept;
use crate::tokens::OperatorToken;
use crate::types_tree::tree_node::TypeNode;


pub struct SemanticAnalyzer {
    context: SemanticContext,
    scopes: Vec<SemanticContext>,
    errors: Vec<SemanticError>,
    types_tree: TypeTree,
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
            types_tree: TypeTree::new(),
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

    pub fn get_built_in_types(&self , built_in: &BuiltInTypes) -> TypeNode {
        self.types_tree.get_type(built_in.as_str()).unwrap()
    }

}

impl Visitor<TypeNode> for SemanticAnalyzer {

    fn visit_for_loop(&mut self, node: &ForNode) -> TypeNode {
        self.enter_scope();
        self.context.symbols.insert(node.variable.clone(), self.get_built_in_types(&BuiltInTypes::Number));
        let return_type = node.body.accept(self);
        self.exit_scope();
        return_type
    }

    fn visit_destructive_assign(&mut self, node: &DestructiveAssignNode) -> TypeNode {
        if self.context.symbols.contains_key(&node.identifier) {
            let new_type = node.expression.accept(self);
            self.context.symbols.insert(node.identifier.clone(), new_type.clone());
            new_type
        }
        else {
            self.new_error(SemanticError::UndefinedIdentifier(node.identifier.clone()));
            self.get_built_in_types(&BuiltInTypes::Unknown)
        }
    }

    fn visit_function_def(&mut self, node: &FunctionDefNode) -> TypeNode {
        self.enter_scope();
        let func_return= node.return_type.clone();
        let mut arg_types: Vec<TypeNode> = vec![];
        for param in &node.params { 
            if let Some(param_type) = self.types_tree.get_type(&param.signature) {
                self.context.symbols.insert(param.name.clone(), param_type.clone());
                arg_types.push(param_type);
            }
            else {
                self.new_error(SemanticError::UndefinedType(param.signature.clone()));
                self.context.symbols.insert(param.name.clone(), self.get_built_in_types(&BuiltInTypes::Unknown));
                arg_types.push(self.get_built_in_types(&BuiltInTypes::Unknown));
            }
        }
        if self.context.declared_functions.contains_key(&node.name) {
            self.new_error(SemanticError::RedefinitionOfFunction(node.name.clone()));
        } else {
            self.context.declared_functions.insert(node.name.clone(), FunctionInfo::new(node.name.clone(), arg_types.clone(),self.types_tree.get_type(&node.return_type.clone()).unwrap()));
        }
        let body_type = node.body.accept(self);
        let mut return_type_node = self.get_built_in_types(&BuiltInTypes::Unknown);
        if let Some(func_type) = self.types_tree.get_type(&func_return.clone()) {
            if ! self.types_tree.is_ancestor(&func_type, &body_type) {
                self.new_error(SemanticError::InvalidFunctionReturn(body_type, func_type.clone(), node.name.clone()));
            }
            return_type_node = func_type;
        } else {
            self.new_error(SemanticError::UndefinedType(func_return.clone()));
        }
        self.exit_scope();
        self.context.declared_functions.insert(node.name.clone(), FunctionInfo::new(node.name.clone(), arg_types, return_type_node.clone()));
        return_type_node
    }

    fn visit_literal_number(&mut self, _node: &NumberLiteralNode) -> TypeNode {
        self.get_built_in_types(&BuiltInTypes::Number)
    }

    fn visit_literal_boolean(&mut self, _node: &BooleanLiteralNode) -> TypeNode {
        self.get_built_in_types(&BuiltInTypes::Boolean)
    }

    fn visit_literal_string(&mut self, _node: &StringLiteralNode) -> TypeNode {
        self.get_built_in_types(&BuiltInTypes::String)
    }

    fn visit_identifier(&mut self, node: &IdentifierNode) -> TypeNode {
        if let Some(return_type) = self.context.symbols.get(&node.value) {
            return_type.clone()
        } else {
            self.new_error(SemanticError::UndefinedIdentifier(node.value.clone()));
            self.get_built_in_types(&BuiltInTypes::Unknown)
        }
    }

    fn visit_function_call(&mut self, node: &FunctionCallNode) -> TypeNode {
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
            self.get_built_in_types(&BuiltInTypes::Unknown)
        }
    }

    fn visit_while_loop(&mut self, node: &WhileNode) -> TypeNode {
        let condition_type = node.condition.accept(self);
        if condition_type != self.get_built_in_types(&BuiltInTypes::Boolean) {
            self.new_error(SemanticError::InvalidConditionType(condition_type));
        }
        let body_type = node.body.accept(self);
        return body_type;
    }

    fn visit_code_block(&mut self, node: &BlockNode) -> TypeNode {
        self.enter_scope();
        let mut last_type = self.get_built_in_types(&BuiltInTypes::Unknown);
        for expr in node.expression_list.expressions.iter() {
            last_type = expr.accept(self);
        }
        self.exit_scope();
        last_type
    }

    fn visit_binary_op(&mut self, node: &BinaryOpNode) -> TypeNode {
        let left_type = node.left.accept(self);
        let right_type = node.right.accept(self);
        
        match node.operator {
            OperatorToken::PLUS | 
            OperatorToken::MINUS |
            OperatorToken::MUL |
            OperatorToken::DIV |
            OperatorToken::MOD |
            OperatorToken::POW => {
                if left_type == self.get_built_in_types(&BuiltInTypes::Number) && right_type == self.get_built_in_types(&BuiltInTypes::Number) {
                    self.get_built_in_types(&BuiltInTypes::Number)
                } else {
                    self.new_error(SemanticError::InvalidBinaryOperation(left_type, right_type,node.operator.clone()));
                    self.get_built_in_types(&BuiltInTypes::Unknown)
                }
            },
            OperatorToken::GT |
            OperatorToken::GTE |
            OperatorToken::LT |
            OperatorToken::LTE |
            OperatorToken::EQ |
            OperatorToken::NEG => {
                if left_type == self.get_built_in_types(&BuiltInTypes::Number) && right_type == self.get_built_in_types(&BuiltInTypes::Number) {
                    self.get_built_in_types(&BuiltInTypes::Boolean)
                } else {
                    self.new_error(SemanticError::InvalidBinaryOperation(left_type, right_type,node.operator.clone()));
                    self.get_built_in_types(&BuiltInTypes::Unknown)
                }
            }
            OperatorToken::CONCAT => {
                if left_type == self.get_built_in_types(&BuiltInTypes::String) || left_type == self.get_built_in_types(&BuiltInTypes::Boolean) || left_type == self.get_built_in_types(&BuiltInTypes::Number) && right_type == self.get_built_in_types(&BuiltInTypes::String) || right_type == self.get_built_in_types(&BuiltInTypes::Boolean) || right_type == self.get_built_in_types(&BuiltInTypes::Number) {
                    self.get_built_in_types(&BuiltInTypes::String)
                } else {
                    self.new_error(SemanticError::InvalidBinaryOperation(left_type, right_type,node.operator.clone()));
                    self.get_built_in_types(&BuiltInTypes::Unknown)
                }

            },
            OperatorToken::AND |
            OperatorToken::OR => {
                if left_type == self.get_built_in_types(&BuiltInTypes::Boolean) && right_type == self.get_built_in_types(&BuiltInTypes::Boolean) {
                    self.get_built_in_types(&BuiltInTypes::Boolean)
                } else {
                    self.new_error(SemanticError::InvalidBinaryOperation(left_type, right_type,node.operator.clone()));
                    self.get_built_in_types(&BuiltInTypes::Unknown)
                }
            },
            _ => {
                self.new_error(SemanticError::UnknownError(format!("Operator ( {} ) not supported in binary operation",node.operator)));
                self.get_built_in_types(&BuiltInTypes::Unknown)
            }
        }
    }

    fn visit_unary_op(&mut self, node: &UnaryOpNode) -> TypeNode {
        let operand_type = node.operand.accept(self);
        
        match node.operator {
            OperatorToken::NEG => {
                if operand_type == self.get_built_in_types(&BuiltInTypes::Number) {
                    self.get_built_in_types(&BuiltInTypes::Number)
                } else {
                    self.new_error(SemanticError::InvalidUnaryOperation(operand_type, node.operator.clone()));
                    self.get_built_in_types(&BuiltInTypes::Unknown)
                }
            },
            OperatorToken::NOT => {
                if operand_type == self.get_built_in_types(&BuiltInTypes::Boolean) {
                    self.get_built_in_types(&BuiltInTypes::Boolean)
                } else {
                    self.new_error(SemanticError::InvalidUnaryOperation(operand_type, node.operator.clone()));
                    self.get_built_in_types(&BuiltInTypes::Unknown)
                }
            },
            _ => {
                self.new_error(SemanticError::UnknownError(format!("Operator ( {} ) not supported in unary operation",node.operator.clone())));
                self.get_built_in_types(&BuiltInTypes::Unknown)
            }
        }
    }

    fn visit_if_else(&mut self, node: &IfElseNode) -> TypeNode {
        let condition_type = node.condition.accept(self);
        if condition_type != self.get_built_in_types(&BuiltInTypes::Boolean) {
            self.new_error(SemanticError::InvalidConditionType(condition_type));
        }
        let then_type = node.then_expression.accept(self);
        let else_type = node.else_expression.accept(self);
        
        if then_type != else_type {
            let lca = self.types_tree.find_lca(&then_type, &else_type);
            if lca.type_name == "Unknown" {
                self.new_error(SemanticError::UnknownError("Incompatible types in if-else branches".to_string()));
            }
            lca
        } else {
            then_type
        }
    }
    
    fn visit_let_in(&mut self, node: &LetInNode) -> TypeNode {
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