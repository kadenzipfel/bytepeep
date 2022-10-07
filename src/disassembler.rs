use crate::evm::*;
use crate::types::*;

pub fn match_push_n(opcode: Opcode) -> usize {
    match opcode {
        Opcode::Push1 => 1,
        Opcode::Push2 => 2,
        Opcode::Push3 => 3,
        Opcode::Push4 => 4,
        Opcode::Push5 => 5,
        Opcode::Push6 => 6,
        Opcode::Push7 => 7,
        Opcode::Push8 => 8,
        Opcode::Push9 => 9,
        Opcode::Push10 => 10,
        Opcode::Push11 => 11,
        Opcode::Push12 => 12,
        Opcode::Push13 => 13,
        Opcode::Push14 => 14,
        Opcode::Push15 => 15,
        Opcode::Push16 => 16,
        Opcode::Push17 => 17,
        Opcode::Push18 => 18,
        Opcode::Push19 => 19,
        Opcode::Push20 => 20,
        Opcode::Push21 => 21,
        Opcode::Push22 => 22,
        Opcode::Push23 => 23,
        Opcode::Push24 => 24,
        Opcode::Push25 => 25,
        Opcode::Push26 => 26,
        Opcode::Push27 => 27,
        Opcode::Push28 => 28,
        Opcode::Push29 => 29,
        Opcode::Push30 => 30,
        Opcode::Push31 => 31,
        Opcode::Push32 => 32,
        _ => 0,
    }
}

fn print_output(bytecode: &Bytecode) {
    for byte in bytecode {
        if byte.pushdata.is_none() {
            println!("{} {}", byte.code_index, byte.opcode.unwrap().op_string());
        } else {
            println!("{} {} {}", byte.code_index, byte.opcode.unwrap().op_string(), byte.pushdata.as_ref().unwrap());
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
            opcode: Some(opcode),
            pushdata: Some(String::from(&trimmed_byte_string[i + 2..i + 2 + bytes_to_push])),
        });
        i += 2;
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
                opcode: Some(Opcode::Push2),
                pushdata: Some(String::from("8080")),
            },
            ByteData {
                code_index: 3,
                opcode: Some(Opcode::Push1),
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
                opcode: Some(Opcode::Push2),
                pushdata: Some(String::from("8080")),
            },
            ByteData {
                code_index: 3,
                opcode: Some(Opcode::Push1),
                pushdata: Some(String::from("54")),
            },
        ];
        assert_eq!(disassembled_bytes, disassemble(&byte_string, false));
    }
}
