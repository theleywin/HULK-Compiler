use super::code_generator::CodeGenerator;
use super::llvm_utils::{to_llvm_type};

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

use crate::tokens::OperatorToken;
use crate::visitor::accept::Accept;
use crate::visitor::visitor_trait::Visitor;

pub struct GeneratorResult {
    pub register: String,
    pub llvm_type: String,
}
impl GeneratorResult {
    pub fn new(register: String, llvm_type: String) -> Self {
        GeneratorResult { register, llvm_type }
    }
}

impl Visitor<GeneratorResult> for CodeGenerator {
    fn visit_function_def(&mut self, node: &mut FunctionDefNode) -> GeneratorResult {
        let function_name = node.name.clone();
        let params = node.params.clone();
        let return_type = node.return_type.clone();
        let function_global_name = format!("@{}", function_name);
        let llvm_args: Vec<String> = params.iter().map(|param| {
            let llvm_type = to_llvm_type(param.signature.clone());
            format!("{} %{}", llvm_type, param.name)
        }).collect();
        self.context.add_global_declaration(function_global_name.clone());
        self.context.add_line(format!("define {} {}({}) {{", to_llvm_type(return_type.clone()), function_global_name, llvm_args.join(", ")));
        let llvm_body = node.body.accept(self);
        self.context.add_line(format!("ret {} {}", llvm_body.llvm_type, llvm_body.register));
        self.context.add_line("}".to_string());
        GeneratorResult::new(function_global_name, to_llvm_type(return_type.clone()))
    }

    fn visit_literal_number(&mut self, node: &mut NumberLiteralNode) -> GeneratorResult {
        let mut raw = node.value.to_string();
        if !raw.contains('.') {
            raw.push_str(".0");
        }
        let temp = self.context.new_temp("Number".to_string());
        self.context
            .add_line(format!("{} = fadd double 0.0, {}", temp, raw));
        GeneratorResult::new(temp, "double".to_string())
    }

    fn visit_literal_boolean(&mut self, node: &mut BooleanLiteralNode) -> GeneratorResult {
        let value = if node.value { "1" } else { "0" };
        let temp = self.context.new_temp("Boolean".to_string());
        self.context
            .add_line(format!("{} = add i1 {}, 0", temp, value));
        GeneratorResult::new(temp, "i1".to_string())
    }

    fn visit_literal_string(&mut self, node: &mut StringLiteralNode) -> GeneratorResult {
        let global_name = self.context.add_string_literal(&node.value);
        let temp = self.context.new_temp("String".to_string());
        let len = node.value.len() + 1;
        self.context.add_line(format!(
            "{} = getelementptr inbounds [{} x i8], [{} x i8]* {}, i64 0, i64 0",
            temp, len, len, global_name
        ));
        GeneratorResult::new(temp, format!("[{} x i8]",len))
    }

    fn visit_identifier(&mut self, node: &mut IdentifierNode) -> GeneratorResult {
       let value = node.value.clone();
       let result = format!("%{}", value);
       GeneratorResult::new(result, to_llvm_type(node.node_type.clone().unwrap().type_name))
    }

    fn visit_function_call(&mut self,  node: &mut FunctionCallNode) -> GeneratorResult {
        let name = node.function_name.clone();
        let llvm_args: Vec<String> = node.arguments.iter().map(|arg| {
            let arg_val = arg.clone().accept(self);
            format!("{} %{}", arg_val.llvm_type, arg_val.register)
        }).collect();
        let node_type = to_llvm_type(node.node_type.clone().unwrap().type_name);
        let temp = self.context.new_temp(node_type.clone());
        self.context.add_line(format!(
            "{} = call {} @{}({})",
            temp, node_type, name, llvm_args.join(", ")
        ));
        GeneratorResult::new(temp, node_type)

    }

    fn visit_while_loop(&mut self, _node: &mut WhileNode) -> GeneratorResult {
        unimplemented!()
    }

    fn visit_for_loop(&mut self, _node: &mut ForNode) -> GeneratorResult {
        unimplemented!()
    }

    fn visit_code_block(&mut self, _node: &mut BlockNode) -> GeneratorResult {
        unimplemented!()
    }

    fn visit_binary_op(&mut self, node: &mut BinaryOpNode) -> GeneratorResult {
        let left_val = node.left.accept(self);
        let right_val = node.right.accept(self);
        let op = node.operator.clone();
        match op {
            OperatorToken::PLUS | OperatorToken::MINUS | OperatorToken::MUL | OperatorToken::DIV => {
                let opcode = match op {
                    OperatorToken::PLUS => "fadd",
                    OperatorToken::MINUS => "fsub",
                    OperatorToken::MUL => "fmul",
                    OperatorToken::DIV => "fdiv",
                    _ => unreachable!(),
                };

                let temp = self.context.new_temp("Number".to_string());
                self.context.add_line(format!(
                    "{} = {} double {}, {}",
                    temp, opcode, left_val.register, right_val.register
                ));
                GeneratorResult::new(temp, "Number".to_string())
            }

            OperatorToken::MOD => {
                if !self.context.runtime_functions.contains("fmod") {
                    self.context.add_global_declaration("declare double @fmod(double, double)".to_string());
                    self.context.runtime_functions.insert("fmod".to_string());
                }

                let temp = self.context.new_temp("Number".to_string());
                self.context.add_line(format!(
                    "{} = call double @fmod(double {}, double {})",
                    temp, left_val.register, right_val.register
                ));
                GeneratorResult::new(temp, "double".to_string())
            }

            OperatorToken::POW => {
                if !self.context.runtime_functions.contains("pow") {
                    self.context.add_global_declaration("declare double @pow(double, double)".to_string());
                    self.context.runtime_functions.insert("pow".to_string());
                }

                let temp = self.context.new_temp("Number".to_string());
                self.context.add_line(format!(
                    "{} = call double @pow(double {}, double {})",
                    temp, left_val.register, right_val.register
                ));
                GeneratorResult::new(temp, "double".to_string())
            }

            OperatorToken::EQ
            | OperatorToken::NEQ
            | OperatorToken::GT
            | OperatorToken::GTE
            | OperatorToken::LT
            | OperatorToken::LTE => {
                if self.context.get_type(&left_val.register) == "Boolean"
                    && self.context.get_type(&right_val.register) == "Boolean"
                {
                    let cmp_op = match op {
                        OperatorToken::EQ => "eq",
                        OperatorToken::NEQ => "ne",
                        _ => panic!("Invalid comparison operator for booleans: {:?}", op),
                    };

                    let temp = self.context.new_temp("Boolean".to_string());
                    self.context.add_line(format!(
                        "{} = icmp {} i1 {}, {}",
                        temp, cmp_op, left_val.register, right_val.register
                    ));
                    GeneratorResult::new(temp, "i1".to_string())
                } else {
                    let cmp_op = match op {
                        OperatorToken::EQ => "oeq",
                        OperatorToken::NEQ => "one",
                        OperatorToken::GT => "ogt",
                        OperatorToken::GTE => "oge",
                        OperatorToken::LT => "olt",
                        OperatorToken::LTE => "ole",
                        _ => unreachable!(),
                    };

                    let temp = self.context.new_temp("Boolean".to_string());
                    self.context.add_line(format!(
                        "{} = fcmp {} double {}, {}",
                        temp, cmp_op, left_val.register, right_val.register
                    ));
                    GeneratorResult::new(temp, "i1".to_string())
                }
            }

            OperatorToken::AND | OperatorToken::OR => {
                let opcode = match op {
                    OperatorToken::AND => "and",
                    OperatorToken::OR => "or",
                    _ => unreachable!(),
                };

                let temp = self.context.new_temp("Boolean".to_string());
                self.context.add_line(format!(
                    "{} = {} i1 {}, {}",
                    temp, opcode, left_val.register, right_val.register
                ));
                GeneratorResult::new(temp, "i1".to_string())
            }

            OperatorToken::CONCAT => {
                if self.context.get_type(&left_val.register) != "String"
                    || self.context.get_type(&right_val.register) != "String"
                {
                    panic!("Concatenation requires string operands");
                }

                if !self.context.runtime_functions.contains("concat") {
                    self.context.add_global_declaration("declare i8* @concat(i8*, i8*)".to_string());
                    self.context.runtime_functions.insert("concat".to_string());
                }

                let temp = self.context.new_temp("String".to_string());
                self.context.add_line(format!(
                    "{} = call i8* @concat(i8* {}, i8* {})",
                    temp, left_val.register, right_val.register
                ));
                GeneratorResult::new(temp, "i8*".to_string())
            }

            _ => panic!("Unsupported binary operator: {:?}", op),
        }
    }

    fn visit_unary_op(&mut self, node: &mut UnaryOpNode) -> GeneratorResult {
        let operand_val = node.operand.accept(self);
        let op = node.operator.clone();
        match op {
            OperatorToken::MINUS => {
                let temp = self.context.new_temp("Number".to_string());
                self.context.add_line(format!("{} = fsub double 0.0, {}", temp, operand_val.register));
                GeneratorResult::new(temp, "double".to_string())
            }
            OperatorToken::NOT => {
                let temp = self.context.new_temp("Boolean".to_string());
                self.context.add_line(format!("{} = xor i1 {}, true", temp, operand_val.register));
                GeneratorResult::new(temp, "i1".to_string())
            }
            _ => panic!("Unsupported unary operator: {:?}", op),
        }
    }

    fn visit_if_else(&mut self, _node: &mut IfElseNode) -> GeneratorResult {
        unimplemented!()
    }

    fn visit_let_in(&mut self, _node: &mut LetInNode) -> GeneratorResult {
        unimplemented!()
    }

    fn visit_destructive_assign(&mut self, _node: &mut DestructiveAssignNode) -> GeneratorResult {
        unimplemented!()
    }

    fn visit_type_def(&mut self, _node: &mut TypeDefNode) -> GeneratorResult {
        unimplemented!()
    }

    fn visit_type_instance(&mut self, _node: &mut TypeInstanceNode) -> GeneratorResult {
        unimplemented!()
    }

    fn visit_type_function_access(&mut self, _node: &mut TypeFunctionAccessNode) -> GeneratorResult {
        unimplemented!()
    }

    fn visit_type_prop_access(&mut self, _node: &mut TypePropAccessNode) -> GeneratorResult {
        unimplemented!()
    }
    
    fn visit_print(&mut self, _node: &mut crate::ast_nodes::print::PrintNode) -> GeneratorResult {
        unimplemented!()
    }
}
