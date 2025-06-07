use crate::ast_nodes::binary_op::BinaryOpNode;
use crate::ast_nodes::destructive_assign::DestructiveAssignNode;
use crate::ast_nodes::for_loop::ForNode;
use crate::ast_nodes::function_call::FunctionCallNode;
use crate::ast_nodes::type_def::TypeDefNode;
use crate::ast_nodes::unary_op::UnaryOpNode;
use crate::ast_nodes::if_else::IfElseNode;
use crate::ast_nodes::literals::{NumberLiteralNode,BooleanLiteralNode,StringLiteralNode,IdentifierNode};
use crate::ast_nodes::while_loop::WhileNode;
use crate::ast_nodes::block::BlockNode;
use crate::ast_nodes::let_in::LetInNode;
use crate::ast_nodes::function_def::FunctionDefNode;
use crate::ast_nodes::type_instance::TypeInstanceNode;
use crate::ast_nodes::type_member_access::{TypeFunctionAccessNode, TypePropAccessNode};

pub trait Visitor<T> {
    fn visit_function_def(&mut self, node: &FunctionDefNode) -> T;
    fn visit_literal_number(&mut self, node: &NumberLiteralNode) -> T;
    fn visit_literal_boolean(&mut self, node: &BooleanLiteralNode) -> T;
    fn visit_literal_string(&mut self, node: &StringLiteralNode) -> T;
    fn visit_identifier(&mut self, node: &IdentifierNode) -> T;
    fn visit_function_call(&mut self, node: &FunctionCallNode) -> T;
    fn visit_while_loop(&mut self, node: &WhileNode) -> T;
    fn visit_for_loop(&mut self, node: &ForNode) -> T;
    fn visit_code_block(&mut self, node: &BlockNode) -> T;
    fn visit_binary_op(&mut self, node: &BinaryOpNode) -> T;
    fn visit_unary_op(&mut self, node: &UnaryOpNode) -> T;
    fn visit_if_else(&mut self, node: &IfElseNode) -> T;
    fn visit_let_in(&mut self,node: &LetInNode) -> T;
    fn visit_destructive_assign(&mut self, node: &DestructiveAssignNode) -> T;
    fn visit_type_def(&mut self, node: &TypeDefNode) -> T;
    fn visit_type_instance(&mut self, node: &TypeInstanceNode) -> T;
    fn visit_type_function_access(&mut self, node: &TypeFunctionAccessNode) -> T;
    fn visit_type_prop_access(&mut self, node: &TypePropAccessNode) -> T;
}