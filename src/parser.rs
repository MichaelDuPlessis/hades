use crate::lexer::scanner::Scanner;

// makes use of pratt parsing
pub struct Parser<'a> {
    scanner: Scanner<'a>,
}
