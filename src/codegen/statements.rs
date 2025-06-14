use super::context::CodeGenContext;
use crate::ast_nodes::expression::Expression;
use crate::ast_nodes::program::Program;
use crate::visitor::accept::Accept;
use crate::visitor::visitor_trait::Visitor;

pub fn gen_expression<V: Visitor<String>>(expr: &mut Box<Expression>, visitor: &mut V) -> String {
    expr.accept(visitor)
}

pub fn gen_program<V: Visitor<String>>(
    _context: &mut CodeGenContext,
    program: &mut Program,
    visitor: &mut V,
) {
    for statement in &mut program.statements {
        statement.accept(visitor);
    }
}
