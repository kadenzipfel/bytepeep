use crate::types::*;
use crate::disassembler::*;
use crate::rules::*;
use crate::evm::*;

pub fn optimize(bytecode: Bytecode) -> Bytecode {
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
                kind: byte.kind
            });
            i += 1;
            continue;
        }

        let mut push_data_size: usize = match_push_n(bytecode[i].opcode.unwrap()) as usize;
        let increment: usize = push_data_size + 2;
        let next_op: usize = (push_data_size + i + 1) as usize;

        // If current opcode is last, push byte and subsequent pushdata if existent
        if next_op >= bytecode.len() {
            let byte = bytecode[i].clone();
            optimized_bytecode.push(ByteData {
                pc: optimized_bytecode.len() as u32,
                opcode: byte.opcode,
                pushdata: byte.pushdata,
                kind: byte.kind
            });

            if push_data_size > 0 {
                for j in 0..push_data_size {
                    let push_byte = bytecode[i + j + 1].clone();

                    optimized_bytecode.push(ByteData {
                        pc: optimized_bytecode.len() as u32,
                        opcode: push_byte.opcode,
                        pushdata: push_byte.pushdata,
                        kind: push_byte.kind
                    })
                }
            }

            break;
        }

        // Grab two byte peephole
        let bytes: Bytecode = vec![bytecode[i].clone(), bytecode[next_op].clone()];

        // Check peephole for rule violations, correct, and place back in bytecode
        check_rules(&bytes).into_iter().for_each(|byte| {
            let byte_pc = ByteData {
                pc: optimized_bytecode.len() as u32,
                opcode: byte.opcode,
                pushdata: byte.pushdata,
                kind: byte.kind
            };
            optimized_bytecode.push(byte_pc);

            if push_data_size > 0 {
                for j in 0..push_data_size {
                    let push_byte = bytecode[i + j + 1].clone();

                    optimized_bytecode.push(ByteData {
                        pc: optimized_bytecode.len() as u32,
                        opcode: push_byte.opcode,
                        pushdata: push_byte.pushdata,
                        kind: push_byte.kind
                    })
                }

                push_data_size = 0;
            }
        });

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
            ByteData { pc: 0, opcode: Some(Opcode::Push1), pushdata: None, kind: ByteKind::Opcode }, 
            ByteData { pc: 1, opcode: None, pushdata: Some(String::from("80")), kind: ByteKind::PushData }, 
            ByteData { pc: 2, opcode: Some(Opcode::Push1), pushdata: None, kind: ByteKind::Opcode }, 
            ByteData { pc: 3, opcode: None, pushdata: Some(String::from("54")), kind: ByteKind::PushData },
            ByteData { pc: 4, opcode: Some(Opcode::Swap1), pushdata: None, kind: ByteKind::Opcode }, 
            ByteData { pc: 5, opcode: Some(Opcode::Add), pushdata: None, kind: ByteKind::PushData }
        ];
        let optimized_bytecode: Bytecode = vec![
            ByteData { pc: 0, opcode: Some(Opcode::Push1), pushdata: None, kind: ByteKind::Opcode }, 
            ByteData { pc: 1, opcode: None, pushdata: Some(String::from("80")), kind: ByteKind::PushData }, 
            ByteData { pc: 2, opcode: Some(Opcode::Push1), pushdata: None, kind: ByteKind::Opcode }, 
            ByteData { pc: 3, opcode: None, pushdata: Some(String::from("54")), kind: ByteKind::PushData },
            ByteData { pc: 4, opcode: Some(Opcode::Add), pushdata: None, kind: ByteKind::PushData }
        ];
        assert_eq!(optimized_bytecode, optimize(bytecode));
    }
}