#[macro_use]
extern crate num_derive;

use crate::vm::opcode::OpCode;
use vm::{chunk::Chunk, dissamble_chunk, VM};

mod compiler;
mod lexer;
mod parser;
mod vm;

fn main() {
    let mut chunk = Chunk::default();
    let constant = chunk.add_constant(1.2);

    chunk.write(OpCode::Constant, 123);
    chunk.write(constant as u8, 123);
    chunk.write(OpCode::Negate, 123);
    chunk.write(OpCode::Return as u8, 123);

    let mut vm = VM::new(&chunk);
    let _ = vm.interpret();
}
