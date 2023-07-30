use std::collections::HashMap;

use super::{token::Token, token_type::TokenType};

// to determine what if a keyword has been found
static KEYWORDS: once_cell::sync::Lazy<HashMap<&str, TokenType>> =
    once_cell::sync::Lazy::new(|| {
        HashMap::from([
            ("while", TokenType::While),
            ("if", TokenType::If),
            ("else", TokenType::Else),
            ("let", TokenType::Let),
        ])
    });

// for error handeling
#[derive(Debug)]
pub enum Error {
    UnrecognizedToken,
    UnterminatedString,
    InvalidNumFormat,
}

pub type Result = std::result::Result<Token, Error>;

pub struct Scanner<'a> {
    source: &'a [u8],
    current: usize,
    start: usize,
    line: usize,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a [u8]) -> Self {
        Self {
            source,
            current: 0,
            start: 0,
            line: 1,
        }
    }

    pub fn scan_token(&mut self) -> Result {
        // skipping tokens we don't want to read
        self.skip();

        if self.is_at_end() {
            return Self::make_token(TokenType::Eof);
        }

        self.start = self.current;

        let byte = self.advance();
        match byte {
            // single character tokens
            b'(' => Self::make_token(TokenType::LParen),
            b')' => Self::make_token(TokenType::RParen),
            b'{' => Self::make_token(TokenType::LBrace),
            b'}' => Self::make_token(TokenType::RBrace),
            b'[' => Self::make_token(TokenType::LBracket),
            b']' => Self::make_token(TokenType::RBracket),
            b'.' => Self::make_token(TokenType::Dot),
            b'+' => Self::make_token(TokenType::Plus),
            b'-' => Self::make_token(TokenType::Minus),
            b'*' => Self::make_token(TokenType::Asterisk),
            b'/' => Self::make_token(TokenType::Slash),
            b',' => Self::make_token(TokenType::Comma),

            // double characters
            b'=' => {
                if self.if_next(b'=') {
                    Self::make_token(TokenType::EqualEqual)
                } else {
                    Self::make_token(TokenType::Equal)
                }
            }
            b'!' => {
                if self.if_next(b'=') {
                    Self::make_token(TokenType::BangEqual)
                } else {
                    Self::make_token(TokenType::Bang)
                }
            }
            b'>' => {
                if self.if_next(b'=') {
                    Self::make_token(TokenType::GreaterEqual)
                } else {
                    Self::make_token(TokenType::Greater)
                }
            }
            b'<' => {
                if self.if_next(b'=') {
                    Self::make_token(TokenType::LessEqual)
                } else {
                    Self::make_token(TokenType::Less)
                }
            }

            // strings
            b'"' => self.string(),

            // error
            b => {
                // numbers
                if b.is_ascii_digit() {
                    self.num()
                } else if Self::valid_ident_chars(b) {
                    self.identifier()
                } else {
                    Err(Error::UnrecognizedToken)
                }
            }
        }
    }

    // this is just a thin wrapper that will always return Ok
    fn make_token(token_type: TokenType) -> Result {
        Ok(Token::new(token_type))
    }

    fn string(&mut self) -> Result {
        self.advance();
        while self.peek() != b'"' {
            if self.is_at_end() {
                return Err(Error::UnterminatedString);
            }

            let byte = self.advance();
            if byte == b'\n' {
                self.line += 1;
            }
        }
        self.advance();

        Ok(Token::new(TokenType::Str(unsafe {
            String::from_utf8_unchecked(self.source[self.start + 1..self.current - 1].to_vec())
        })))
    }

    fn num(&mut self) -> Result {
        while self.peek().is_ascii_digit() {
            self.advance();
        }

        if self.peek() == b'.' {
            self.advance();
            if !self.peek().is_ascii_digit() {
                return Err(Error::InvalidNumFormat);
            }

            while self.peek().is_ascii_digit() {
                self.advance();
            }
        }

        Ok(Token::new(TokenType::Num(unsafe {
            std::str::from_utf8_unchecked(&self.source[self.start..self.current])
                .parse()
                .unwrap()
        })))
    }

    fn identifier(&mut self) -> Result {
        while !self.is_at_end() && Self::valid_ident_chars(self.peek()) {
            self.advance();
        }

        let ident =
            unsafe { std::str::from_utf8_unchecked(&self.source[self.start..self.current]) };
        if let Some(keyword) = KEYWORDS.get(ident) {
            Self::make_token(keyword.clone())
        } else {
            Self::make_token(TokenType::Identifier(ident.to_owned()))
        }
    }

    fn valid_ident_chars(byte: u8) -> bool {
        byte.is_ascii_alphabetic() || byte == b'_'
    }

    fn is_at_end(&self) -> bool {
        self.current == self.source.len()
    }

    fn advance(&mut self) -> u8 {
        let byte = self.source[self.current];
        self.current += 1;
        byte
    }

    fn peek(&self) -> u8 {
        self.source[self.current]
    }

    // can panic if current == source.len - 1
    fn peek_next(&self) -> u8 {
        self.source[self.current + 1]
    }

    fn if_next(&self, byte: u8) -> bool {
        self.peek() == byte
    }

    fn skip(&mut self) {
        while !self.is_at_end() {
            let byte = self.peek();
            if byte == b'#' {
                self.skip_comment();
                continue;
            }
            if byte == b' ' || byte == b'\t' || byte == b'\r' {
                self.advance();
                continue;
            }
            if byte == b'\n' {
                self.line += 1;
                self.advance();
                continue;
            }
            break;
        }
    }

    fn skip_whitespace(&mut self) {
        loop {
            let byte = self.peek();
            if byte != b' ' && byte != b'\t' && byte != b'\r' {
                break;
            }
            self.advance();
        }
    }

    fn skip_comment(&mut self) {
        if self.peek() == b'#' {
            while self.advance() != b'\n' {}
        }
        self.line += 1;
    }
}

impl Iterator for Scanner<'_> {
    type Item = Result;

    fn next(&mut self) -> Option<Self::Item> {
        if self.is_at_end() {
            None
        } else {
            Some(self.scan_token())
        }
    }
}
