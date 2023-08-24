use self::{chunk::Chunk, common::Value};
use crate::vm::opcode::OpCode;
use num_traits::FromPrimitive;

pub mod chunk;
pub mod common;
pub mod opcode;
pub mod value_array;

// macro to no repeat operator code
macro_rules! binary_op {
    ($self:ident, $op:tt) => {
        {
            let y = $self.pop_stack();
            let x = $self.pop_stack();
            $self.push_stack(x $op y);
        }
    };
}

pub enum InterpretError {
    CompileError,
    RuntimeError,
}

pub type InterpretResult = Result<(), InterpretError>;

pub struct VM<'a> {
    chunk: &'a Chunk, // the chunk of code to executee
    ip: usize,        // the next instruction to execute
    source: &'a [u8], // the source code
    // recode to custom stack structure
    stack: Vec<Value>, // a stack holding values
}

impl<'a> VM<'a> {
    pub fn new(chunk: &'a Chunk, source: impl Into<&'a [u8]>) -> Self {
        Self {
            chunk,
            ip: 0,
            source: source.into(),
            stack: Vec::new(),
        }
    }

    pub fn interpret(&mut self) -> InterpretResult {
        if !compile(self.source, self.chunk) {
            return Err(InterpretError::CompileError);
        }

        self.run()
    }

    fn run(&mut self) -> InterpretResult {
        loop {
            let instruction = self.read_byte();
            match FromPrimitive::from_u8(instruction).unwrap() {
                OpCode::Constant => {
                    let constant = self.read_constant();
                    self.push_stack(constant);
                }
                OpCode::Negate => {
                    let value = -self.pop_stack();
                    self.push_stack(value);
                }
                OpCode::Add => {
                    binary_op!(self, +)
                }
                OpCode::Subtract => {
                    binary_op!(self, -)
                }
                OpCode::Multiply => {
                    binary_op!(self, *)
                }
                OpCode::Divide => {
                    binary_op!(self, /)
                }
                OpCode::Return => return Ok(()),
            }
        }
    }

    fn read_byte(&mut self) -> u8 {
        let byte = self.ip;
        self.ip += 1;
        self.chunk.index_code(byte)
    }

    fn read_constant(&mut self) -> Value {
        self.chunk.index_constants(self.read_byte().into())
    }

    fn push_stack(&mut self, value: Value) {
        self.stack.push(value);
    }

    fn pop_stack(&mut self) -> Value {
        self.stack.pop().unwrap()
    }
}

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
