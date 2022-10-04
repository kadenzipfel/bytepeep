use crate::{evm::*, types::*};

pub fn check_rules(peephole: &Bytecode) -> Bytecode {
    let new_bytecode: Bytecode = match peephole[..] {
        // Redundant swaps on non-commutative operations
        [ByteData {opcode: Some(Opcode::Swap1), .. }, ByteData {opcode: Some(Opcode::Add), .. }] => peephole[1..].to_vec(),
        _ => peephole[..].to_vec()
    };
    new_bytecode
}