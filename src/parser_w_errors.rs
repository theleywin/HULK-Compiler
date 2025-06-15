use std::collections::HashSet;

use crate::ast_nodes::program::Program;
use lalrpop_util::ParseError;

use crate::parser::ProgramParser;

pub struct Parser {
    core: ProgramParser,
    pub missplacement: i32,
}
impl Parser {
    pub fn new(missplacement : i32) -> Self {
        Parser {
            core: ProgramParser::new(),
            missplacement: missplacement,
        }
    }

    fn offset_to_line_col(input: &str, offset: usize, missplacement: i32) -> (usize, usize) {
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

        (line - missplacement as usize, col)
    }

    fn token_to_human_readable(token: &str) -> String {
        match token {
            _ => token
                .replace('"', "")
                .replace('\\', "")
                .replace("r#", "")
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
                    let (line, col) = Self::offset_to_line_col(input, location, self.missplacement);
                    errors.push(format!(
                        "\x1b[31mSyntax Error (line {}, column {}): Invalid token\x1b[0m",
                        line, col
                    ));
                    Err(errors)
                }
                ParseError::UnrecognizedEof { location, expected } => {
                    let (line, col) = Self::offset_to_line_col(input, location, self.missplacement);
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
                    let (line, col) = Self::offset_to_line_col(input, token.0, self.missplacement);
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
                    let (line, col) = Self::offset_to_line_col(input, token.0, self.missplacement);
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
