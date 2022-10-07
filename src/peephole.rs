use crate::evm::*;
use crate::rules::*;
use crate::types::*;

pub fn optimize(bytecode: &Bytecode) -> Bytecode {
    let mut i: usize = 0;
    let mut code_index: usize = 0;
    let mut optimized_bytecode: Bytecode = vec![];

    while i < bytecode.len() {
        let mut increment = 0;
        let mut next_op = (i + 1) as usize;

        // If current opcode is last, push byte
        if next_op >= bytecode.len() {
            let byte = bytecode[i].clone();
            optimized_bytecode.push(ByteData {
                code_index: code_index,
                opcode: byte.opcode,
                pushdata: byte.pushdata,
            });

            break;
        }

        // Grab two byte peephole
        let bytes: Bytecode = vec![bytecode[i].clone(), bytecode[next_op].clone()];

        // Check peephole for rule violations, and place first optimized byte in bytecode
        let peeped_bytes = check_rules(&bytes);
        let byte: ByteData = peeped_bytes[0].clone();
        let byte_code_index = ByteData {
            code_index: code_index,
            opcode: byte.opcode,
            pushdata: byte.pushdata,
        };
        optimized_bytecode.push(byte_code_index);
        code_index += 1;

        // If both opcodes remain, go to next opcode
        if peeped_bytes.len() == 2 {
            increment += 1;
        } else {
            // If any opcodes removed, go to opcode after peephole
            increment += 2;
        }

        i += increment;
    }

    optimized_bytecode
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_optimize() {
        let bytecode: Bytecode = vec![
            ByteData {
                code_index: 0,
                opcode: Some(Opcode::Push2),
                pushdata: Some(String::from("8080")),
            },
            ByteData {
                code_index: 3,
                opcode: Some(Opcode::Dup1),
                pushdata: None,
            },
            ByteData {
                code_index: 4,
                opcode: Some(Opcode::Xor),
                pushdata: None,
            },
            ByteData {
                code_index: 5,
                opcode: Some(Opcode::Push1),
                pushdata: Some(String::from("54")),
            },
            ByteData {
                code_index: 7,
                opcode: Some(Opcode::Swap1),
                pushdata: None,
            },
            ByteData {
                code_index: 8,
                opcode: Some(Opcode::Add),
                pushdata: None,
            },
        ];
        let optimized_bytecode: Bytecode = vec![
            ByteData {
                code_index: 0,
                opcode: Some(Opcode::Push2),
                pushdata: Some(String::from("8080")),
            },
            ByteData {
                code_index: 3,
                opcode: Some(Opcode::Push1),
                pushdata: Some(String::from("00")),
            },
            ByteData {
                code_index: 5,
                opcode: Some(Opcode::Push1),
                pushdata: Some(String::from("54")),
            },
            ByteData {
                code_index: 7,
                opcode: Some(Opcode::Add),
                pushdata: None,
            },
        ];
        assert_eq!(optimized_bytecode, optimize(&bytecode));
    }
}
