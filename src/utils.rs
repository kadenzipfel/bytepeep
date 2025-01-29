use crate::evm::*;
use std::{path::Path, process::Command};

#[derive(Debug, Clone, strum::EnumString)]
#[strum(ascii_case_insensitive)]
pub enum Source {
    Raw,
    Huff,
}

pub fn compile_huff(path: &str) -> Result<String, String> {
    if !Path::new(path).exists() {
        return Err(format!("File not found: {}", path));
    }

    let output = Command::new("huffc")
        .args(["-b", path])
        .output()
        .map_err(|e| format!("Failed to compile the huff code: {}", e))?;

    if !output.status.success() {
        return Err(format!("huffc failed: {}", 
            String::from_utf8_lossy(&output.stderr)));
    }

    let output_str = String::from_utf8_lossy(&output.stdout);

    // Extract the bytecode from last line of the o/p
    let bytecode = output_str
        .lines()
        .last()
        .ok_or_else(|| "No output".to_string())?
        .trim()
        .trim_end_matches('%')
        .to_string();

    Ok(bytecode)
}

// Find minimum viable length for pushdata
pub fn min_pushdata_len(string: &String) -> (usize, String) {
    let mut len = string.len() / 2;
    let mut start = 0;
    for i in 0..len {
        if string[i * 2..i * 2 + 2] == String::from("00") {
            len -= 1;
            start += 2;
        } else {
            break;
        }
    };
    (len, string[start..].to_string())
}

// Get push size from PushN opcode
pub fn match_push_n(opcode: Opcode) -> usize {
    match opcode {
        Opcode::Push0 => 0,
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

mod tests {
    use super::*;

    #[test]
    fn test_min_pushdata_len() {
        let push_string = String::from("10101010");
        assert_eq!((4, String::from("10101010")), min_pushdata_len(&push_string));

        let push_string = String::from("00100010");
        assert_eq!((3, String::from("100010")), min_pushdata_len(&push_string));
    }
}
