use std::ops::Index;

use super::{common::Value, opcode::OpCode, value_array::ValueArray};

// this represents a chunk
#[derive(Default)]
pub struct Chunk {
    code: Vec<OpCode>,
    constants: ValueArray,
}

impl Chunk {
    // writes a byte to a chunk
    pub fn write(&mut self, byte: OpCode) {
        self.code.push(byte)
    }

    pub fn add_constant(&mut self, value: Value) -> usize {
        self.constants.write(value);
        self.constants.len() - 1
    }

    pub fn iter(&self) -> impl Iterator + '_ {
        self.code.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator + '_ {
        self.code.iter_mut()
    }

    pub fn len(&self) -> usize {
        self.code.len()
    }
}

impl Index<usize> for Chunk {
    type Output = OpCode;

    fn index(&self, index: usize) -> &Self::Output {
        &self.code[index]
    }
}

impl IntoIterator for Chunk {
    type Item = <Vec<OpCode> as IntoIterator>::Item;

    type IntoIter = <Vec<OpCode> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.code.into_iter()
    }
}

impl<'a> IntoIterator for &'a Chunk {
    type Item = &'a <Vec<OpCode> as IntoIterator>::Item;

    type IntoIter = <&'a Vec<OpCode> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.code.iter()
    }
}

impl<'a> IntoIterator for &'a mut Chunk {
    type Item = &'a mut <Vec<OpCode> as IntoIterator>::Item;

    type IntoIter = <&'a mut Vec<OpCode> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.code.iter_mut()
    }
}
