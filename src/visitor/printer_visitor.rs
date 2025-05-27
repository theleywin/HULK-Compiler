use crate::ast_nodes::binary_op::BinaryOpNode;
use crate::ast_nodes::function_call::FunctionCallNode;
use crate::ast_nodes::unary_op::UnaryOpNode;
use crate::ast_nodes::if_else::IfElseNode;
use crate::ast_nodes::literals::{NumberLiteralNode,BooleanLiteralNode,StringLiteralNode,IdentifierNode};
use crate::ast_nodes::while_loop::WhileNode;
use crate::ast_nodes::for_loop::ForNode;
use crate::ast_nodes::block::BlockNode;
use crate::ast_nodes::let_in::LetInNode;
use crate::ast_nodes::function_def::FunctionDefNode;
use crate::ast_nodes::destructive_assign::DestructiveAssignNode;
use super::visitor_trait::Visitor;
use super::accept::Accept;

pub struct PrinterVisitor; 
//Here we can store data like variables and functions, but we just print things in this case
//so we don't need to store anything

impl Visitor<String> for PrinterVisitor {
    fn visit_program(&mut self, node: &crate::ast_nodes::program::Program) -> String {
        let statements: Vec<String> = node.statements.iter()
            .map(|statement| format!("{} ;\n", statement.accept(self)))
            .collect();
        format!("{}", statements.join("\n")) 
    }
    fn visit_function_def(&mut self, node: &FunctionDefNode) -> String {
        let name = &node.name;
        let params = node.params.join(", ");
        let body = node.body.accept(self);
        format!("function {} ({}) {{ \n{}\n}}" , name, params, body)
    }
    fn visit_literal_number(&mut self, node: &NumberLiteralNode) -> String {
        format!("{}", node.value)
    }
    fn visit_literal_boolean(&mut self, node: &BooleanLiteralNode) -> String {
        format!("{}", node.value)
    }
    fn visit_literal_string(&mut self, node: &StringLiteralNode) -> String {
        format!("\"{}\"", node.value)
    }
    fn visit_identifier(&mut self, node: &IdentifierNode) -> String {
        format!("{}", node.value)
    }
    fn visit_function_call(&mut self, node: &FunctionCallNode) -> String {
        let args: Vec<String> = node.arguments.iter()
            .map(|arg| arg.accept(self))
            .collect();

        format!("{}({})", node.function_name, args.join(", "))
    }
    fn visit_while_loop(&mut self, node: &WhileNode) -> String {
        let condition = node.condition.accept(self);
        let body = node.body.accept(self);
        format!("while ({}) {{\n{}\n}}", condition, body)
    }
    fn visit_for_loop(&mut self, node: &ForNode) -> String {
        let variable = &node.variable;
        let start = node.start.accept(self);
        let end = node.end.accept(self);
        let body = node.body.accept(self);
        format!("for ({} in range({}, {})) {{\n{}\n}}", variable, start, end, body)
    }
    fn visit_code_block(&mut self, node: &BlockNode) -> String {
        let expressions: Vec<String> = node.expression_list.expressions.iter()
            .map(|expr| expr.accept(self))
            .collect();
        format!("{}", expressions.join("\n"))
    }
    fn visit_binary_op(&mut self, node: &BinaryOpNode) -> String {
        let left = node.left.accept(self);
        let right = node.right.accept(self);
        
        format!("{} {} {}", left, node.operator, right)
    }
    fn visit_unary_op(&mut self, node: &UnaryOpNode) -> String {
        let operand = node.operand.accept(self);
        format!("{} {}", node.operator, operand)
    }
    fn visit_if_else(&mut self, node: &IfElseNode) -> String {
        let condition = node.condition.accept(self);
        let then_branch = node.then_expression.accept(self);
        let else_branch = node.else_expression.accept(self);
        format!("if ({}) {{\n{}\n}} else {{\n{}\n}}", condition, then_branch, else_branch)
    }
    fn visit_let_in(&mut self,node: &LetInNode) -> String {
        let assignments: Vec<String> = node.assignments.iter()
            .map(|assignment| format!("{} = {}", assignment.identifier, assignment.expression.accept(self)))
            .collect();
        let body = node.body.accept(self);
        format!("let {} in {}", assignments.join(", "), body)
    }
    fn visit_destructive_assign(&mut self, node: &DestructiveAssignNode) -> String {
        let id = &node.identifier;
        let expr = node.expression.accept(self);
        format!("{} := {}", id, expr)
    }
}