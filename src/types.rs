use crate::evm::Opcode;

pub type PushData = String;

#[derive(Debug, PartialEq)]
pub enum ByteType {
    Opcode(Opcode),
    PushData(PushData)
}

#[derive(Debug, PartialEq)]
pub struct ByteData {
    pub pc: usize,
    pub bytes: ByteType
}

pub type Bytecode = Vec<ByteData>;