// this represents a chunk
pub struct Chunk(Vec<u8>);

impl Chunk {
    // writes a byte to a chunk
    pub fn write(&mut self, byte: impl Into<u8>) {
        self.0.push(byte.into())
    }

    pub fn iter(&self) -> impl Iterator + '_ {
        self.0.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator + '_ {
        self.0.iter_mut()
    }
}

impl IntoIterator for Chunk {
    type Item = <Vec<u8> as IntoIterator>::Item;

    type IntoIter = <Vec<u8> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a> IntoIterator for &'a Chunk {
    type Item = &'a <Vec<u8> as IntoIterator>::Item;

    type IntoIter = <&'a Vec<u8> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl<'a> IntoIterator for &'a mut Chunk {
    type Item = &'a mut <Vec<u8> as IntoIterator>::Item;

    type IntoIter = <&'a mut Vec<u8> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter_mut()
    }
}
