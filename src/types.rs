pub enum ByteKind {
    Opcode,
    PushData,
    InvalidOpcode
}

pub struct ByteType {
    kind: ByteKind
}

pub struct ByteData {
    pc: usize,
    bytes: ByteType
}

pub type Bytecode = Vec<ByteData>;