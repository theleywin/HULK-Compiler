use super::function_def::FunctionDefNode;
use super::expression::Expression;

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