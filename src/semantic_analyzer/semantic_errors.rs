use crate::tokens::{OperatorToken, Span};
use crate::types_tree::tree_node::TypeNode;

#[derive(Debug, Clone, PartialEq)]
pub enum SemanticError {
    DivisionByZero(Span),
    UndefinedIdentifier(String, Span),
    InvalidConditionType(TypeNode, Span),
    InvalidBinaryOperation(TypeNode, TypeNode, OperatorToken, Span),
    InvalidUnaryOperation(TypeNode, OperatorToken, Span),
    RedefinitionOfFunction(String, Span),
    UndeclaredFunction(String, Span),
    UnknownError(String, Span),
    InvalidArgumentsCount(usize, usize, String, Span),
    InvalidTypeArgument(String, String, String, usize, String, Span),
    InvalidFunctionReturn(TypeNode, TypeNode, String, Span),
    RedefinitionOfVariable(String, Span),
    UndefinedType(String, Span),
    ParamNameAlreadyExist(String, String, String, Span),
    RedefinitionOfType(String, Span),
    CycleDetected(String, Span), // Fixed spelling: Cycle instead of Cicle
    InvalidTypeArgumentCount(usize, usize, String, Span),
    InvalidTypeFunctionAccess(String, String, Span),
    InvalidTypePropertyAccess(String, String, Span),
    InvalidTypeProperty(String, String, Span),
    InvalidPrint(String, Span),
    InvalidIterable(String, usize, Span),
}

impl SemanticError {
    pub fn message(&self) -> String {
        match self {
            SemanticError::DivisionByZero(_) => "Division by zero is not allowed".to_string(),
            SemanticError::UndefinedIdentifier(id, _) => {
                format!("Undefined identifier: {id}")
            }
            SemanticError::InvalidConditionType(t, _) => {
                format!("Invalid condition type: {}", t.type_name)
            }
            SemanticError::InvalidBinaryOperation(l, r, op, _) => format!(
                "Invalid binary operation between types {} and {} with operator {}",
                l.type_name, r.type_name, op
            ),
            SemanticError::InvalidUnaryOperation(t, op, _) => format!(
                "Invalid unary operation on type {} with operator {}",
                t.type_name, op
            ),
            SemanticError::RedefinitionOfFunction(name, _) => {
                format!("Function '{name}' is already defined")
            }
            SemanticError::UndeclaredFunction(name, _) => {
                format!("Function '{name}' is not defined")
            }
            SemanticError::InvalidArgumentsCount(found, expected, fname, _) => {
                format!("Function '{fname}' expects {expected} arguments, found {found}")
            }
            SemanticError::InvalidTypeArgument(_, found, expected, pos, stmt_name, _) => {
                format!(
                    "{stmt_name}: Argument {} should be {expected}, found {found}",
                    pos + 1
                )
            }
            SemanticError::InvalidFunctionReturn(body, ret, fname, _) => format!(
                "Function '{fname}' should return {}, found {}",
                ret.type_name, body.type_name
            ),
            SemanticError::RedefinitionOfVariable(var, _) => {
                format!("Variable '{var}' is already defined")
            }
            SemanticError::UndefinedType(ty, _) => {
                format!("Type '{ty}' is not defined")
            }
            SemanticError::ParamNameAlreadyExist(param, stmt_name, kind, _) => {
                format!("Duplicate parameter '{param}' in {kind} '{stmt_name}'")
            }
            SemanticError::RedefinitionOfType(ty, _) => {
                format!("Type '{ty}' is already defined")
            }
            SemanticError::CycleDetected(node, _) => {
                format!("Type dependency cycle detected: {node}")
            }
            SemanticError::InvalidTypeArgumentCount(found, expected, ty, _) => {
                format!("Type '{ty}' expects {expected} arguments, found {found}")
            }
            SemanticError::InvalidTypeFunctionAccess(ty, fn_name, _) => {
                format!("Type '{ty}' has no method '{fn_name}'")
            }
            SemanticError::InvalidTypePropertyAccess(ty, prop, _) => {
                format!("Property '{prop}' of type '{ty}' is private")
            }
            SemanticError::InvalidTypeProperty(ty, prop, _) => {
                format!("Type '{ty}' has no property '{prop}'")
            }
            SemanticError::InvalidPrint(ty, _) => {
                format!("Cannot print values of type '{ty}'")
            }
            SemanticError::InvalidIterable(fn_name, cnt, _) => {
                format!("For loops require range() function, found '{fn_name}({cnt} arguments)'")
            }
            SemanticError::UnknownError(msg, _) => msg.clone(),
        }
    }

    fn span(&self) -> &Span {
        match self {
            SemanticError::DivisionByZero(sp)
            | SemanticError::UndefinedIdentifier(_, sp)
            | SemanticError::InvalidConditionType(_, sp)
            | SemanticError::InvalidBinaryOperation(_, _, _, sp)
            | SemanticError::InvalidUnaryOperation(_, _, sp)
            | SemanticError::RedefinitionOfFunction(_, sp)
            | SemanticError::UndeclaredFunction(_, sp)
            | SemanticError::UnknownError(_, sp)
            | SemanticError::InvalidArgumentsCount(_, _, _, sp)
            | SemanticError::InvalidTypeArgument(_, _, _, _, _, sp)
            | SemanticError::InvalidFunctionReturn(_, _, _, sp)
            | SemanticError::RedefinitionOfVariable(_, sp)
            | SemanticError::UndefinedType(_, sp)
            | SemanticError::ParamNameAlreadyExist(_, _, _, sp)
            | SemanticError::RedefinitionOfType(_, sp)
            | SemanticError::CycleDetected(_, sp)
            | SemanticError::InvalidTypeArgumentCount(_, _, _, sp)
            | SemanticError::InvalidTypeFunctionAccess(_, _, sp)
            | SemanticError::InvalidTypePropertyAccess(_, _, sp)
            | SemanticError::InvalidTypeProperty(_, _, sp)
            | SemanticError::InvalidPrint(_, sp)
            | SemanticError::InvalidIterable(_, _, sp) => sp,
        }
    }

    pub fn report(&self, input: &str, missplacement: i32) -> String {
        let span = self.span();
        let (line, col, line_str, _) = get_line_context(input, span.start, missplacement);
        let caret = build_caret_point(col);

        let message = self.message();
        let location = format!("(line {line}, column {col})");

        format!(
            "\x1b[31mError {location}: {message}\n  {}\n  {}\x1b[0m",
            line_str, caret
        )
    }
}

fn get_line_context(
    input: &str,
    offset: usize,
    missplacement: i32,
) -> (usize, usize, String, usize) {
    if input.is_empty() {
        return (1, 1, String::new(), 0);
    }
    let mut line_start = 0;
    let mut line_number = 1;
    for (idx, c) in input.char_indices() {
        if idx > offset {
            break;
        }
        if c == '\n' {
            line_number += 1;
            line_start = idx + 1;
        }
    }
    let rest = &input[line_start..];
    let line_end = rest
        .find('\n')
        .map(|p| line_start + p)
        .unwrap_or(input.len());
    let line_str = input[line_start..line_end].to_string();

    let byte_in_line = offset.saturating_sub(line_start);
    let chars_before = input[line_start..line_start + byte_in_line].chars().count();
    let column = chars_before + 1;
    let adjusted_line = (line_number as i32 - missplacement).max(1) as usize;

    (adjusted_line, column, line_str, line_start)
}

fn build_caret_point(col: usize) -> String {
    " ".repeat(col.saturating_sub(1)) + "^"
}
