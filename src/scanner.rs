use std::collections::HashMap;
use std::iter::FromIterator;

#[derive(Debug, Clone, Copy, PartialEq)]
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

    IF,
    ELSE,
    WHILE,
    FOR,
    TRUE,
    FALSE,

    CLASS,
    FUN,
    NIL,
    SUPER,
    THIS,
    PRINT,
    RETURN,

    VAR,

    EOF,
}

pub struct Token {
    token_type: TokenType,
    lexeme: String,
    // literal;
    line: usize,
}

impl Token {
    pub(crate) fn new(token_type: TokenType, lexeme: String, line: usize) -> Token {
        Token {
            token_type,
            lexeme,
            line,
        }
    }
    pub fn to_string(&self) -> String {
        format!("L{}: {:?} {}", self.line, self.token_type, self.lexeme)
    }
}

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    // err: Option<Error>,
    start: usize,
    current: usize,
    line: usize,
    // col: isize,
    keywords: HashMap<String, TokenType>,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        // self.source = source;
        let mut ret = Self {
            source,
            tokens: vec![],
            start: 0,
            current: 0,
            line: 1,
            keywords: HashMap::from_iter([
                ("and".into(), TokenType::AND),
                ("or".into(), TokenType::OR),
                ("true".into(), TokenType::TRUE),
                ("false".into(), TokenType::FALSE),
                ("if".into(), TokenType::IF),
                ("else".into(), TokenType::ELSE),
                ("while".into(), TokenType::WHILE),
                ("for".into(), TokenType::FOR),
                ("return".into(), TokenType::RETURN),
                ("class".into(), TokenType::CLASS),
                ("fun".into(), TokenType::FUN),
                ("this".into(), TokenType::THIS),
                ("super".into(), TokenType::SUPER),
                ("print".into(), TokenType::PRINT),
                ("nil".into(), TokenType::NIL),
                ("var".into(), TokenType::VAR),
            ]),
        };
        ret.scan_tokens();
        ret
    }
    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }
    fn advance(&mut self) {
        self.current += 1;
    }
    fn advance_match(&mut self, expect: char) -> bool {
        if self.is_at_end() || self.source.chars().nth(self.current).unwrap() != expect {
            return false;
        }
        self.advance();
        true
    }
    fn scan_tokens(&mut self) {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }
    }
    fn scan_token(&mut self) {
        self.advance();
        if let Some(c) = self.source.chars().nth(self.current - 1) {
            match c {
                '(' => self.add_token(TokenType::LeftParen),
                ')' => self.add_token(TokenType::RightParen),
                '{' => self.add_token(TokenType::LeftBrace),
                '}' => self.add_token(TokenType::RightBrace),
                '/' => {
                    if self.advance_match('/') {
                        while self.peek() != Some('\n')
                            && !self.is_at_end()
                        {
                            self.advance();
                        }
                    } else {
                        self.add_token(TokenType::Slash);
                    }
                }
                '*' => self.add_token(TokenType::Star),
                '+' => self.add_token(TokenType::Plus),
                '-' => self.add_token(TokenType::Minus),
                ',' => self.add_token(TokenType::Comma),
                '.' => self.add_token(TokenType::Dot),
                ';' => self.add_token(TokenType::Semicolon),
                '>' => {
                    if self.advance_match('=') {
                        self.add_token(TokenType::GreaterEqual);
                    } else {
                        self.add_token(TokenType::Greater);
                    }
                }
                '<' => {
                    if self.advance_match('=') {
                        self.add_token(TokenType::LessEqual);
                    } else {
                        self.add_token(TokenType::Less);
                    }
                }
                '!' => {
                    if self.advance_match('=') {
                        self.add_token(TokenType::BangEqual);
                    } else {
                        self.add_token(TokenType::Bang);
                    }
                }
                '=' => {
                    if self.advance_match('=') {
                        self.add_token(TokenType::EqualEqual);
                    } else {
                        self.add_token(TokenType::Equal);
                    }
                }
                '"' => {
                    while self.peek() != Some('"') && !self.is_at_end() {
                        if self.peek() == Some('\n') {
                            self.line += 1;
                        }
                        self.advance();
                    }
                    if self.is_at_end() {
                        eprintln!("Unterminated string at line {}", self.line);
                        return;
                    }
                    self.advance();
                    self.add_token_with_lexeme(
                        TokenType::STRING,
                        self.source[self.start..self.current].to_string(),
                    );
                }
                '0'..='9' => {
                    self.advance_while_true(|c| c.is_digit(10));
                    if !self.is_at_end() && self.peek().unwrap() == '.' && self.peek_next(1).unwrap().is_digit(10) {
                        self.advance();
                        self.advance_while_true(|c| c.is_digit(10));
                    }
                    self.add_token_with_lexeme(
                        TokenType::NUMBER,
                        self.source[self.start..self.current].to_string(),
                    );
                }
                c if Self::is_alpha(&c) => {
                    self.advance_while_true(|c| Self::is_alpha(&c) || c.is_digit(10));
                    let text = self.source[self.start..self.current].to_string();
                    if let Some(tokentype) = self.keywords.get(&text) {
                        self.add_token(tokentype.clone());
                    } else {
                        self.add_token_with_lexeme(
                            TokenType::IDENTIFIER,
                            self.source[self.start..self.current].to_string(),
                        );
                    }
                }
                ' ' | '\r' | '\t' => {}
                '\n' => self.line += 1,
                _ => {
                    eprintln!("Unexpected token {} at line {}", c, self.line);
                }
            }
        }
    }

    fn add_token(&mut self, token_type: TokenType) {
        self.tokens
            .push(Token::new(token_type, "".to_string(), self.line));
    }
    fn add_token_with_lexeme(&mut self, token_type: TokenType, lexeme: String) {
        self.tokens.push(Token::new(token_type, lexeme, self.line));
    }
    fn is_alpha(c: &char) -> bool {
        c.is_ascii_alphanumeric() || c == &'_'
    }
    fn peek(&self) -> Option<char> {
        self.source.chars().nth(self.current)
    }
    fn peek_next(&self, next: usize) -> Option<char> {
        self.source.chars().nth(self.current + next)
    }
    fn advance_while_true<F>(&mut self, f: F)
    where
        F: Fn(&char) -> bool,
    {
        loop {
            let next = self.peek();
            if let Some(next) = next {
                if f(&next) {
                    self.advance();
                    continue;
                }
            }
            break;
        }
    }
    pub fn print_tokens(&self) {
        for i in &self.tokens {
            println!("{}", i.to_string());
        }
    }
}
