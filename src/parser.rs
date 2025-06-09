use crate::tokens::Token;
use crate::ast_nodes::program::{Program, Statement};
use crate::ast_nodes::expression::Expression;
use crate::ast_nodes::let_in::Assignment;
use crate::ast_nodes::function_def::{FunctionDefNode, FunctionParams};
use crate::ast_nodes::block::ExpressionList;
use crate::tokens::OperatorToken;
use lalrpop_util::ParseError;

pub type ParseErrorType = ParseError<usize, Token, String>;

pub fn format_error(error: ParseErrorType) -> String {
    match error {
        ParseError::InvalidToken { location } =>
            format!("Token inválido en posición {}", location),
        ParseError::UnrecognizedToken { token: Some((l, token, _)), expected } =>
            format!("Token inesperado '{:?}' en posición {}. Se esperaba: {}", token, l, expected.join(", ")),
        ParseError::ExtraToken { token: (l, token) } =>
            format!("Token extra '{:?}' en posición {}", token, l),
        ParseError::User { error } =>
            format!("Error de sintaxis: {}", error),
        _ => format!("Error desconocido: {:?}", error)
    }
}

pub fn extract_identifier(token: Token) -> Result<String, String> {
    match token {
        Token::Identifier(s) => Ok(s),
        _ => Err("Se esperaba un identificador".into()),
    }
}

pub fn extract_number(token: Token) -> Result<String, String> {
    match token {
        Token::Number(s) => Ok(s),
        _ => Err("Se esperaba un número".into()),
    }
}

pub fn extract_string(token: Token) -> Result<String, String> {
    match token {
        Token::StringLiteral(s) => Ok(s),
        _ => Err("Se esperaba una cadena".into()),
    }
}

pub fn extract_signature(token: Token) -> Result<String, String> {
    extract_identifier(token)
}

pub fn extract_operator(token: Token) -> Result<OperatorToken, String> {
    match token {
        Token::Operator(op) => Ok(op),
        _ => Err("Se esperaba un operador".into()),
    }
}