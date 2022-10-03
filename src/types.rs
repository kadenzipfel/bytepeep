use crate::evm::Opcode;

#[derive(Debug, PartialEq)]
pub struct ByteData {
    pub pc: usize,
    pub bytes: Opcode
}

pub type Bytecode = Vec<ByteData>;