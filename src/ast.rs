use crate::tokens::*;

//#[derive(Debug, PartialEq)]
// pub struct Program{
//     pub statements: Vec<Box<Statement>>,
// }

//#[derive(Debug, PartialEq)]
// pub enum Statement {
//     Expression(Expr),
//     FunctionDef(FuncDef),
// }

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
    Assignment(String, Box<Expr>),
    FunctionFullDef(FunctionDef),//cambiar
    FunctionArrowDef(FunctionDef),
    LetIn(Vec<Box<Expr>>, Box<Expr>),
    WhileLoop(Box<Expr>, Box<Expr>),
    IfElse(Box<Expr>, Box<Expr>, Box<Expr>),
    CodeBlock(Vec<Box<Expr>>),
}

//#[derive(Debug, PartialEq)]
// pub enum FuncDef{
//     FunctionFullDef(FunctionDef),
//     FunctionArrowDef(FunctionDef),
// }

#[derive(Debug, PartialEq)]
pub struct FunctionDef {
    pub name: String,
    pub params: Vec<String>,
    pub body: Box<Expr>,
}

