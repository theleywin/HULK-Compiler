
use crate::tokens::*;

#[derive(Debug, PartialEq)]
pub enum Expr {
    Number(i32),
    BinaryOp(Box<Expr>, OperatorToken, Box<Expr>),
    Print(Box<Expr>)
}