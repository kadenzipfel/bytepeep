use crate::evm::Opcode;

pub type PushData = String;

#[derive(Debug, PartialEq)]
pub enum ByteKind {
    Opcode,
    PushData
}

#[derive(Debug, PartialEq)]
pub struct ByteData {
    pub pc: u32,
    pub opcode: Option<Opcode>,
    pub pushdata: Option<PushData>,
    pub kind: ByteKind
}

pub type Bytecode = Vec<ByteData>;