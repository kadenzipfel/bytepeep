use crate::evm::Opcode;

pub type PushData = String;

#[derive(Debug, PartialEq, Clone)]
pub struct ByteData {
    pub code_index: usize,
    pub opcode: Option<Opcode>,
    pub pushdata: Option<PushData>
}

pub type Bytecode = Vec<ByteData>;
