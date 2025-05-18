use crate::visitor::accept::Accept;
use super::function_def::FunctionDefNode;
use super::expression::Expression;
use crate::visitor::visitor_trait::Visitor;

#[derive(Debug, PartialEq)]
pub struct Program{
    pub statements: Vec<Statement>,
}

#[derive(Debug, PartialEq)]
pub enum Statement {
    StatementExpression(Box<Expression>),
    StatementFunctionDef(Box<FunctionDefNode>),
}

impl Statement {
    pub fn new_expression(expression: Expression) -> Self {
        Statement::StatementExpression(Box::new(expression))
    }

    pub fn new_function_def(func_def: FunctionDefNode) -> Self {
        Statement::StatementFunctionDef(Box::new(func_def))
    }
}

impl Accept for Program {
    fn accept<V: Visitor<T>,T>(&self, visitor: &mut V) -> T {
        visitor.visit_program(self)
    }
}

impl Accept for Statement {
    fn accept<V: Visitor<T>,T>(&self, visitor: &mut V) -> T {
        match self {
            Statement::StatementExpression(expr) => expr.accept(visitor),
            Statement::StatementFunctionDef(node) => visitor.visit_function_def(node),
        }
    }
}

