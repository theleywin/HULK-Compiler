use super::context::CodeGenContext;
use super::expressions::*;
use super::llvm_utils::*;
use crate::ast_nodes::binary_op::BinaryOpNode;
use crate::ast_nodes::block::BlockNode;
use crate::ast_nodes::destructive_assign::DestructiveAssignNode;
use crate::ast_nodes::for_loop::ForNode;
use crate::ast_nodes::function_call::FunctionCallNode;
use crate::ast_nodes::function_def::FunctionDefNode;
use crate::ast_nodes::if_else::IfElseNode;
use crate::ast_nodes::let_in::LetInNode;
use crate::ast_nodes::literals::{
    BooleanLiteralNode, IdentifierNode, NumberLiteralNode, StringLiteralNode,
};
use crate::ast_nodes::program::{Program, Statement};
use crate::ast_nodes::type_def::TypeDefNode;
use crate::ast_nodes::type_instance::TypeInstanceNode;
use crate::ast_nodes::type_member_access::{TypeFunctionAccessNode, TypePropAccessNode};
use crate::ast_nodes::unary_op::UnaryOpNode;
use crate::ast_nodes::while_loop::WhileNode;
use crate::codegen::llvm_utils;
use crate::visitor::accept::Accept;
use crate::visitor::visitor_trait::Visitor;

pub struct CodeGenerator {
    context: CodeGenContext,
}

impl CodeGenerator {
    pub fn new() -> Self {
        Self {
            context: CodeGenContext::new(),
        }
    }

    pub fn generate(&mut self, program: &Program) -> String {
        // 1) Module header + declarations
        let mut module_code: Vec<String> = vec![];
        generate_header(&mut module_code);
        declare_printf(&mut module_code);
        module_code.push("".into());

        // 2) Build the body in its own context
        let mut body_context = CodeGenContext::new();
        // swap it in
        std::mem::swap(&mut self.context, &mut body_context);
        self.generate_body(program);
        let body_code = self.context.take_code();
        // restore original (empty) context if you need it later:
        std::mem::swap(&mut self.context, &mut body_context);

        // 3) Emit `define @main` into module_code
        generate_main_wrapper(&mut module_code, &body_code);

        // 4) Join and return
        module_code.join("\n")
    }

    fn generate_body(&mut self, program: &Program) {
        for statement in &program.statements {
            if let Statement::StatementExpression(expr) = statement {
                let result = expr.accept(self);
                llvm_utils::generate_printf(&mut self.context, &result);
            }
        }
    }
}

impl Visitor<String> for CodeGenerator {
    fn visit_literal_number(&mut self, node: &NumberLiteralNode) -> String {
        gen_number(&mut self.context, &node.value.to_string())
    }

    fn visit_binary_op(&mut self, node: &BinaryOpNode) -> String {
        let left_val = node.left.accept(self);
        let right_val = node.right.accept(self);
        gen_binary_op(
            &mut self.context,
            left_val,
            node.operator.clone(),
            right_val,
        )
    }

    fn visit_function_def(&mut self, _node: &FunctionDefNode) -> String {
        unimplemented!()
    }

    fn visit_literal_boolean(&mut self, _node: &BooleanLiteralNode) -> String {
        unimplemented!()
    }

    fn visit_literal_string(&mut self, _node: &StringLiteralNode) -> String {
        unimplemented!()
    }

    fn visit_identifier(&mut self, _node: &IdentifierNode) -> String {
        unimplemented!()
    }

    fn visit_function_call(&mut self, _node: &FunctionCallNode) -> String {
        unimplemented!()
    }

    fn visit_while_loop(&mut self, _node: &WhileNode) -> String {
        unimplemented!()
    }

    fn visit_for_loop(&mut self, _node: &ForNode) -> String {
        unimplemented!()
    }

    fn visit_code_block(&mut self, _node: &BlockNode) -> String {
        unimplemented!()
    }

    fn visit_unary_op(&mut self, node: &UnaryOpNode) -> String {
        let operand_val = node.operand.accept(self);
        gen_unary_op(&mut self.context, node.operator.clone(), operand_val)
    }

    fn visit_if_else(&mut self, _node: &IfElseNode) -> String {
        unimplemented!()
    }

    fn visit_let_in(&mut self, _node: &LetInNode) -> String {
        unimplemented!()
    }

    fn visit_destructive_assign(&mut self, _node: &DestructiveAssignNode) -> String {
        unimplemented!()
    }

    fn visit_type_def(&mut self, _node: &TypeDefNode) -> String {
        unimplemented!()
    }

    fn visit_type_instance(&mut self, _node: &TypeInstanceNode) -> String {
        unimplemented!()
    }

    fn visit_type_function_access(&mut self, _node: &TypeFunctionAccessNode) -> String {
        unimplemented!()
    }

    fn visit_type_prop_access(&mut self, _node: &TypePropAccessNode) -> String {
        unimplemented!()
    }
}
