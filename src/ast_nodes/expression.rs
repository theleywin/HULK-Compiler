use super::binary_op::BinaryOpNode;
use super::function_call::FunctionCallNode;
use super::unary_op::UnaryOpNode;
use super::if_else::IfElseNode;
use super::literals::{NumberLiteralNode,BooleanLiteralNode,StringLiteralNode,IdentifierNode};
use super::while_loop::WhileNode;
use super::block::{BlockNode, ExpressionList};
use super::for_loop::ForNode;
use super::let_in::{LetInNode,Assignment};
use super::destructive_assign::DestructiveAssignNode;
use crate::ast_nodes::print::PrintNode;
use crate::ast_nodes::type_member_access::{TypeFunctionAccessNode, TypePropAccessNode};
use crate::tokens::OperatorToken;
use crate::visitor::accept::Accept;
use crate::visitor::visitor_trait::Visitor;
use crate::ast_nodes::type_instance::TypeInstanceNode;

#[derive(Debug, PartialEq,Clone)]
pub enum Expression {
    Number(NumberLiteralNode),
    Boolean(BooleanLiteralNode),
    Str(StringLiteralNode),
    Identifier(IdentifierNode),
    FunctionCall(FunctionCallNode),
    WhileLoop(WhileNode),
    ForLoop(ForNode),
    CodeBlock(BlockNode),
    BinaryOp(BinaryOpNode),
    UnaryOp(UnaryOpNode),
    IfElse(IfElseNode),
    LetIn(LetInNode),
    DestructiveAssign(DestructiveAssignNode),
    TypeInstance(TypeInstanceNode),
    TypeFunctionAccess(TypeFunctionAccessNode),
    TypePropAccess(TypePropAccessNode),
    Print(PrintNode)
}

impl Expression {
    pub fn new_number(value: String) -> Self {
        Expression::Number(NumberLiteralNode::new(&value))
    }

    pub fn new_boolean(value: bool) -> Self {
        Expression::Boolean(BooleanLiteralNode::new(value))
    }

    pub fn new_string(value: String ) -> Self {
        Expression::Str(StringLiteralNode::new(&value))
    }

    pub fn new_identifier(value: String) -> Self {
        Expression::Identifier(IdentifierNode::new(&value))
    }

    pub fn new_function_call(function: String, arguments: Vec<Expression>) -> Self {
        Expression::FunctionCall(FunctionCallNode::new(function, arguments))
    }

    pub fn new_while_loop(condition: Expression, body: Expression) -> Self {
        Expression::WhileLoop(WhileNode::new(condition, body))
    }

    pub fn new_for_loop(variable: String, iterable: Expression , body: Expression) -> Self {
        Expression::ForLoop(ForNode::new(variable, iterable, body))
    }

    pub fn new_code_block(expression_list: ExpressionList) -> Self {
        Expression::CodeBlock(BlockNode::new(expression_list))
    }

    pub fn new_binary_op(left: Expression, operator: OperatorToken, right: Expression) -> Self {
        Expression::BinaryOp(BinaryOpNode::new(left, operator, right))
    }

    pub fn new_unary_op(operator: OperatorToken, operand: Expression) -> Self {
        Expression::UnaryOp(UnaryOpNode::new(operator, operand))
    }

    pub fn new_if_else(condition: Expression, if_expression: Expression, elifs: Vec<(Option<Expression>, Expression)>) -> Self {
        Expression::IfElse(IfElseNode::new(condition, if_expression, elifs))
    }

    pub fn new_let_in(assignments: Vec<Assignment>, body: Expression) -> Self {
        Expression::LetIn(LetInNode::new(assignments, body))
    }

    pub fn new_destructive_assign(identifier: Expression, expr: Expression) -> Self {
        Expression::DestructiveAssign(DestructiveAssignNode::new(identifier, expr))
    }

    pub fn new_type_instance(type_name: String, type_args: Vec<Expression>) -> Self {
        Expression::TypeInstance(TypeInstanceNode::new(type_name, type_args))
    }

    pub fn new_type_function_access(object: Expression, member: FunctionCallNode) -> Self {
        Expression::TypeFunctionAccess(TypeFunctionAccessNode::new(object, member))
    }

    pub fn new_type_prop_access(object: Expression, member: String) -> Self {
        Expression::TypePropAccess(TypePropAccessNode::new(object, member))
    }
    pub fn new_print(expression: Expression) -> Self {
        Expression::Print(PrintNode::new(expression))
    }
}

impl Accept for Expression {
    fn accept<V: Visitor<T>,T>(&mut self, visitor: &mut V) -> T {
        match self {
            Expression::Number(node) => visitor.visit_literal_number(node),
            Expression::Boolean(node) => visitor.visit_literal_boolean(node),
            Expression::Str(node) => visitor.visit_literal_string(node),
            Expression::Identifier(node) => visitor.visit_identifier(node),
            Expression::FunctionCall(node) => visitor.visit_function_call(node),
            Expression::WhileLoop(node) => visitor.visit_while_loop(node),
            Expression::ForLoop(node) => visitor.visit_for_loop(node),
            Expression::CodeBlock(node) => visitor.visit_code_block(node),
            Expression::BinaryOp(node) => visitor.visit_binary_op(node),
            Expression::UnaryOp(node) => visitor.visit_unary_op(node),
            Expression::IfElse(node) => visitor.visit_if_else(node),
            Expression::LetIn(node) => visitor.visit_let_in(node),
            Expression::DestructiveAssign(node) => visitor.visit_destructive_assign(node),
            Expression::TypeInstance(node) => visitor.visit_type_instance(node),
            Expression::TypeFunctionAccess(node) => visitor.visit_type_function_access(node),
            Expression::TypePropAccess(node) => visitor.visit_type_prop_access(node),
            Expression::Print(node) => visitor.visit_print(node)
        }
    }
}
