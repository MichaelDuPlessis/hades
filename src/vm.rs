use num_traits::FromPrimitive;

use crate::vm::opcode::OpCode;

use self::chunk::Chunk;

pub mod chunk;
pub mod common;
pub mod opcode;
pub mod value_array;

fn simple_instruction(offset: usize) -> usize {
    offset + 1
}

fn consant_instruction(chunk: &Chunk, offset: usize) -> usize {
    let constant = chunk.index_code(offset + 1);
    println!(
        "Constant Value {}",
        chunk.index_constants(constant as usize)
    );
    offset + 2
}

fn dissamble_instruction(chunk: &Chunk, offset: usize) -> usize {
    println!("Offset: {offset}");

    let instruction = FromPrimitive::from_u8(chunk.index_code(offset)).unwrap();
    match instruction {
        OpCode::Constant => consant_instruction(chunk, offset),
        OpCode::Return => simple_instruction(offset),
        _ => {
            println!("Unknown Instruction");
            offset + 1
        }
    }
}

pub fn dissamble_chunk(chunk: &Chunk) {
    let mut offset = 0;
    while offset < chunk.len() {
        offset = dissamble_instruction(chunk, offset);
    }
}
