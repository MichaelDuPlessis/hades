#[derive(Clone, Copy, FromPrimitive, ToPrimitive)]
pub enum OpCode {
    Constant,
    Return,
}
