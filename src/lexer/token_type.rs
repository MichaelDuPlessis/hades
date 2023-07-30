#[derive(Debug, Clone)]
pub enum TokenType {
    // Braces
    LParen,
    RParen,
    LBrace,
    RBrace,
    LBracket,
    RBracket,

    // Operations
    Equal,
    Plus,
    Minus,
    Asterisk,
    Slash,
    Dot,
    Let,

    // Equality stuff
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    Bang,
    BangEqual,

    // Values
    Identifier(String),
    Str(String),
    Num(f64), // maybe introduce two number type
    True,
    False,

    // Conditional
    While,
    If,
    Else,

    // other
    Comma,
    Eof,
}
