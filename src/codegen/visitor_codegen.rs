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
        self.context.string_literals.insert(temp.clone(), node.value.clone());
        self.context.add_line(format!("{} = getelementptr [{} x i8], ptr {}, i32 0, i32 0",temp,len + 1,global_const));
        GeneratorResult::new(temp, "ptr".to_string())
    }

    fn visit_identifier(&mut self, node: &mut IdentifierNode) -> GeneratorResult {
        let value = node.value.clone();
        let llvm_type = to_llvm_type(node.node_type.clone().unwrap().type_name);
        if self.context.is_global_constant(&value) {

            let register = self.context.new_temp(llvm_type.clone());
            let global_ref = format!("@{}", value);
            self.context.add_line(format!(
                "{} = load {}, ptr {}",register, llvm_type, global_ref // Usar directamente el nombre (@PI, @E)
            ));
             GeneratorResult::new(register, llvm_type)
        } 
        else {
            let register = self.context.new_temp(llvm_type.clone());
            self.context.add_line(format!(
                 "{} = load {}, ptr {}", register, llvm_type.clone(), self.context.get_variable(value)
             ));
             GeneratorResult::new(register, llvm_type.clone())
        }
    }

    fn visit_function_call(&mut self,  node: &mut FunctionCallNode) -> GeneratorResult {
        let name = node.function_name.clone();
        let llvm_args: Vec<String> = node.arguments.iter().map(|arg| {
            let arg_val = arg.clone().accept(self);
            let arg_reg = self.context.new_temp(arg_val.llvm_type.clone());
            self.context.add_line(format!("{} = alloca {}", arg_reg.clone(), arg_val.llvm_type));
            self.context.add_line(format!(
                "store {} {}, ptr {}",
                arg_val.llvm_type, arg_val.register, arg_reg.clone()
            ));
            format!("ptr {}", arg_reg)
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
                let len1 = self.context.new_temp("Number".to_string());
                let len2 = self.context.new_temp("Number".to_string());
                let total_len = self.context.new_temp("Number".to_string());
                let total_len_plus_one = self.context.new_temp("Number".to_string());
                let result_ptr = self.context.new_temp("String".to_string());
                let copy_reg = self.context.new_temp("ptr".to_string());
                let result = self.context.new_temp("String".to_string());
                self.context.add_line(
                    format!(
                        "{} = call i32 @strlen(ptr {})\n
                        {} = call i32 @strlen(ptr {})\n
                        {} = add i32 {}, {}\n
                        {} = add i32 {}, 1\n
                        {} = call ptr @malloc(i32 {})\n
                        {} = call ptr @strcpy(ptr {}, ptr {})\n 
                        {} = call ptr @strcat(ptr {}, ptr {})\n
                        ",len1,left_val.register,len2, right_val.register,
                        total_len,len1,len2,total_len_plus_one,total_len,
                        result_ptr,total_len_plus_one,copy_reg,result_ptr,left_val.register,
                        result,result_ptr,right_val.register
                    )
                );
                GeneratorResult::new(result, "ptr".to_string())
            }

            _ => panic!("Unsupported binary operator: {:?}", op),
        }
    }

    fn visit_unary_op(&mut self, node: &mut UnaryOpNode) -> GeneratorResult {
        let operand_val = node.operand.accept(self);
        let op = node.operator.clone();
        match op {
            OperatorToken::NEG => {
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

    fn visit_if_else(&mut self, node: &mut IfElseNode) -> GeneratorResult {
        let node_type = node.node_type.clone().unwrap().type_name;
        let node_type = to_llvm_type(node_type.clone());
        let result_reg = self.context.new_temp(node_type.clone());
        let exit_id = self.context.new_id();
        let exit_label = format!("if_else_exit.{}", exit_id);
        self.context.add_line(format!("{} = alloca {}", result_reg, node_type));
        let cond_reg = node.condition.accept(self);
        let if_id = self.context.new_id();
        let if_true_label = format!("if_true.{}", if_id);
        let if_false_label = format!("if_false.{}", if_id);
        self.context.add_line(format!(
            "br i1 {}, label %{}, label %{}",
            cond_reg.register, if_true_label, if_false_label
        ));
        self.context.add_line(format!("{}:", if_true_label));
        let if_expr = node.if_expression.accept(self);
        self.context.add_line(format!(
            "store {} {}, ptr {}\n",
            node_type, if_expr.register, result_reg
        ));
        self.context.add_line(format!("br label %{}\n\n", exit_label));
        self.context.add_line(format!("{}:", if_false_label));
        if node.elifs.len() > 0 {
            for (cond, expr) in node.elifs.iter_mut() {
                let elif_id = self.context.new_id();
                let elif_label = format!("elif_true.{}", elif_id);
                let elif_false_label = format!("elif_false.{}", elif_id);
                let elif_cond_reg = if let Some(cond_expr) = cond {
                    cond_expr.accept(self)
                } else {
                    let else_reg = self.context.new_temp("Boolean".to_string());
                    self.context.add_line(format!(
                        "{} = add i1 0, 1", else_reg
                    ));
                    GeneratorResult::new(else_reg, "i1".to_string())
                };
                if let Some(_) = cond {
                    self.context.add_line(format!(
                        "br i1 {}, label %{}, label %{}",
                        elif_cond_reg.register, elif_label, elif_false_label
                    ));
                } else {
                    self.context.add_line(format!("br label %{}\n\n", elif_label));
                }
                self.context.add_line(format!("{}:", elif_label));
                let elif_expr = expr.clone().accept(self);
                self.context.add_line(format!(
                    "store {} {}, ptr {}\n",
                    node_type, elif_expr.register, result_reg
                ));
                if let Some(_) = cond {
                    self.context.add_line(format!("br label %{}\n\n", exit_label));
                }
                
                if let Some(_) = cond {
                    self.context.add_line(format!("{}:", elif_false_label));
                } else {
                    self.context.add_line(format!("br label %{}\n\n", exit_label));
                }
            }
        } else {
            self.context.add_line(format!("br label %{}\n\n", exit_label));
        }
        self.context.add_line(format!("{}:", exit_label));
        let final_result = self.context.new_temp(node_type.clone());
        self.context.add_line(format!(
            "{} = load {}, ptr {}\n",
            final_result, node_type, result_reg
        ));
        GeneratorResult::new(final_result, node_type)
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
        let call_reg = self.context.new_temp("i32".to_string());
        let new_line_reg = self.context.new_temp("ptr".to_string());
        let call_new_line = self.context.new_temp("ptr".to_string());
        let id = self.context.new_id();
        let new_line = format!("{} = getelementptr [2 x i8], ptr @.newline, i32 0, i32 0", new_line_reg);
        let new_line2 = format!("{} = call i32 (ptr, ...) @printf(ptr {})", call_new_line, new_line_reg);
        if arg.llvm_type == "i1" {
            self.context.add_line(format!("%bool_ptr{} = select i1 {}, ptr @.true_str, ptr @.false_str", id ,arg.register));
            self.context.add_line(format!("{} = call i32 (ptr, ...) @printf(ptr %bool_ptr{})", call_reg,id));
            self.context.add_line(new_line);
            self.context.add_line(new_line2);
        } else if arg.llvm_type == "double" {
            self.context.add_line(format!("%fmt_dbl_ptr{} = getelementptr [4 x i8], ptr @.str.f, i32 0, i32 0", id)); 
            self.context.add_line(format!("{} = call i32 (ptr, ...) @printf(ptr %fmt_dbl_ptr{}, double {})", call_reg, id, arg.register));
            self.context.add_line(new_line);
            self.context.add_line(new_line2);
        } else if arg.llvm_type == "ptr" {
            self.context.add_line(format!("{} = call i32 (ptr, ...) @printf(ptr {})", call_reg, arg.register));
            self.context.add_line(new_line);
            self.context.add_line(new_line2);
        } else {
            panic!("Unsupported expression type for print: {:?}", node.expression);
        }
        GeneratorResult::new(arg.register, arg.llvm_type)
    }

}
