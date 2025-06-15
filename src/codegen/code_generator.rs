use super::context::CodeGenContext;
use super::llvm_utils::*;
use crate::ast_nodes::program::{Program, Statement};
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
        generate_runtime_declarations(&mut module_code);
        module_code.push("".into());

        let mut body_context = CodeGenContext::new();
        std::mem::swap(&mut self.context, &mut body_context);
        
        let globals = self.context.take_globals();
        std::mem::swap(&mut self.context, &mut body_context);

        module_code.extend(globals);
        if !module_code.last().map(|s| s.is_empty()).unwrap_or(false) {
            module_code.push("".into());
        }
        module_code.extend(self.get_definitions(program));
        let main_code = &self.get_main_code(program);
        generate_main_wrapper(&mut module_code, &main_code , self.context.str_constants.clone());
        module_code.join("\n")
    }


    fn get_main_code(&mut self, program: &mut Program) -> Vec<String> {
        let main_code;
        for statement in &mut program.statements {
            match statement {
                Statement::StatementExpression(_) => {
                    statement.accept(self);
                }
                _ => continue,
            }
        }
        main_code = self.context.code.clone();
        self.context.code.clear();
        main_code
    }

    fn get_definitions(&mut self, program: &mut Program) -> Vec<String> {
        let definitions ;
        for statement in &mut program.statements {
            match statement {
                Statement::StatementTypeDef(_) => {
                    statement.accept(self);
                }
                Statement::StatementFunctionDef(_) => {
                    statement.accept(self);
                }
                _ => continue,
            }
        }
        definitions = self.context.code.clone();
        self.context.code.clear();
        definitions 
    }

}

fn generate_main_wrapper(module_code: &mut Vec<String>, body_code: &[String] , global_consts: Vec<String>) {
    for global_const in global_consts {
        module_code.push(global_const);
    }
    module_code.push("define i32 @main() {".into());
    module_code.push("entry:".into());
    for line in body_code {
        module_code.push("  ".to_string() + line);
    }
    module_code.push("  ret i32 0".into());
    module_code.push("}".into());
}
