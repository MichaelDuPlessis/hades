use crate::{
    lexer::{
        scanner::{self, Scanner},
        token::Token,
    },
    vm::chunk::Chunk,
};

pub struct Parser<'a, I: Iterator<Item = &'a scanner::Result>> {
    scanner: I,
    chunk: Chunk,
    previous: &'a scanner::Result,
    current: &'a scanner::Result,
}

impl<'a, I: Iterator<Item = &'a scanner::Result>> Parser<'a, I> {
    pub fn new(scanner: &Scanner<'_>) -> Self {
        let scanner = scanner.peekable();
        let current = scanner.peek().unwrap();
        Self {
            scanner,
            chunk: Chunk::default(),
            current,
            previous: current,
        }
    }
}
