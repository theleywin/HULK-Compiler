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

    pub fn generate(&mut self, program: &mut Program) -> String {
        let mut module_code: Vec<String> = vec![];
        generate_header(&mut module_code);
        declare_printf(&mut module_code);
        module_code.push("".into());

        let mut body_context = CodeGenContext::new();
        std::mem::swap(&mut self.context, &mut body_context);
        self.generate_body(program);

        // Get globals and body code separately
        let globals = self.context.take_globals();
        let body_code = self.context.take_body();

        std::mem::swap(&mut self.context, &mut body_context);

        // Add globals to module
        module_code.extend(globals);
        if !module_code.last().map(|s| s.is_empty()).unwrap_or(false) {
            module_code.push("".into());
        }

        // Generate main wrapper with body code
        generate_main_wrapper(&mut module_code, &body_code);
        module_code.join("\n")
    }

    fn generate_body(&mut self, program: &mut Program) {
        for statement in &mut program.statements {
            if let Statement::StatementExpression(expr) = statement {
                let result = expr.accept(self);

                // Depending on the type, print appropriately
                if self.context.is_bool(&result) {
                    // Convert boolean (i1) to i32 for printf with %d
                    let i32_temp = self.context.new_temp();
                    self.context
                        .add_line(format!("{} = zext i1 {} to i32", i32_temp, result));
                    llvm_utils::generate_printf(&mut self.context, &i32_temp, "%d");
                } else if self.context.is_string(&result) {
                    llvm_utils::generate_printf(&mut self.context, &result, "%s");
                } else {
                    // It's a double
                    llvm_utils::generate_printf(&mut self.context, &result, "%f");
                }
            }
        }
    }
}

impl Visitor<String> for CodeGenerator {
    fn visit_literal_number(&mut self, node: &mut NumberLiteralNode) -> String {
        gen_number(&mut self.context, &node.value.to_string())
    }

    fn visit_binary_op(&mut self, node: &mut BinaryOpNode) -> String {
        let left_val = node.left.accept(self);
        let right_val = node.right.accept(self);
        gen_binary_op(
            &mut self.context,
            left_val,
            node.operator.clone(),
            right_val,
        )
    }

    fn visit_function_def(&mut self, _node: &mut FunctionDefNode) -> String {
        unimplemented!()
    }

    fn visit_literal_boolean(&mut self, node: &mut BooleanLiteralNode) -> String {
        let value = gen_boolean(&mut self.context, node.value);
        let temp = self.context.new_temp();
        // Store as i1
        self.context
            .add_line(format!("{} = add i1 {}, 0", temp, value));
        self.context.add_bool_var(temp.clone());
        temp
    }

    fn visit_literal_string(&mut self, node: &mut StringLiteralNode) -> String {
        // Create a global string constant and get a pointer to it
        let global_name = self.context.add_string_literal(&node.value);
        let temp = self.context.new_temp();
        let len = node.value.len() + 1; // +1 for null terminator
        self.context.add_line(format!(
            "{} = getelementptr inbounds [{} x i8], [{} x i8]* {}, i64 0, i64 0",
            temp, len, len, global_name
        ));
        self.context.add_string_var(temp.clone());
        temp
    }

    fn visit_identifier(&mut self, _node: &mut IdentifierNode) -> String {
        unimplemented!()
    }

    fn visit_function_call(&mut self, _node: &mut FunctionCallNode) -> String {
        unimplemented!()
    }

    fn visit_while_loop(&mut self, _node: &mut WhileNode) -> String {
        unimplemented!()
    }

    fn visit_for_loop(&mut self, _node: &mut ForNode) -> String {
        unimplemented!()
    }

    fn visit_code_block(&mut self, _node: &mut BlockNode) -> String {
        unimplemented!()
    }

    fn visit_unary_op(&mut self, node: &mut UnaryOpNode) -> String {
        let operand_val = node.operand.accept(self);
        gen_unary_op(&mut self.context, node.operator.clone(), operand_val)
    }

    fn visit_if_else(&mut self, _node: &mut IfElseNode) -> String {
        unimplemented!()
    }

    fn visit_let_in(&mut self, _node: &mut LetInNode) -> String {
        unimplemented!()
    }

    fn visit_destructive_assign(&mut self, _node: &mut DestructiveAssignNode) -> String {
        unimplemented!()
    }

    fn visit_type_def(&mut self, _node: &mut TypeDefNode) -> String {
        unimplemented!()
    }

    fn visit_type_instance(&mut self, _node: &mut TypeInstanceNode) -> String {
        unimplemented!()
    }

    fn visit_type_function_access(&mut self, _node: &mut TypeFunctionAccessNode) -> String {
        unimplemented!()
    }

    fn visit_type_prop_access(&mut self, _node: &mut TypePropAccessNode) -> String {
        unimplemented!()
    }
}
