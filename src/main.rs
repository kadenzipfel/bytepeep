use std::env;

mod types;
mod evm;

use crate::types::*;
use crate::evm::*;

fn disassemble(byte_string: &String) -> Bytecode {
    let mut pc: usize = 0;
    let trimmed_byte_string: &str;

    if byte_string.starts_with(&String::from("0x")) {
        trimmed_byte_string = &byte_string[2..];
    } else {
        trimmed_byte_string = byte_string;
    }

    (0..trimmed_byte_string.len()).step_by(2).map(|byte| {
        pc += 1;
        ByteData {
            pc: pc - 1,
            bytes: Opcode::new(&trimmed_byte_string[byte..byte + 2])
        }
    }).collect()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let bytecode = &args[1];
    println!("Bytecode: {}", bytecode);

    let bytes = disassemble(bytecode);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_disassemble() {
        let byte_string = String::from("0x60806054");
        let disassembled_bytes: Bytecode = vec![
            ByteData { pc: 0, bytes: Opcode::Push1 }, 
            ByteData { pc: 1, bytes: Opcode::Dup1 }, 
            ByteData { pc: 2, bytes: Opcode::Push1 }, 
            ByteData { pc: 3, bytes: Opcode::Sload }
        ];
        assert_eq!(disassembled_bytes, disassemble(&byte_string));
    }

    #[test]
    fn test_disassemble_no_0x() {
        let byte_string = String::from("60806054");
        let disassembled_bytes: Bytecode = vec![
            ByteData { pc: 0, bytes: Opcode::Push1 }, 
            ByteData { pc: 1, bytes: Opcode::Dup1 }, 
            ByteData { pc: 2, bytes: Opcode::Push1 }, 
            ByteData { pc: 3, bytes: Opcode::Sload }
        ];
        assert_eq!(disassembled_bytes, disassemble(&byte_string));
    }
}