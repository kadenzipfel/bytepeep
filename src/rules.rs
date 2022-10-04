use crate::{evm::*, types::*};

pub fn check_rules(peephole: &Bytecode) -> Bytecode {
    let new_bytecode: Bytecode = match peephole[..] {
        // Redundant swaps on commutative operations
        [
            ByteData {opcode: Some(Opcode::Swap1), .. }, 
            ByteData {opcode: Some(Opcode::Add), .. }
        ] => peephole[1..].to_vec(),
        [
            ByteData {opcode: Some(Opcode::Swap1), .. }, 
            ByteData {opcode: Some(Opcode::Mul), .. }
        ] => peephole[1..].to_vec(),
        [
            ByteData {opcode: Some(Opcode::Swap1), .. }, 
            ByteData {opcode: Some(Opcode::Eq), .. }
        ] => peephole[1..].to_vec(),
        [
            ByteData {opcode: Some(Opcode::Swap1), .. }, 
            ByteData {opcode: Some(Opcode::And), .. }
        ] => peephole[1..].to_vec(),
        [
            ByteData {opcode: Some(Opcode::Swap1), .. }, 
            ByteData {opcode: Some(Opcode::Or), .. }
        ] => peephole[1..].to_vec(),
        [
            ByteData {opcode: Some(Opcode::Swap1), .. }, 
            ByteData {opcode: Some(Opcode::Xor), .. }
        ] => peephole[1..].to_vec(),
        _ => peephole[..].to_vec()
    };
    new_bytecode
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_commutative_swaps() {
        // Swap1, Add => Add
        let peephole: Bytecode = vec![
            ByteData { pc: 4, opcode: Some(Opcode::Swap1), pushdata: None, kind: ByteKind::Opcode }, 
            ByteData { pc: 5, opcode: Some(Opcode::Add), pushdata: None, kind: ByteKind::PushData }
        ];
        let optimized_peephole: Bytecode = vec![
            ByteData { pc: 5, opcode: Some(Opcode::Add), pushdata: None, kind: ByteKind::PushData }
        ];
        assert_eq!(optimized_peephole, check_rules(&peephole));
        
        // Swap1, Mul => Mul
        let peephole: Bytecode = vec![
            ByteData { pc: 4, opcode: Some(Opcode::Swap1), pushdata: None, kind: ByteKind::Opcode }, 
            ByteData { pc: 5, opcode: Some(Opcode::Mul), pushdata: None, kind: ByteKind::PushData }
        ];
        let optimized_peephole: Bytecode = vec![
            ByteData { pc: 5, opcode: Some(Opcode::Mul), pushdata: None, kind: ByteKind::PushData }
        ];
        assert_eq!(optimized_peephole, check_rules(&peephole));
        
        // Swap1, Eq => Eq
        let peephole: Bytecode = vec![
            ByteData { pc: 4, opcode: Some(Opcode::Swap1), pushdata: None, kind: ByteKind::Opcode }, 
            ByteData { pc: 5, opcode: Some(Opcode::Eq), pushdata: None, kind: ByteKind::PushData }
        ];
        let optimized_peephole: Bytecode = vec![
            ByteData { pc: 5, opcode: Some(Opcode::Eq), pushdata: None, kind: ByteKind::PushData }
        ];
        assert_eq!(optimized_peephole, check_rules(&peephole));

        // Swap1, And => And
        let peephole: Bytecode = vec![
            ByteData { pc: 4, opcode: Some(Opcode::Swap1), pushdata: None, kind: ByteKind::Opcode }, 
            ByteData { pc: 5, opcode: Some(Opcode::And), pushdata: None, kind: ByteKind::PushData }
        ];
        let optimized_peephole: Bytecode = vec![
            ByteData { pc: 5, opcode: Some(Opcode::And), pushdata: None, kind: ByteKind::PushData }
        ];
        assert_eq!(optimized_peephole, check_rules(&peephole));

        // Swap1, Or => Or
        let peephole: Bytecode = vec![
            ByteData { pc: 4, opcode: Some(Opcode::Swap1), pushdata: None, kind: ByteKind::Opcode }, 
            ByteData { pc: 5, opcode: Some(Opcode::Or), pushdata: None, kind: ByteKind::PushData }
        ];
        let optimized_peephole: Bytecode = vec![
            ByteData { pc: 5, opcode: Some(Opcode::Or), pushdata: None, kind: ByteKind::PushData }
        ];
        assert_eq!(optimized_peephole, check_rules(&peephole));

        // Swap1, Xor => Xor
        let peephole: Bytecode = vec![
            ByteData { pc: 4, opcode: Some(Opcode::Swap1), pushdata: None, kind: ByteKind::Opcode }, 
            ByteData { pc: 5, opcode: Some(Opcode::Xor), pushdata: None, kind: ByteKind::PushData }
        ];
        let optimized_peephole: Bytecode = vec![
            ByteData { pc: 5, opcode: Some(Opcode::Xor), pushdata: None, kind: ByteKind::PushData }
        ];
        assert_eq!(optimized_peephole, check_rules(&peephole));
    }
}