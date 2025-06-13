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

    pub fn parse(&self, input: &str) -> Result<Program, Vec<String>> {
        let mut errors = Vec::<String>::new();
        let result = self.core.parse(input);
        match result {
            Ok(program) => Ok(program),
            Err(err) => match err {
                ParseError::InvalidToken { location } => {
                    errors.push(format!(
                        "\x1b[31mSintax Error!: Invalid token at location: {}\x1b[0m",
                        location
                    ));
                    Err(errors)
                }
                ParseError::UnrecognizedEof { location, expected } => {
                    let expected_clean: Vec<String> = expected
                        .into_iter()
                        .map(|s| s.replace('"', "`").replace("\\", ""))
                        .collect();
                    let expected_str = expected_clean.join(", ");
                    errors.push(format!(
                        "\x1b[31mSintax Error!: Unrecognized EOF at location: {}, expected: {}\x1b[0m",
                        location, expected_str
                    ));
                    Err(errors)
                }
                ParseError::UnrecognizedToken { token, expected } => {
                    let expected_clean: Vec<String> = expected
                        .into_iter()
                        .map(|s| s.replace('"', "`").replace("\\", ""))
                        .collect();
                    let expected_str = expected_clean.join(", ");
                    errors.push(format!(
                        "\x1b[31mSintax Error!: Unrecognized token at location: {}, token: `{}`, expected: {}\x1b[0m",
                        token.0, token.1.1, expected_str
                    ));
                    Err(errors)
                }
                ParseError::ExtraToken { token } => {
                    errors.push(format!(
                        "\x1b[31mSintax Error!: Extra token at location: {}, token: {}\x1b[0m",
                        token.0, token.1.1
                    ));
                    Err(errors)
                }
                ParseError::User { error } => {
                    errors.push(format!("\x1b[31mSintax Error!: {}\x1b[0m", error));
                    Err(errors)
                }
            },
        }
    }
}