use super::common::Value;

#[derive(Default)]
pub struct ValueArray(Vec<Value>);

impl ValueArray {
    pub fn write(&mut self, value: Value) {
        self.0.push(value)
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}
