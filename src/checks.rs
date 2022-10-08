use crate::{evm::*, types::*};

// Check if bytecode contains jumps
pub fn contains_jumps(bytecode: &Bytecode) -> bool {
    bytecode.iter().any(|byte| {
        (byte.opcode == Opcode::Jump) || (byte.opcode == Opcode::Jumpi)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contains_jumps() {
        let bytecode: Bytecode = vec![
            ByteData {
                code_index: 0,
                opcode: Opcode::Push1,
                pushdata: Some(String::from("4")),
            },
            ByteData {
                code_index: 2,
                opcode: Opcode::Jump,
                pushdata: None,
            },
            ByteData {
                code_index: 3,
                opcode: Opcode::Swap1,
                pushdata: None,
            },
            ByteData {
                code_index: 4,
                opcode: Opcode::Jumpdest,
                pushdata: None,
            },
        ];
        assert_eq!(true, contains_jumps(&bytecode));
    }

    #[test]
    fn test_not_contains_jumps() {
        let bytecode: Bytecode = vec![
            ByteData {
                code_index: 0,
                opcode: Opcode::Push1,
                pushdata: Some(String::from("80")),
            },
            ByteData {
                code_index: 2,
                opcode: Opcode::Push1,
                pushdata: Some(String::from("54")),
            },
            ByteData {
                code_index: 4,
                opcode: Opcode::Swap1,
                pushdata: None,
            },
            ByteData {
                code_index: 5,
                opcode: Opcode::Add,
                pushdata: None,
            },
        ];
        assert_eq!(false, contains_jumps(&bytecode));
    }
}
