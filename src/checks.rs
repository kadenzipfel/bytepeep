use crate::evm::*;
use crate::types::*;

// Check if bytecode contains jumps
pub fn contains_jumps(bytecode: &Bytecode) -> bool {
    bytecode.iter().any(|byte| {
        (byte.opcode.unwrap() == Opcode::Jump) || (byte.opcode.unwrap() == Opcode::Jumpi)
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
                opcode: Some(Opcode::Push1),
                pushdata: Some(String::from("4")),
            },
            ByteData {
                code_index: 2,
                opcode: Some(Opcode::Jump),
                pushdata: None,
            },
            ByteData {
                code_index: 3,
                opcode: Some(Opcode::Swap1),
                pushdata: None,
            },
            ByteData {
                code_index: 4,
                opcode: Some(Opcode::Jumpdest),
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
                opcode: Some(Opcode::Push1),
                pushdata: Some(String::from("80")),
            },
            ByteData {
                code_index: 2,
                opcode: Some(Opcode::Push1),
                pushdata: Some(String::from("54")),
            },
            ByteData {
                code_index: 4,
                opcode: Some(Opcode::Swap1),
                pushdata: None,
            },
            ByteData {
                code_index: 5,
                opcode: Some(Opcode::Add),
                pushdata: None,
            },
        ];
        assert_eq!(false, contains_jumps(&bytecode));
    }
}
