use super::context::{CodeGenContext, Type};
use super::llvm_utils::*;
use crate::ast_nodes::program::{Program, Statement};
use crate::codegen::llvm_utils;
use crate::visitor::accept::Accept;

pub struct CodeGenerator {
    pub(crate) context: CodeGenContext,
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

        let globals = self.context.take_globals();
        let body_code = self.context.take_body();
        std::mem::swap(&mut self.context, &mut body_context);

        module_code.extend(globals);
        if !module_code.last().map(|s| s.is_empty()).unwrap_or(false) {
            module_code.push("".into());
        }

        generate_main_wrapper(&mut module_code, &body_code);
        module_code.join("\n")
    }

    fn generate_body(&mut self, program: &mut Program) {
        for statement in &mut program.statements {
            if let Statement::StatementExpression(expr) = statement {
                let result = expr.accept(self);

                let ty = self.context.get_type(&result);
                match ty {
                    Type::Boolean => {
                        let i32_temp = self.context.new_temp(Type::Double);
                        self.context
                            .add_line(format!("{} = zext i1 {} to i32", i32_temp, result));
                        llvm_utils::generate_printf(&mut self.context, &i32_temp, "%d");
                    }
                    Type::String => {
                        llvm_utils::generate_printf(&mut self.context, &result, "%s");
                    }
                    Type::Double => {
                        llvm_utils::generate_printf(&mut self.context, &result, "%f");
                    }
                }
            }
        }
    }
}

fn generate_main_wrapper(module_code: &mut Vec<String>, body_code: &[String]) {
    module_code.push("define i32 @main() {".into());
    module_code.push("entry:".into());
    for line in body_code {
        module_code.push("  ".to_string() + line);
    }
    module_code.push("  ret i32 0".into());
    module_code.push("}".into());
}
