use super::token_type::TokenType;

#[derive(Debug)]
pub struct Token {
    token_type: TokenType,
}

impl Token {
    pub fn new(token_type: TokenType) -> Self {
        Self { token_type }
    }
}