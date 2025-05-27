use std::fmt;

#[derive(Debug, PartialEq)]
pub enum KeywordToken {
    //PRINT,
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
    ASSIGN,
    CONCAT,
    DASSIGN,
    OR,
    AND,
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
        };
        write!(f, "{}", s)
    }
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
