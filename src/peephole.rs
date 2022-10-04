use crate::types::*;
use crate::disassembler::*;
use crate::rules::*;

pub fn optimize(bytecode: Bytecode) -> Result<Bytecode, &'static str> {
    let mut i: usize = 0;
    let mut optimized_bytecode: Bytecode = vec![];

    while i < bytecode.len() {
        // Skip non-opcodes
        if bytecode[i].kind != ByteKind::Opcode {
            optimized_bytecode.push(bytecode[i].clone())
        }

        let next_byte: usize = (match_push_n(bytecode[i].opcode.unwrap()) + i as u32 + 1) as usize;
        if next_byte > bytecode.len() {
            break;
        }

        let bytes: Bytecode = vec![bytecode[i].clone(), bytecode[next_byte].clone()];
        
        let mut increment: usize = 3;

        check_rules(&bytes).into_iter().for_each(|byte| {
            let byte_pc = ByteData {
                pc: optimized_bytecode.len() as u32,
                opcode: byte.opcode,
                pushdata: byte.pushdata,
                kind: byte.kind
            };
            optimized_bytecode.push(byte_pc);
            increment -= 1;
        });

        i += increment;
    }

    Ok(optimized_bytecode)
}