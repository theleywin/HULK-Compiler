use super::code_generator::CodeGenerator;
use super::llvm_utils::{to_llvm_type};
use crate::ast_nodes::binary_op::BinaryOpNode;
use crate::ast_nodes::block::BlockNode;
use crate::ast_nodes::destructive_assign::DestructiveAssignNode;
use crate::ast_nodes::expression::Expression;
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
use crate::ast_nodes::print::PrintNode;
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
        self.context.enter_scope();
        let llvm_args: Vec<String> = params.iter().map(|param| {
            let llvm_type = to_llvm_type(param.signature.clone());
            let register_name = format!("%{}.{}", param.name.clone(), self.context.get_scope());
            self.context.add_variable(register_name.clone(), llvm_type.clone());
            format!("ptr {}", register_name)
        }).collect();
        self.context.add_line(format!("define {} {}({}) {{", to_llvm_type(return_type.clone()), function_global_name, llvm_args.join(", ")));
        let llvm_body = node.body.accept(self);
        self.context.add_line(format!("ret {} {}", llvm_body.llvm_type, llvm_body.register));
        self.context.add_line("}".to_string());
        self.context.exit_scope();
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
        let temp = self.context.new_temp("String".to_string());
        let len = node.value.len();
        let global_const = self.context.add_str_const(node.value.clone(), len.clone());
        self.context.add_line(format!("{} = getelementptr [{} x i8], ptr {}, i32 0, i32 0",temp,len + 1,global_const));
        GeneratorResult::new(temp, "ptr".to_string())
    }

    fn visit_identifier(&mut self, node: &mut IdentifierNode) -> GeneratorResult {
       let value = node.value.clone();
       let llvm_type = to_llvm_type(node.node_type.clone().unwrap().type_name);
       let register = self.context.new_temp(llvm_type.clone());
        self.context.add_line(format!(
            "{} = load {}, ptr {}",
            register, llvm_type.clone(), self.context.get_variable(value)
        ));
       GeneratorResult::new(register, llvm_type.clone())
    }

    fn visit_function_call(&mut self,  node: &mut FunctionCallNode) -> GeneratorResult {
        let name = node.function_name.clone();
        let llvm_args: Vec<String> = node.arguments.iter().map(|arg| {
            let arg_val = arg.clone().accept(self);
            let id = self.context.new_id();
            self.context.add_line(format!("%{} = alloca {}", id.clone(), arg_val.llvm_type));
            self.context.add_line(format!(
                "store {} {}, ptr %{}",
                arg_val.llvm_type, arg_val.register, id.clone()
            ));
            format!("ptr %{}", id)
        }).collect();
        let node_type = to_llvm_type(node.node_type.clone().unwrap().type_name);
        let temp = self.context.new_temp(node_type.clone());
        self.context.add_line(format!(
            "{} = call {} @{}({})",
            temp, node_type, name, llvm_args.join(", ")
        ));
        GeneratorResult::new(temp, node_type)

    }

    fn visit_while_loop(&mut self, node: &mut WhileNode) -> GeneratorResult {
        let id_cond = self.context.new_id();
        let id_loop = self.context.new_id();
        let cond_label = format!("while_cond.{}",id_cond);
        let loop_label = format!("while_loop.{}",id_loop);
        let id_exit = self.context.new_id();
        let exit_label = format!("while_exit.{}",id_exit);
        let node_type = to_llvm_type(node.node_type.clone().unwrap().type_name);

        let result_reg = self.context.new_temp(node_type.clone());
        self.context.add_line(format!("{} = alloca {}", result_reg.clone() ,node_type.clone()));
        
        self.context.add_line(format!("br label %{}\n\n", cond_label));

        self.context.add_line(format!("{}:", cond_label));
        let cond_register = node.condition.accept(self);
        self.context.add_line(format!(
            "br i1 {}, label %{}, label %{}\n\n",
            cond_register.register, loop_label, exit_label
        ));
        self.context.add_line(format!("{}:", loop_label));
        let body_register = node.body.accept(self);
        self.context.add_line(format!(
            "store {} {}, ptr {}\n\n",
            node_type.clone(), body_register.register, result_reg.clone()
        ));
        self.context.add_line(format!("br label %{}\n\n", cond_label));

        self.context.add_line(format!("{}:", exit_label));
        let return_reg = self.context.new_temp(node_type.clone());
        self.context.add_line(format!(
            "{} = load {}, ptr {}\n",
            return_reg.clone(), node_type.clone(), result_reg.clone()
        ));
        GeneratorResult::new(return_reg, node_type)
    }

    fn visit_for_loop(&mut self, node: &mut ForNode) -> GeneratorResult {
        let start_reg = node.start.accept(self);
        let end_reg = node.end.accept(self);
        self.context.enter_scope();
        let for_condition_id = self.context.new_id();
        let for_condition_label = format!("for_condition.{}", for_condition_id); 
        let for_body_id = self.context.new_id();
        let for_body_label = format!("for_body.{}", for_body_id);
        let for_exit_id = self.context.new_id();
        let for_exit_label = format!("for_exit.{}", for_exit_id);
        let index_reg = format!("%{}.{}", node.variable, self.context.get_scope());
        let comp_reg = self.context.new_temp("Number".to_string());
        self.context.add_line(format!(
            "{} = fcmp ole double {}, {}",
            comp_reg, start_reg.register, end_reg.register
        ));
        let step_reg = self.context.new_temp("Number".to_string());
        self.context.add_line(format!(
            "{} = select i1 {}, double 1.0, double -1.0",
            step_reg, comp_reg
        ));
        self.context.add_variable(index_reg.clone(), start_reg.llvm_type.clone());
        self.context.add_line(format!(
            "{} = alloca {}",
            index_reg, start_reg.llvm_type
        )); 
        self.context.add_line(format!(
            "store {} {}, ptr {}",
            start_reg.llvm_type, start_reg.register, index_reg
        ));

        self.context.add_line(format!("br label %{}\n\n", for_condition_label));
        self.context.add_line(format!("{}:", for_condition_label));
        let curr = self.context.new_temp("Number".to_string());
        self.context.add_line(format!(
            "{} = load {}, ptr {}\n",
            curr, start_reg.llvm_type, index_reg
        ));
        let comp_up = self.context.new_temp("Number".to_string());
        self.context.add_line(format!(
            "{} = fcmp ole double {}, {}",
            comp_up, curr, end_reg.register
        ));
        let comp_down = self.context.new_temp("Number".to_string());
        self.context.add_line(format!(
            "{} = fcmp oge double {}, {}",
            comp_down, curr, end_reg.register
        ));
        let condition = self.context.new_temp("Boolean".to_string());
        self.context.add_line(format!(
            "{} = select i1 {}, i1 {}, i1 {}",
            condition, comp_reg ,comp_up, comp_down
        ));
        self.context.add_line(format!(
            "br i1 {}, label %{}, label %{}\n\n",
            condition, for_body_label, for_exit_label
        ));
        self.context.add_line(format!("{}:", for_body_label));
        let body_result = node.body.accept(self);
        let step_val = self.context.new_temp("Number".to_string());
        self.context.add_line(format!(
            "{} = fadd double {}, {}\n",
            step_val, curr, step_reg
        ));
        self.context.add_line(format!(
            "store double {}, ptr {}\n\n",
            step_val, index_reg
        ));
        self.context.add_line(format!("br label %{}\n\n", for_condition_label));
        self.context.add_line(format!("{}:", for_exit_label));
        GeneratorResult::new(body_result.register, body_result.llvm_type)
    }

    fn visit_code_block(&mut self, node: &mut BlockNode) -> GeneratorResult {
        self.context.enter_scope();
        let mut result = GeneratorResult::new("".to_string(), "".to_string());
        for expr in node.expression_list.expressions.iter_mut() {
            let current = expr.accept(self);
            result = current;
        }
        result
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
                GeneratorResult::new(temp, "double".to_string())
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

    fn visit_let_in(&mut self, node: &mut LetInNode) -> GeneratorResult {
        self.context.enter_scope();
        for assig in node.assignments.clone().iter_mut() {
            let identifier = assig.identifier.clone();
            let body = assig.expression.accept(self);
            let llvm_type = body.llvm_type.clone();
            let register_name = format!("%{}.{}", identifier, self.context.get_scope());
            self.context.add_variable(register_name.clone(), llvm_type.clone());
            self.context.add_line(format!(
                "{} = alloca {}",
                register_name.clone(), llvm_type
            ));
            self.context.add_line(format!(
                "store {} {}, ptr {}",
                llvm_type, body.register, register_name
            ));
        }
        let body_result = node.body.accept(self);
        self.context.exit_scope();
        GeneratorResult::new(body_result.register, body_result.llvm_type)
    }

    fn visit_destructive_assign(&mut self, node: &mut DestructiveAssignNode) -> GeneratorResult {
        let expr_result = node.expression.accept(self);
        match *node.identifier.clone() {
            Expression::Identifier(id) => {
                let identifier_register = self.context.get_variable(id.value.clone());
                self.context.add_line(format!(
                    "store {} {}, ptr {}",
                    expr_result.llvm_type, expr_result.register, identifier_register
                ));
            }
            _ => todo!() // Prop access 
        }
        GeneratorResult::new(expr_result.register, expr_result.llvm_type)
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
    
    fn visit_print(&mut self, node: &mut PrintNode) -> GeneratorResult {
        let arg = node.expression.accept(self);
        let id = self.context.new_id();
        if arg.llvm_type == "i1" {
            self.context.add_line(format!("%bool_ptr{} = select i1 {}, ptr @.true_str, ptr @.false_str", id ,arg.register));
            self.context.add_line(format!("call i32 (ptr, ...) @printf(ptr %bool_ptr{})", id));
        } else if arg.llvm_type == "double" {
            self.context.add_line(format!("%fmt_dbl_ptr{} = getelementptr [4 x i8], ptr @.str.f, i32 0, i32 0", id));
            self.context.add_line(format!("call i32 (ptr, ...) @printf(ptr %fmt_dbl_ptr{}, double {})", id, arg.register));
        } else if arg.llvm_type == "ptr" {
            self.context.add_line(format!("call i32 (ptr, ...) @printf(ptr {})", arg.register));
            // match node.expression.as_ref() {
            //     Expression::Str(string_lit) => {
                    
            //     }
            //     _ => {
            //         panic!("Unsupported expression type for print: {:?}", node.expression);
            //     } 
            // }
        } else {
            panic!("Unsupported expression type for print: {:?}", node.expression);
        }
        // match node.expression.as_ref() {
        //     Expression::Boolean(_) => {
        //         self.context.add_line(format!("%bool_ptr{} = select i1 {}, ptr @.true_str, ptr @.false_str", id ,arg.register));
        //         self.context.add_line(format!("call i32 (ptr, ...) @printf(ptr %sbool_ptr{})", id));
        //     }
        //     Expression::Number(_) => {
        //         self.context.add_line(format!("%fmt_dbl_ptr{} = getelementptr [4 x i8], ptr @.str.f, i32 0, i32 0", id));
        //         self.context.add_line(format!("call i32 (ptr, ...) @printf(ptr %fmt_dbl_ptr{}, double {})", id, arg.register));
        //     }
        //     Expression::Str(string_lit) => {
        //         let len = string_lit.value.len();
        //         let global_const = self.context.add_str_const(string_lit.value.clone(), len.clone() );
        //         self.context.add_line(format!("%str_ptr{} = getelementptr [{} x i8], ptr {}, i32 0, i32 0",id,len + 1,global_const));
        //         self.context.add_line(format!("call i32 (ptr, ...) @printf(ptr %str_ptr{})",id));
        //     }
        //     _ => {
        //         panic!("Unsupported expression type for print: {:?}", node.expression);
        //     }
        // }
        GeneratorResult::new(arg.register, arg.llvm_type)
    }
}
