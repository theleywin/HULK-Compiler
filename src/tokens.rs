
#[derive(Debug, PartialEq)]
pub enum KeywordToken {
    PRINT,
    WHILE,
    ELIF,
    ELSE,
    IF,
    IN,
    LET,
}
#[derive(Debug, PartialEq)]
pub enum OperatorToken {
    MUL,
    DIV,
    PLUS,
    MINUS,
    MOD,
}
#[derive(Debug, PartialEq)]
pub enum DelimiterToken {
    SEMICOLON,
    COMMA,
    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,
}
#[derive(Debug, PartialEq)]
pub enum IdentifierToken {
    IDENTIFIER(String),
}
#[derive(Debug, PartialEq)]
pub enum Token {
    Keyword(KeywordToken),
    Operator(OperatorToken),
    Delimiter(DelimiterToken),
    Identifier(IdentifierToken),
    EOF,
}