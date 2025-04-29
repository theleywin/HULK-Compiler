use crate::tokens::*;

#[derive(Debug, PartialEq)]
pub enum Expr {
    Number(i32),
    Boolean(bool),
    Str(String),
    Identifier(String),
    BinaryOp(Box<Expr>, OperatorToken, Box<Expr>),
    UnaryOp(OperatorToken, Box<Expr>),
    Print(Box<Expr>),
}