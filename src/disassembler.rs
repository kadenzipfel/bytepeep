use crate::evm::*;
use crate::types::*;
use crate::utils::*;

fn print_output(bytecode: &Bytecode) {
    for byte in bytecode {
        if byte.pushdata.is_none() {
            println!("{} {}", byte.code_index, byte.opcode.op_string());
        } else {
            println!("{} {} {}", byte.code_index, byte.opcode.op_string(), byte.pushdata.as_ref().unwrap());
        }
    }
}

pub fn disassemble(byte_string: &String, print: bool) -> Bytecode {
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

    if print {
        print_output(&bytecode);
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
        assert_eq!(disassembled_bytes, disassemble(&byte_string, false));
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
        assert_eq!(disassembled_bytes, disassemble(&byte_string, false));
    }
}
