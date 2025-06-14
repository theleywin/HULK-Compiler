use std::collections::HashSet;

use crate::ast_nodes::program::Program;
use lalrpop_util::ParseError;

use crate::parser::ProgramParser;

pub struct Parser {
    core: ProgramParser,
}
impl Parser {
    pub fn new() -> Self {
        Parser {
            core: ProgramParser::new(),
        }
    }

    fn offset_to_line_col(input: &str, offset: usize) -> (usize, usize) {
        let mut line = 1;
        let mut col = 1;
        let mut current_offset = 0;

        for c in input.chars() {
            if current_offset >= offset {
                break;
            }

            if c == '\n' {
                line += 1;
                col = 1;
            } else {
                col += 1;
            }

            current_offset += c.len_utf8();
        }

        (line, col)
    }

    fn token_to_human_readable(token: &str) -> String {
        // Named Tokens
        match token {
            // Keywords
            "Function" => "`function`".to_string(),
            "Type" => "`type`".to_string(),
            "Let" => "`let`".to_string(),
            "In" => "`in`".to_string(),
            "If" => "`if`".to_string(),
            "Else" => "`else`".to_string(),
            "Elif" => "`elif`".to_string(),
            "While" => "`while`".to_string(),
            "For" => "`for`".to_string(),
            "Inherits" => "`inherits`".to_string(),
            "New" => "`new`".to_string(),
            "Print" => "`print`".to_string(),
            "True" => "`true`".to_string(),
            "False" => "`false`".to_string(),

            // Operators
            "Assign" => "`=`".to_string(),
            "DestructiveAssignOp" => "`:=`".to_string(),
            "Arrow" => "`=>`".to_string(),
            "LogicalAndOp" => "`&`".to_string(),
            "LogicalOrOp" => "`|`".to_string(),
            "DotOp" => "`.`".to_string(),
            "UnaryOp" => "`!` or `-`".to_string(),
            "PowOp" => "`^`".to_string(),
            "FactorOp" => "`*`, `/`, or `%`".to_string(),
            "TermOp" => "`+`, `-`, or `@`".to_string(),
            "ComparisonOp" => "`>`, `>=`, `<`, or `<=`".to_string(),
            "EqualityOp" => "`==` or `!=`".to_string(),

            // Delimiters
            "Semicolon" => "`;`".to_string(),
            "LParen" => "`(`".to_string(),
            "RParen" => "`)`".to_string(),
            "LBrace" => "`{`".to_string(),
            "RBrace" => "`}`".to_string(),
            "Comma" => "`,`".to_string(),
            "Colon" => "`:`".to_string(),

            // Default fallback
            _ => token
                .replace('"', "")
                .replace('\\', "")
                .replace("r#", "")
                .replace("r", "")
                .replace('#', ""),
        }
    }

    pub fn parse(&self, input: &str) -> Result<Program, Vec<String>> {
        let mut errors = Vec::<String>::new();
        let result = self.core.parse(input);

        match result {
            Ok(program) => Ok(program),
            Err(err) => match err {
                ParseError::InvalidToken { location } => {
                    let (line, col) = Self::offset_to_line_col(input, location);
                    errors.push(format!(
                        "\x1b[31mSyntax Error (line {}, column {}): Invalid token\x1b[0m",
                        line, col
                    ));
                    Err(errors)
                }
                ParseError::UnrecognizedEof { location, expected } => {
                    let (line, col) = Self::offset_to_line_col(input, location);
                    let expected_clean: Vec<String> = expected
                        .iter()
                        .map(|s| Self::token_to_human_readable(s))
                        .collect();

                    let mut unique_expected: HashSet<String> = HashSet::new();
                    for token in expected_clean {
                        unique_expected.insert(token);
                    }
                    let mut sorted_expected: Vec<String> = unique_expected.into_iter().collect();
                    sorted_expected.sort();

                    errors.push(format!(
                            "\x1b[31mSyntax Error (line {}, column {}): Unexpected end of input. Expected one of: {}\x1b[0m",
                            line, col,
                            sorted_expected.join(", ")
                        ));
                    Err(errors)
                }
                ParseError::UnrecognizedToken { token, expected } => {
                    let (line, col) = Self::offset_to_line_col(input, token.0);
                    let token_value = &token.1.1;
                    let expected_clean: Vec<String> = expected
                        .iter()
                        .map(|s| Self::token_to_human_readable(s))
                        .collect();

                    let mut unique_expected: HashSet<String> = HashSet::new();
                    for token in expected_clean {
                        unique_expected.insert(token);
                    }
                    let mut sorted_expected: Vec<String> = unique_expected.into_iter().collect();
                    sorted_expected.sort();

                    errors.push(format!(
                        "\x1b[31mSyntax Error (line {}, column {}): Unexpected token `{}`. Expected one of: {}\x1b[0m",
                        line, col, token_value,
                        sorted_expected.join(", ")
                    ));
                    Err(errors)
                }
                ParseError::ExtraToken { token } => {
                    let (line, col) = Self::offset_to_line_col(input, token.0);
                    errors.push(format!(
                        "\x1b[31mSyntax Error (line {}, column {}): Extra token `{}`\x1b[0m",
                        line, col, token.1.1
                    ));
                    Err(errors)
                }
                ParseError::User { error } => {
                    errors.push(format!("\x1b[31mSyntax Error: {}\x1b[0m", error));
                    Err(errors)
                }
            },
        }
    }
}
