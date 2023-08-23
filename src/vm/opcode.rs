#[derive(Clone, Copy, FromPrimitive, ToPrimitive)]
#[repr(u8)]
pub enum OpCode {
    Constant,
    Negate,
    Add,
    Subtract,
    Multiply,
    Divide,
    Return,
}

impl From<OpCode> for u8 {
    fn from(value: OpCode) -> Self {
        value as u8
    }
}
