use crate::{evm::*, types::*, utils::*};

// Output disassembled bytecode string
pub fn output(bytecode: &Bytecode) -> String {
    let mut output: String = String::from("");
    for byte in bytecode {
        if byte.pushdata.is_none() {
            output.push_str(format!("\n{} {}", byte.code_index, byte.opcode.op_string()).as_str());
        } else {
            output.push_str(format!("\n{} {} {}", byte.code_index, byte.opcode.op_string(), byte.pushdata.as_ref().unwrap()).as_str());
        }
    }
    output
}

// Disassemble bytecode
pub fn disassemble(byte_string: &String) -> Bytecode {
    let mut i = 0;
    let mut code_index: usize = 0;
    let trimmed_byte_string: &str;

    // Remove leading 0x if present
    if byte_string.starts_with(&String::from("0x")) {
        trimmed_byte_string = &byte_string[2..];
    } else {
        trimmed_byte_string = byte_string;
    }

    let mut bytecode: Bytecode = Vec::new();

    // Grab each opcode and corresponding pushdata if present
    while i < trimmed_byte_string.len() {
        let opcode = Opcode::new(&trimmed_byte_string[i..i + 2]);
        let bytes_to_push = match_push_n(opcode);
        // No pushdata for push0
        if opcode == Opcode::Push0 {
            bytecode.push(ByteData {
                code_index: code_index,
                opcode: opcode,
                pushdata: None,
            });
            i += 2;
            code_index += 1;
        } else {
            bytecode.push(ByteData {
                code_index: code_index,
                opcode: opcode,
                pushdata: Some(String::from(&trimmed_byte_string[i + 2..i + 2 + bytes_to_push * 2])),
            });
            i += 2 + bytes_to_push * 2;
            code_index += 1 + bytes_to_push;
        }
    }

    bytecode
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_disassemble() {
        let byte_string = String::from("0x6180806054");
        let disassembled_bytes: Bytecode = vec![
            ByteData {
                code_index: 0,
                opcode: Opcode::Push2,
                pushdata: Some(String::from("8080")),
            },
            ByteData {
                code_index: 3,
                opcode: Opcode::Push1,
                pushdata: Some(String::from("54")),
            },
        ];
        assert_eq!(disassembled_bytes, disassemble(&byte_string));
    }

    #[test]
    fn test_disassemble_no_0x() {
        let byte_string = String::from("6180806054");
        let disassembled_bytes: Bytecode = vec![
            ByteData {
                code_index: 0,
                opcode: Opcode::Push2,
                pushdata: Some(String::from("8080")),
            },
            ByteData {
                code_index: 3,
                opcode: Opcode::Push1,
                pushdata: Some(String::from("54")),
            },
        ];
        assert_eq!(disassembled_bytes, disassemble(&byte_string));
    }

    #[test]
    fn test_disassemble_push0() {
        // Test PUSH0 followed by PUSH1 01 to verify both regular push and Push0 work
        let byte_string = String::from("5f6001");  // PUSH0 PUSH1 01
        let disassembled_bytes: Bytecode = vec![
            ByteData {
                code_index: 0,
                opcode: Opcode::Push0,
                pushdata: None,
            },
            ByteData {
                code_index: 1,
                opcode: Opcode::Push1,
                pushdata: Some(String::from("01")),
            },
        ];
        assert_eq!(disassembled_bytes, disassemble(&byte_string));
    }
}
