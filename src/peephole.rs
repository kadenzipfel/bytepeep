use crate::evm::*;
use crate::rules::*;
use crate::types::*;
use crate::utils::*;

pub fn optimize(bytecode: &Bytecode) -> Bytecode {
    let mut i: usize = 0;
    let mut pc: usize = 0;
    let mut optimized_bytecode: Bytecode = vec![];

    while i < bytecode.len() {
        // Skip non-opcodes
        if bytecode[i].kind != ByteKind::Opcode {
            let byte = bytecode[i].clone();
            optimized_bytecode.push(ByteData {
                pc: pc as u32,
                opcode: byte.opcode,
                pushdata: byte.pushdata,
                kind: byte.kind,
            });
            pc += bytecode[i].clone().pushdata.unwrap().len() / 2;
            i += 1;
            continue;
        }

        let is_push = is_push_op(bytecode[i].opcode.unwrap());
        let mut increment = 0;
        let mut next_op = (i + 1) as usize;
        if is_push {
            increment += 1;
            next_op += 1;
        }

        // If current opcode is last, push byte and subsequent pushdata if existent
        if next_op >= bytecode.len() {
            let byte = bytecode[i].clone();
            optimized_bytecode.push(ByteData {
                pc: pc as u32,
                opcode: byte.opcode,
                pushdata: byte.pushdata,
                kind: byte.kind,
            });

            if is_push {
                let push_byte = bytecode[i + 1].clone();

                optimized_bytecode.push(ByteData {
                    pc: (pc as u32) + 1,
                    opcode: push_byte.opcode,
                    pushdata: push_byte.pushdata,
                    kind: push_byte.kind,
                })
            }

            break;
        }

        // Grab two byte peephole
        let bytes: Bytecode = vec![bytecode[i].clone(), bytecode[next_op].clone()];

        // Grab corresponding pushdata if present
        let mut peephole_pushdata: Vec<Option<PushData>> = Vec::new();
        if is_push {
            peephole_pushdata.push(bytecode[i + 1].clone().pushdata);

            if is_push_op(bytecode[i + 2].opcode.unwrap()) {
                peephole_pushdata.push(bytecode[i + 3].clone().pushdata);
            }
        } else {
            if is_push_op(bytecode[i + 1].opcode.unwrap()) {
                peephole_pushdata.push(bytecode[i + 2].clone().pushdata);
            }
        }

        // Check peephole for rule violations, and place first optimized byte in bytecode
        let peeped_bytes = check_rules(&bytes, peephole_pushdata);
        let byte: ByteData = peeped_bytes[0].clone();
        let byte_pc = ByteData {
            pc: pc as u32,
            opcode: byte.opcode,
            pushdata: byte.pushdata,
            kind: byte.kind,
        };
        optimized_bytecode.push(byte_pc);
        pc += 1;

        // If pushdata returned from rule check, append to bytecode
        if peeped_bytes.len() > 1 && peeped_bytes[1].kind == ByteKind::PushData {
            let push_byte = peeped_bytes[1].clone();
            optimized_bytecode.push(ByteData {
                pc: pc as u32,
                opcode: push_byte.opcode,
                pushdata: push_byte.pushdata,
                kind: push_byte.kind
            });
            pc += peeped_bytes[1].clone().pushdata.unwrap().len() / 2;
        } else if is_push {
            // Place any trailing pushdata back in the bytecode
            let push_byte = bytecode[i + 1].clone();
            optimized_bytecode.push(ByteData {
                pc: pc as u32,
                opcode: push_byte.opcode,
                pushdata: push_byte.pushdata,
                kind: push_byte.kind,
            });
            pc += bytecode[i + 1].clone().pushdata.unwrap().len() / 2;
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
                opcode: Some(Opcode::Push2),
                pushdata: None,
                kind: ByteKind::Opcode,
            },
            ByteData {
                pc: 1,
                opcode: None,
                pushdata: Some(String::from("8080")),
                kind: ByteKind::PushData,
            },
            ByteData {
                pc: 3,
                opcode: Some(Opcode::Dup1),
                pushdata: None,
                kind: ByteKind::Opcode
            },
            ByteData {
                pc: 4,
                opcode: Some(Opcode::Xor),
                pushdata: None,
                kind: ByteKind::Opcode
            },
            ByteData {
                pc: 5,
                opcode: Some(Opcode::Push1),
                pushdata: None,
                kind: ByteKind::Opcode,
            },
            ByteData {
                pc: 6,
                opcode: None,
                pushdata: Some(String::from("00")),
                kind: ByteKind::PushData,
            },
            ByteData {
                pc: 7,
                opcode: Some(Opcode::Push1),
                pushdata: None,
                kind: ByteKind::Opcode,
            },
            ByteData {
                pc: 8,
                opcode: None,
                pushdata: Some(String::from("00")),
                kind: ByteKind::PushData,
            },
            ByteData {
                pc: 9,
                opcode: Some(Opcode::Push1),
                pushdata: None,
                kind: ByteKind::Opcode,
            },
            ByteData {
                pc: 10,
                opcode: None,
                pushdata: Some(String::from("54")),
                kind: ByteKind::PushData,
            },
            ByteData {
                pc: 11,
                opcode: Some(Opcode::Swap1),
                pushdata: None,
                kind: ByteKind::Opcode,
            },
            ByteData {
                pc: 12,
                opcode: Some(Opcode::Add),
                pushdata: None,
                kind: ByteKind::Opcode,
            },
        ];
        let optimized_bytecode: Bytecode = vec![
            ByteData {
                pc: 0,
                opcode: Some(Opcode::Push2),
                pushdata: None,
                kind: ByteKind::Opcode,
            },
            ByteData {
                pc: 1,
                opcode: None,
                pushdata: Some(String::from("8080")),
                kind: ByteKind::PushData,
            },
            ByteData {
                pc: 3,
                opcode: Some(Opcode::Push1),
                pushdata: None,
                kind: ByteKind::Opcode
            },
            ByteData {
                pc: 4,
                opcode: None,
                pushdata: Some(String::from("00")),
                kind: ByteKind::PushData,
            },
            ByteData {
                pc: 5,
                opcode: Some(Opcode::Push1),
                pushdata: None,
                kind: ByteKind::Opcode,
            },
            ByteData {
                pc: 6,
                opcode: None,
                pushdata: Some(String::from("00")),
                kind: ByteKind::PushData,
            },
            ByteData {
                pc: 7,
                opcode: Some(Opcode::Dup1),
                pushdata: None,
                kind: ByteKind::Opcode,
            },
            ByteData {
                pc: 8,
                opcode: Some(Opcode::Push1),
                pushdata: None,
                kind: ByteKind::Opcode,
            },
            ByteData {
                pc: 9,
                opcode: None,
                pushdata: Some(String::from("54")),
                kind: ByteKind::PushData,
            },
            ByteData {
                pc: 10,
                opcode: Some(Opcode::Add),
                pushdata: None,
                kind: ByteKind::Opcode,
            },
        ];
        assert_eq!(optimized_bytecode, optimize(&bytecode));
    }
}
