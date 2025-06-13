use super::code_generator::CodeGenerator;
use super::context::Type;
use super::expressions::{gen_binary_op, gen_unary_op};

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
use crate::ast_nodes::type_def::TypeDefNode;
use crate::ast_nodes::type_instance::TypeInstanceNode;
use crate::ast_nodes::type_member_access::{TypeFunctionAccessNode, TypePropAccessNode};
use crate::ast_nodes::unary_op::UnaryOpNode;
use crate::ast_nodes::while_loop::WhileNode;

use crate::visitor::accept::Accept;
use crate::visitor::visitor_trait::Visitor;

impl Visitor<String> for CodeGenerator {
    fn visit_function_def(&mut self, _node: &mut FunctionDefNode) -> String {
        unimplemented!()
    }

    fn visit_literal_number(&mut self, node: &mut NumberLiteralNode) -> String {
        let mut raw = node.value.to_string();
        if !raw.contains('.') {
            raw.push_str(".0");
        }
        let temp = self.context.new_temp(Type::Double);
        self.context
            .add_line(format!("{} = fadd double 0.0, {}", temp, raw));
        temp
    }

    fn visit_literal_boolean(&mut self, node: &mut BooleanLiteralNode) -> String {
        let value = if node.value { "1" } else { "0" };
        let temp = self.context.new_temp(Type::Boolean);
        self.context
            .add_line(format!("{} = add i1 {}, 0", temp, value));
        temp
    }

    fn visit_literal_string(&mut self, node: &mut StringLiteralNode) -> String {
        let global_name = self.context.add_string_literal(&node.value);
        let temp = self.context.new_temp(Type::String);
        let len = node.value.len() + 1;
        self.context.add_line(format!(
            "{} = getelementptr inbounds [{} x i8], [{} x i8]* {}, i64 0, i64 0",
            temp, len, len, global_name
        ));
        temp
    }

    fn visit_identifier(&mut self, _node: &mut IdentifierNode) -> String {
        panic!("Identifiers not supported by this code generator");
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
    
    fn visit_print(&mut self, _node: &mut crate::ast_nodes::print::PrintNode) -> String {
        unimplemented!()
    }
}
