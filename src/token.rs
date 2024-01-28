#[derive(Debug)]
pub enum TokenType {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    IDENTIFIER,
    STRING,
    NUMBER,

    AND,
    OR,
    CLASS,
    ELSE,
    FALSE,
    FUN,
    FOR,
    IF,
    NIL,
    PRINT,
    RETURN,
    SUPER,
    THIS,
    TRUE,
    VAR,
    WHILE,

    EOF,
}

pub struct Token {
    token_type: TokenType,
    lexeme: String,
    // literal;
    line: isize,
}

impl Token {
    pub(crate) fn new(token_type: TokenType, lexeme: String, line: isize) -> Token {
        Token {
            token_type,
            lexeme,
            line,
        }
    }
    fn to_string(&self) -> String {
        format!("{:?} {} {}", self.token_type, self.lexeme, self.line)
    }
}