use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

impl Span {
    pub fn new(start: usize, end: usize) -> Self {
        Span { start, end }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum KeywordToken {
    PRINT,
    WHILE,
    FOR,
    ELIF,
    ELSE,
    IF,
    IN,
    LET,
    TRUE,
    FALSE,
    FUNCTION,
    INHERITS,
    TYPE,
    NEW,
}

#[derive(Debug, PartialEq, Clone)]
pub enum OperatorToken {
    MUL,
    DIV,
    PLUS,
    MINUS,
    MOD,
    POW,
    NEG,
    NOT,
    EQ,
    NEQ,
    GT,
    GTE,
    LT,
    LTE,
    ASSIGN,
    CONCAT,
    DASSIGN,
    OR,
    AND,
    DOT,
}

impl fmt::Display for OperatorToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            OperatorToken::MUL => "*",
            OperatorToken::DIV => "/",
            OperatorToken::PLUS => "+",
            OperatorToken::MINUS => "-",
            OperatorToken::MOD => "%",
            OperatorToken::POW => "^",
            OperatorToken::NEG => "-",
            OperatorToken::NOT => "!",
            OperatorToken::EQ => "==",
            OperatorToken::NEQ => "!=",
            OperatorToken::GT => ">",
            OperatorToken::GTE => ">=",
            OperatorToken::LT => "<",
            OperatorToken::LTE => "<=",
            OperatorToken::ASSIGN => "=",
            OperatorToken::CONCAT => "@",
            OperatorToken::DASSIGN => ":=",
            OperatorToken::AND => "&",
            OperatorToken::OR => "|",
            OperatorToken::DOT => ".",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum DelimiterToken {
    SEMICOLON,
    COLON,
    COMMA,
    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,
    ARROW,
}
