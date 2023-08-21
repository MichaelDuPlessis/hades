#[macro_use]
extern crate num_derive;

use crate::vm::opcode::OpCode;
use lexer::scanner::Scanner;
use std::mem::size_of;

mod lexer;
mod parser;
mod vm;

fn main() {
    let source = std::fs::read("./test.hd").unwrap();
    // dbg!(&source);
    let scanner = Scanner::new(&source);
    for token in scanner {
        dbg!(token.unwrap());
    }
    println!("{}", size_of::<OpCode>())
}
