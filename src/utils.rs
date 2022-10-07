use crate::evm::*;

pub fn is_push_op(opcode: Opcode) -> bool {
    match opcode {
        Opcode::Push1 => true,
        Opcode::Push2 => true,
        Opcode::Push3 => true,
        Opcode::Push4 => true,
        Opcode::Push5 => true,
        Opcode::Push6 => true,
        Opcode::Push7 => true,
        Opcode::Push8 => true,
        Opcode::Push9 => true,
        Opcode::Push10 => true,
        Opcode::Push11 => true,
        Opcode::Push12 => true,
        Opcode::Push13 => true,
        Opcode::Push14 => true,
        Opcode::Push15 => true,
        Opcode::Push16 => true,
        Opcode::Push17 => true,
        Opcode::Push18 => true,
        Opcode::Push19 => true,
        Opcode::Push20 => true,
        Opcode::Push21 => true,
        Opcode::Push22 => true,
        Opcode::Push23 => true,
        Opcode::Push24 => true,
        Opcode::Push25 => true,
        Opcode::Push26 => true,
        Opcode::Push27 => true,
        Opcode::Push28 => true,
        Opcode::Push29 => true,
        Opcode::Push30 => true,
        Opcode::Push31 => true,
        Opcode::Push32 => true,
        _ => false,
    }
}