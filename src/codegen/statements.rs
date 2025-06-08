use super::context::CodeGenContext;
use crate::ast_nodes::expression::Expression;
use crate::ast_nodes::program::{Program, Statement};
use crate::visitor::accept::Accept;
use crate::visitor::visitor_trait::Visitor;

pub fn gen_expression<V: Visitor<String>>(expr: &Expression, visitor: &mut V) -> String {
    expr.accept(visitor)
}

pub fn gen_program<V: Visitor<String>>(
    _context: &mut CodeGenContext,
    program: &Program,
    visitor: &mut V,
) {
    for statement in &program.statements {
        if let Statement::StatementExpression(expr) = statement {
            let _result = gen_expression(expr, visitor);
        }
    }
}
