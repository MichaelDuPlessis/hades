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
            ("true", TokenType::True),
            ("false", TokenType::False),
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
            return self.make_token(TokenType::Eof);
        }

        self.start = self.current;

        let byte = self.advance();
        match byte {
            // single character tokens
            b'(' => self.make_token(TokenType::LParen),
            b')' => self.make_token(TokenType::RParen),
            b'{' => self.make_token(TokenType::LBrace),
            b'}' => self.make_token(TokenType::RBrace),
            b'[' => self.make_token(TokenType::LBracket),
            b']' => self.make_token(TokenType::RBracket),
            b'.' => self.make_token(TokenType::Dot),
            b'+' => self.make_token(TokenType::Plus),
            b'-' => self.make_token(TokenType::Minus),
            b'*' => self.make_token(TokenType::Asterisk),
            b'/' => self.make_token(TokenType::Slash),
            b',' => self.make_token(TokenType::Comma),

            // double characters
            b'=' => {
                self.make_token(self.choose_next(b'=', TokenType::EqualEqual, TokenType::Equal))
            }
            b'!' => self.make_token(self.choose_next(b'=', TokenType::BangEqual, TokenType::Bang)),
            b'>' => {
                self.make_token(self.choose_next(b'=', TokenType::GreaterEqual, TokenType::Greater))
            }
            b'<' => self.make_token(self.choose_next(b'=', TokenType::LessEqual, TokenType::Less)),

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
    fn make_token(&self, token_type: TokenType) -> Result {
        Ok(Token::new(token_type, self.line))
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

        self.make_token(TokenType::Str(unsafe {
            String::from_utf8_unchecked(self.source[self.start + 1..self.current - 1].to_vec())
        }))
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

        self.make_token(TokenType::Num(unsafe {
            std::str::from_utf8_unchecked(&self.source[self.start..self.current])
                .parse()
                .unwrap()
        }))
    }

    fn identifier(&mut self) -> Result {
        while !self.is_at_end() && Self::valid_ident_chars(self.peek()) {
            self.advance();
        }

        let ident =
            unsafe { std::str::from_utf8_unchecked(&self.source[self.start..self.current]) };
        if let Some(keyword) = KEYWORDS.get(ident) {
            self.make_token(keyword.clone())
        } else {
            self.make_token(TokenType::Identifier(ident.to_owned()))
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

    fn choose_next(&self, byte: u8, t1: TokenType, t2: TokenType) -> TokenType {
        if self.peek() == byte {
            t1
        } else {
            t2
        }
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
