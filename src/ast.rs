use crate::tokens::*;

#[derive(Debug, PartialEq)]
pub enum Expr {
    Number(f64),
    Boolean(bool),
    Str(String),
    Identifier(String),
    BinaryOp(Box<Expr>, OperatorToken, Box<Expr>),
    UnaryOp(OperatorToken, Box<Expr>),
    Print(Box<Expr>),
    FunctionCall(String, Vec<Box<Expr>>),
}

#[derive(Debug, PartialEq)]
pub struct FunctionDef {
    pub name: String,
    pub params: Vec<String>,
    pub body: Box<Expr>,
}

#[derive(Debug, PartialEq)]
pub struct Program {
    pub functions: Vec<FunctionDef>,
    pub main: Vec<Box<Expr>>,
}
