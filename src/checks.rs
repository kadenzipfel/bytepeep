use crate::evm::*;
use crate::types::*;

// Check if bytecode contains jumps
pub fn contains_jumps(bytecode: &Bytecode) -> bool {
    bytecode.iter().any(|byte| {
        if byte.kind == ByteKind::Opcode {
            (byte.opcode.unwrap() == Opcode::Jump) || (byte.opcode.unwrap() == Opcode::Jumpi)
        } else {
            false
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contains_jumps() {
        let bytecode: Bytecode = vec![
            ByteData {
                pc: 0,
                opcode: Some(Opcode::Push1),
                pushdata: None,
                kind: ByteKind::Opcode,
            },
            ByteData {
                pc: 1,
                opcode: None,
                pushdata: Some(String::from("4")),
                kind: ByteKind::PushData,
            },
            ByteData {
                pc: 2,
                opcode: Some(Opcode::Jump),
                pushdata: None,
                kind: ByteKind::Opcode,
            },
            ByteData {
                pc: 3,
                opcode: Some(Opcode::Swap1),
                pushdata: None,
                kind: ByteKind::Opcode,
            },
            ByteData {
                pc: 4,
                opcode: Some(Opcode::Jumpdest),
                pushdata: None,
                kind: ByteKind::Opcode,
            },
        ];
        assert_eq!(true, contains_jumps(&bytecode));
    }

    #[test]
    fn test_not_contains_jumps() {
        let bytecode: Bytecode = vec![
            ByteData {
                pc: 0,
                opcode: Some(Opcode::Push1),
                pushdata: None,
                kind: ByteKind::Opcode,
            },
            ByteData {
                pc: 1,
                opcode: None,
                pushdata: Some(String::from("80")),
                kind: ByteKind::PushData,
            },
            ByteData {
                pc: 2,
                opcode: Some(Opcode::Push1),
                pushdata: None,
                kind: ByteKind::Opcode,
            },
            ByteData {
                pc: 3,
                opcode: None,
                pushdata: Some(String::from("54")),
                kind: ByteKind::PushData,
            },
            ByteData {
                pc: 4,
                opcode: Some(Opcode::Swap1),
                pushdata: None,
                kind: ByteKind::Opcode,
            },
            ByteData {
                pc: 5,
                opcode: Some(Opcode::Add),
                pushdata: None,
                kind: ByteKind::Opcode,
            },
        ];
        assert_eq!(false, contains_jumps(&bytecode));
    }
}
