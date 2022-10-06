use crate::disassembler::*;
use crate::evm::*;
use crate::rules::*;
use crate::types::*;

pub fn optimize(bytecode: &Bytecode) -> Bytecode {
    let mut i: usize = 0;
    let mut optimized_bytecode: Bytecode = vec![];

    while i < bytecode.len() {
        // Skip non-opcodes
        if bytecode[i].kind != ByteKind::Opcode {
            let byte = bytecode[i].clone();
            optimized_bytecode.push(ByteData {
                pc: optimized_bytecode.len() as u32,
                opcode: byte.opcode,
                pushdata: byte.pushdata,
                kind: byte.kind,
            });
            i += 1;
            continue;
        }

        let push_data_size: usize = match_push_n(bytecode[i].opcode.unwrap()) as usize;
        let mut increment: usize = push_data_size;
        let next_op: usize = (push_data_size + i + 1) as usize;

        // If current opcode is last, push byte and subsequent pushdata if existent
        if next_op >= bytecode.len() {
            let byte = bytecode[i].clone();
            optimized_bytecode.push(ByteData {
                pc: optimized_bytecode.len() as u32,
                opcode: byte.opcode,
                pushdata: byte.pushdata,
                kind: byte.kind,
            });

            if push_data_size > 0 {
                for j in 0..push_data_size {
                    let push_byte = bytecode[i + j + 1].clone();

                    optimized_bytecode.push(ByteData {
                        pc: optimized_bytecode.len() as u32,
                        opcode: push_byte.opcode,
                        pushdata: push_byte.pushdata,
                        kind: push_byte.kind,
                    })
                }
            }

            break;
        }

        // Grab two byte peephole
        let bytes: Bytecode = vec![bytecode[i].clone(), bytecode[next_op].clone()];

        // Grab corresponding pushdata if present
        let mut pushdata: Vec<Option<Bytecode>> = Vec::new();
        if push_data_size > 0 {
            pushdata.push(Some(bytecode[i + 1..i + 1 + push_data_size].to_vec()));
        } else {
            pushdata.push(None);
        }
        let push_data_size_1 = match_push_n(bytes[1].opcode.unwrap()) as usize;
        if push_data_size_1 > 0 {
            pushdata.push(Some(bytecode[i + 2 + push_data_size..i + 2 + push_data_size + push_data_size_1].to_vec()));
        } else {
            pushdata.push(None);
        }

        // Check peephole for rule violations, and place first optimized byte in bytecode
        let peeped_bytes = check_rules(&bytes, pushdata);
        let byte: ByteData = peeped_bytes[0].clone();
        let byte_pc = ByteData {
            pc: optimized_bytecode.len() as u32,
            opcode: byte.opcode,
            pushdata: byte.pushdata,
            kind: byte.kind,
        };
        optimized_bytecode.push(byte_pc);

        // If pushdata returned from rule check, append to bytecode
        // TODO: Support more than 1 byte of pushdata
        if peeped_bytes.len() > 1 && peeped_bytes[1].kind == ByteKind::PushData {
            let push_byte = peeped_bytes[1].clone();
            optimized_bytecode.push(ByteData {
                pc: optimized_bytecode.len() as u32,
                opcode: push_byte.opcode,
                pushdata: push_byte.pushdata,
                kind: push_byte.kind
            })
        }

        // Place any trailing pushdata back in the bytecode
        if push_data_size > 0 {
            for j in 0..push_data_size {
                let push_byte = bytecode[i + j + 1].clone();

                optimized_bytecode.push(ByteData {
                    pc: optimized_bytecode.len() as u32,
                    opcode: push_byte.opcode,
                    pushdata: push_byte.pushdata,
                    kind: push_byte.kind,
                })
            }
        }

        // If both opcodes remain, go to next opcode
        if peeped_bytes.len() == 2 && peeped_bytes[1].kind == ByteKind::Opcode {
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
                opcode: Some(Opcode::Dup1),
                pushdata: None,
                kind: ByteKind::Opcode
            },
            ByteData {
                pc: 3,
                opcode: Some(Opcode::Xor),
                pushdata: None,
                kind: ByteKind::Opcode
            },
            ByteData {
                pc: 4,
                opcode: Some(Opcode::Push1),
                pushdata: None,
                kind: ByteKind::Opcode,
            },
            ByteData {
                pc: 5,
                opcode: None,
                pushdata: Some(String::from("54")),
                kind: ByteKind::PushData,
            },
            ByteData {
                pc: 6,
                opcode: Some(Opcode::Swap1),
                pushdata: None,
                kind: ByteKind::Opcode,
            },
            ByteData {
                pc: 7,
                opcode: Some(Opcode::Add),
                pushdata: None,
                kind: ByteKind::Opcode,
            },
        ];
        let optimized_bytecode: Bytecode = vec![
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
                kind: ByteKind::Opcode
            },
            ByteData {
                pc: 3,
                opcode: None,
                pushdata: Some(String::from("00")),
                kind: ByteKind::PushData,
            },
            ByteData {
                pc: 4,
                opcode: Some(Opcode::Push1),
                pushdata: None,
                kind: ByteKind::Opcode,
            },
            ByteData {
                pc: 5,
                opcode: None,
                pushdata: Some(String::from("54")),
                kind: ByteKind::PushData,
            },
            ByteData {
                pc: 6,
                opcode: Some(Opcode::Add),
                pushdata: None,
                kind: ByteKind::Opcode,
            },
        ];
        assert_eq!(optimized_bytecode, optimize(&bytecode));
    }
}
