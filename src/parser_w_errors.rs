//! Custom parser wrapper that integrates LALRPOP-generated parser and enhances syntax error messages.

use crate::ast_nodes::program::Program;
use crate::parser::ProgramParser;
use lalrpop_util::ParseError;
use std::collections::HashSet;

/// Wrapper around the LALRPOP `ProgramParser` that handles syntax errors
/// and improves their reporting with line/column context and caret diagnostics.
pub struct Parser {
    core: ProgramParser,
    /// Line offset due to code injected before the user program.
    pub missplacement: i32,
}

impl Parser {
    /// Constructs a new `Parser` with the given line misplacement.
    ///
    /// # Arguments
    /// * `missplacement` - The number of lines added before the user's code,
    ///                     used to adjust error line numbers accordingly.
    pub fn new(missplacement: i32) -> Self {
        Parser {
            core: ProgramParser::new(),
            missplacement,
        }
    }

    /// Retrieves line context and position info for a byte offset.
    ///
    /// # Arguments
    /// * `input` - The full source input as a string.
    /// * `offset` - The byte offset where the error occurred.
    /// * `missplacement` - Lines to subtract from the reported line number.
    ///
    /// # Returns
    /// Tuple `(line, column, line_content, line_start_offset)`
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

        for (index, c) in input.char_indices() {
            if index > offset {
                break;
            }
            if c == '\n' {
                line_number += 1;
                line_start = index + 1;
            }
        }

        let rest = &input[line_start..];
        let line_end = match rest.find('\n') {
            Some(pos) => line_start + pos,
            None => input.len(),
        };

        let line_str = input[line_start..line_end].to_string();
        let byte_offset_in_line = offset - line_start;
        let substring = if byte_offset_in_line <= line_str.len() {
            &line_str[..byte_offset_in_line]
        } else {
            &line_str
        };
        let char_offset = substring.chars().count();
        let column = char_offset + 1;
        let adjusted_line = (line_number as i32 - missplacement).max(1) as usize;

        (adjusted_line, column, line_str, line_start)
    }

    /// Builds a simple caret (`^`) pointer for a given column.
    fn build_caret_point(col: usize) -> String {
        " ".repeat(col - 1) + "^"
    }

    /// Builds an underline of `^` under the offending token.
    fn build_caret_token(line_str: &str, col: usize, token_str: &str) -> String {
        let token_char_count = token_str.chars().count();
        let remaining_chars_in_line = line_str.chars().skip(col - 1).count();
        let underline_count = token_char_count.min(remaining_chars_in_line);
        let spaces = " ".repeat(col - 1);
        let underlines = "^".repeat(underline_count);
        spaces + &underlines
    }

    /// Converts LALRPOP token names into more human-readable forms.
    fn token_to_human_readable(token: &str) -> String {
        token
            .replace('"', "")
            .replace('\\', "")
            .replace("r#", "")
            .replace('#', "")
    }

    /// Parses the input source code and returns an AST or formatted syntax errors.
    ///
    /// # Arguments
    /// * `input` - The source code as a string.
    ///
    /// # Returns
    /// * `Ok(Program)` if parsing succeeds.
    /// * `Err(Vec<String>)` with diagnostic error messages if parsing fails.
    pub fn parse(&self, input: &str) -> Result<Program, Vec<String>> {
        let mut errors = Vec::new();
        let result = self.core.parse(input);

        match result {
            Ok(program) => Ok(program),
            Err(err) => match err {
                ParseError::InvalidToken { location } => {
                    let (line, col, line_str, _) =
                        Self::get_line_context(input, location, self.missplacement);
                    let caret = Self::build_caret_point(col);
                    errors.push(format!(
                        "\x1b[31mSyntax Error (line {}, column {}): Invalid token\n{}\n{}\x1b[0m",
                        line, col, line_str, caret
                    ));
                    Err(errors)
                }
                ParseError::UnrecognizedEof { location, expected } => {
                    let (line, col, line_str, _) =
                        Self::get_line_context(input, location, self.missplacement);
                    let caret = Self::build_caret_point(col);

                    let expected_clean: Vec<String> = expected
                        .iter()
                        .map(|s| Self::token_to_human_readable(s))
                        .collect();
                    let mut unique_expected: HashSet<String> = expected_clean.into_iter().collect();
                    let mut sorted_expected: Vec<String> = unique_expected.drain().collect();
                    sorted_expected.sort();

                    errors.push(format!(
                        "\x1b[31mSyntax Error (line {}, column {}): Unexpected end of input. Expected one of: {}\n{}\n{}\x1b[0m",
                        line, col, sorted_expected.join(", "), line_str, caret
                    ));
                    Err(errors)
                }
                ParseError::UnrecognizedToken { token, expected } => {
                    let (start, token_val, end) = token;
                    let token_value = &token_val.1;
                    let token_str = &input[start..end];

                    let (line, col, line_str, _) =
                        Self::get_line_context(input, start, self.missplacement);
                    let caret = Self::build_caret_token(&line_str, col, token_str);

                    let expected_clean: Vec<String> = expected
                        .iter()
                        .map(|s| Self::token_to_human_readable(s))
                        .collect();
                    let mut unique_expected: HashSet<String> = expected_clean.into_iter().collect();
                    let mut sorted_expected: Vec<String> = unique_expected.drain().collect();
                    sorted_expected.sort();

                    errors.push(format!(
                        "\x1b[31mSyntax Error (line {}, column {}): Unexpected token `{}`. Expected one of: {}\n{}\n{}\x1b[0m",
                        line, col, token_value, sorted_expected.join(", "), line_str, caret
                    ));
                    Err(errors)
                }
                ParseError::ExtraToken { token } => {
                    let (start, token_val, end) = token;
                    let token_str = &input[start..end];

                    let (line, col, line_str, _) =
                        Self::get_line_context(input, start, self.missplacement);
                    let caret = Self::build_caret_token(&line_str, col, token_str);

                    errors.push(format!(
                        "\x1b[31mSyntax Error (line {}, column {}): Extra token `{}`\n{}\n{}\x1b[0m",
                        line, col, token_val.1, line_str, caret
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
