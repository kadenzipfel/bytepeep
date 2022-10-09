use crate::{evm::*, rules::*, types::*};

// Optimize bytecode by creating peepholes and running rule checks
pub fn optimize(bytecode: &Bytecode) -> Bytecode {
    let mut i: usize = 0;
    let mut code_index: usize = 0;
    let mut optimized_bytecode: Bytecode = vec![];

    while i < bytecode.len() {
        let mut increment = 0;

        // If current opcode is last, push byte
        if i + 1 >= bytecode.len() {
            let byte = bytecode[i].clone();
            optimized_bytecode.push(ByteData {
                code_index: code_index,
                opcode: byte.opcode,
                pushdata: byte.pushdata,
            });

            break;
        }

        // Grab two byte peephole
        let mut bytes: Bytecode = vec![bytecode[i].clone(), bytecode[i + 1].clone()];

        // Check peephole for rule violations, and place first optimized byte in bytecode
        let peeped_bytes = check_rules(&mut bytes);

        // Handle both bytes removed
        if peeped_bytes.len() == 0 {
            i += 2;
            continue;
        }

        let byte: ByteData = peeped_bytes[0].clone();
        let byte_code_index = ByteData {
            code_index: code_index,
            opcode: byte.opcode,
            pushdata: byte.pushdata,
        };
        optimized_bytecode.push(byte_code_index);
        let mut push_data_size: usize = 0;
        if !peeped_bytes[0].clone().pushdata.is_none() {
            push_data_size = peeped_bytes[0].clone().pushdata.unwrap().len() / 2;
        }
        code_index += 1 + push_data_size;

        if peeped_bytes.len() == 2 {
            let byte: ByteData = peeped_bytes[1].clone();
            // If second byte returned different from input, push to optimized bytecode vector
            if byte.opcode != bytecode[i + 1].clone().opcode {
                optimized_bytecode.push(ByteData {
                    code_index: code_index,
                    opcode: byte.opcode,
                    pushdata: byte.pushdata,
                });

                if !peeped_bytes[1].clone().pushdata.is_none() {
                    push_data_size = peeped_bytes[1].clone().pushdata.unwrap().len() / 2;
                } else {
                    push_data_size = 0
                }

                code_index += 1 + push_data_size;
                increment += 2;
            } else {
                increment += 1;
            }
        } else {
            increment += 2;
        }

        i += increment;
    }

    optimized_bytecode
}

mod tests {
    use super::*;

    #[test]
    fn test_optimize() {
        let bytecode: Bytecode = vec![
            ByteData {
                code_index: 0,
                opcode: Opcode::Push2,
                pushdata: Some(String::from("0080")),
            },
            ByteData {
                code_index: 3,
                opcode: Opcode::Dup1,
                pushdata: None,
            },
            ByteData {
                code_index: 4,
                opcode: Opcode::Xor,
                pushdata: None,
            },
            ByteData {
                code_index: 5,
                opcode: Opcode::Not,
                pushdata: None,
            },
            ByteData {
                code_index: 6,
                opcode: Opcode::Not,
                pushdata: None,
            },
            ByteData {
                code_index: 7,
                opcode: Opcode::Push2,
                pushdata: Some(String::from("8080")),
            },
            ByteData {
                code_index: 10,
                opcode: Opcode::Push2,
                pushdata: Some(String::from("8080")),
            },
            ByteData {
                code_index: 13,
                opcode: Opcode::Push1,
                pushdata: Some(String::from("54")),
            },
            ByteData {
                code_index: 14,
                opcode: Opcode::Swap1,
                pushdata: None,
            },
            ByteData {
                code_index: 15,
                opcode: Opcode::Add,
                pushdata: None,
            },
        ];
        let optimized_bytecode: Bytecode = vec![
            ByteData {
                code_index: 0,
                opcode: Opcode::Push1,
                pushdata: Some(String::from("80")),
            },
            ByteData {
                code_index: 2,
                opcode: Opcode::Push1,
                pushdata: Some(String::from("00")),
            },
            ByteData {
                code_index: 4,
                opcode: Opcode::Push2,
                pushdata: Some(String::from("8080")),
            },
            ByteData {
                code_index: 7,
                opcode: Opcode::Dup1,
                pushdata: None,
            },
            ByteData {
                code_index: 8,
                opcode: Opcode::Push1,
                pushdata: Some(String::from("54")),
            },
            ByteData {
                code_index: 10,
                opcode: Opcode::Add,
                pushdata: None,
            },
        ];
        assert_eq!(optimized_bytecode, optimize(&bytecode));
    }
}
