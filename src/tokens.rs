#[derive(Debug, PartialEq)]
pub enum KeywordToken {
    PRINT,
    WHILE,
    ELIF,
    ELSE,
    IF,
    IN,
    LET,
    TRUE,
    FALSE,
    FUNCTION,
}

#[derive(Debug, PartialEq)]
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
}

#[derive(Debug, PartialEq)]
pub enum DelimiterToken {
    SEMICOLON,
    COMMA,
    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,
    ARROW,
}
