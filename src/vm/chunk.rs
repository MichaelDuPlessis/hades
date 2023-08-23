use super::{common::Value, value_array::ValueArray};

// this represents a chunk
#[derive(Default)]
pub struct Chunk {
    code: Vec<u8>,
    lines: Vec<usize>,
    constants: ValueArray,
}

impl Chunk {
    // writes a byte to a chunk
    pub fn write(&mut self, byte: impl Into<u8>, line: usize) {
        self.code.push(byte.into());
        self.lines.push(line);
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

    pub fn index_code(&self, index: usize) -> u8 {
        self.code[index]
    }

    pub fn index_constants(&self, index: usize) -> Value {
        self.constants[index]
    }
}

impl IntoIterator for Chunk {
    type Item = <Vec<u8> as IntoIterator>::Item;

    type IntoIter = <Vec<u8> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.code.into_iter()
    }
}

impl<'a> IntoIterator for &'a Chunk {
    type Item = &'a <Vec<u8> as IntoIterator>::Item;

    type IntoIter = <&'a Vec<u8> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.code.iter()
    }
}

impl<'a> IntoIterator for &'a mut Chunk {
    type Item = &'a mut <Vec<u8> as IntoIterator>::Item;

    type IntoIter = <&'a mut Vec<u8> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.code.iter_mut()
    }
}
