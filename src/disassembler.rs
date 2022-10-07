use crate::evm::*;
use crate::types::*;

pub fn match_push_n(opcode: Opcode) -> u32 {
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
            println!("{} {}", byte.pc, byte.opcode.unwrap().op_string());
        } else {
            println!("{} {} {}", byte.pc, byte.opcode.unwrap().op_string(), byte.pushdata.as_ref().unwrap());
        }
    }
}

pub fn disassemble(byte_string: &String, print: bool) -> Bytecode {
    let mut i = 0;
    let mut pc: u32 = 0;
    let mut bytes_to_push: u32 = 0;
    let trimmed_byte_string: &str;

    if byte_string.starts_with(&String::from("0x")) {
        trimmed_byte_string = &byte_string[2..];
    } else {
        trimmed_byte_string = byte_string;
    }

    let mut bytecode: Bytecode = Vec::new();

    while i < trimmed_byte_string.len() {
        if bytes_to_push > 0 {
            let pushdata = ByteData {
                pc: pc,
                opcode: None,
                pushdata: Some(String::from(&trimmed_byte_string[i..i + bytes_to_push as usize * 2])),
                kind: ByteKind::PushData,
            };

            i += 2 * bytes_to_push as usize;
            pc += bytes_to_push;
            bytes_to_push = 0;
            bytecode.push(pushdata);
            continue
        }

        let opcode = Opcode::new(&trimmed_byte_string[i..i + 2]);
        bytes_to_push = match_push_n(opcode);

        bytecode.push(ByteData {
            pc: pc,
            opcode: Some(opcode),
            pushdata: None,
            kind: ByteKind::Opcode,
        });
        i += 2;
        pc += 1;
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
                pc: 0,
                opcode: Some(Opcode::Push2),
                pushdata: None,
                kind: ByteKind::Opcode,
            },
            ByteData {
                pc: 1,
                opcode: None,
                pushdata: Some(String::from("8080")),
                kind: ByteKind::PushData,
            },
            ByteData {
                pc: 3,
                opcode: Some(Opcode::Push1),
                pushdata: None,
                kind: ByteKind::Opcode,
            },
            ByteData {
                pc: 4,
                opcode: None,
                pushdata: Some(String::from("54")),
                kind: ByteKind::PushData,
            },
        ];
        assert_eq!(disassembled_bytes, disassemble(&byte_string, false));
    }

    #[test]
    fn test_disassemble_no_0x() {
        let byte_string = String::from("6180806054");
        let disassembled_bytes: Bytecode = vec![
            ByteData {
                pc: 0,
                opcode: Some(Opcode::Push2),
                pushdata: None,
                kind: ByteKind::Opcode,
            },
            ByteData {
                pc: 1,
                opcode: None,
                pushdata: Some(String::from("8080")),
                kind: ByteKind::PushData,
            },
            ByteData {
                pc: 3,
                opcode: Some(Opcode::Push1),
                pushdata: None,
                kind: ByteKind::Opcode,
            },
            ByteData {
                pc: 4,
                opcode: None,
                pushdata: Some(String::from("54")),
                kind: ByteKind::PushData,
            },
        ];
        assert_eq!(disassembled_bytes, disassemble(&byte_string, false));
    }
}
