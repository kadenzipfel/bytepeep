use crate::{evm::*, types::*, utils::*};

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

pub fn disassemble(byte_string: &String) -> Bytecode {
    let mut i = 0;
    let mut code_index: usize = 0;
    let trimmed_byte_string: &str;

    if byte_string.starts_with(&String::from("0x")) {
        trimmed_byte_string = &byte_string[2..];
    } else {
        trimmed_byte_string = byte_string;
    }

    let mut bytecode: Bytecode = Vec::new();

    while i < trimmed_byte_string.len() {
        let opcode = Opcode::new(&trimmed_byte_string[i..i + 2]);
        let bytes_to_push = match_push_n(opcode);

        bytecode.push(ByteData {
            code_index: code_index,
            opcode: opcode,
            pushdata: Some(String::from(&trimmed_byte_string[i + 2..i + 2 + bytes_to_push * 2])),
        });
        i += 2 + bytes_to_push * 2;
        code_index += 1 + bytes_to_push;
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
}
