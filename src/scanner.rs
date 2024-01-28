use std::collections::HashMap;
use std::io::stderr;
use std::thread::current;
use crate::token::{Token, TokenType};


struct Scanner {
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
    fn new(source: String) -> Self {
        // self.source = source;
        Self { source, tokens: vec![], start: 0, current: 0, line: 1, keywords: HashMap::new() }
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
                        while self.source.chars().nth(self.current) != Some('\n') && !self.is_at_end() {
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
                    while self.source.chars().nth(self.current) != Some('"') && !self.is_at_end() {
                        if self.source.chars().nth(self.current) == Some('\n') {
                            self.line += 1;
                        }
                        self.advance();
                    }
                    if self.is_at_end() {
                        eprintln!("Unterminated string at line {}", self.line);
                        return;
                    }
                    self.advance();
                    self.add_token(TokenType::STRING);
                }
                '0'..='9' => {
                    while self.source.chars().nth(self.current).unwrap().is_digit(10) {
                        self.advance();
                    }
                    if self.source.chars().nth(self.current).unwrap() == '.' &&
                        self.source.chars().nth(self.current + 1).unwrap().is_digit(10) {

                        self.advance();
                        while self.source.chars().nth(self.current).unwrap().is_digit(10) {
                            self.advance();
                        }
                    }
                    self.add_token(TokenType::NUMBER);
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
        self.keywords.insert(self.source[self.start..self.current].parse().unwrap(), token_type);
    }
    // fn scan_tokens(&mut self) -> Vec<Token> {
    //     // while !self.isAtEnd() {
    //     //     self.start = self.current;
    //     //     self.scanToken();
    //     // }
    //     // self.tokens.push(Token::new(TokenType::EOF, "
    //                 ".to_string(), self.line as isize));
    //     // self.tokens.clone()
    // }
}