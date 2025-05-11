use crate::tokens::*;


#[derive(Debug, PartialEq)]
pub enum Expression {
    Number(f64),
    Boolean(bool),
    Str(String),
    Identifier(String),
    Print(Box<Expr>),
    FunctionCall(Box<FunctionCall>),
    WhileLoop(Box<Expr>, Box<Expr>),
    IfElse(Box<Expr>, Box<Expr>, Box<Expr>),
    CodeBlock(Vec<Box<Expr>>),
    BinaryOp(Box<BinaryOp>),
    UnaryOp(Box<UnaryOp>),
    IfElse(Box<IfElse>),
    LetIn(Vec<Assignment>, Box<Expression>),
}
