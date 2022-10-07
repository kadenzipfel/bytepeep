use crate::evm::Opcode;

pub type PushData = String;

#[derive(Debug, PartialEq, Clone)]
pub struct ByteData {
    pub pc: u32,
    pub opcode: Option<Opcode>,
    pub pushdata: Option<PushData>
}

pub type Bytecode = Vec<ByteData>;
