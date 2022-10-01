use std::env;

fn disassemble(byte_string: &String) -> Vec<(usize, u8)> {
    let mut pc: usize = 0;
    let trimmed_byte_string: &str;

    if byte_string.starts_with(&String::from("0x")) {
        trimmed_byte_string = &byte_string[2..];
    } else {
        trimmed_byte_string = byte_string;
    }

    (0..trimmed_byte_string.len()).step_by(2).map(|byte| {
        let curr_pc = pc;
        pc += 1;
        (curr_pc, u8::from_str_radix(&trimmed_byte_string[byte..byte + 2], 16).unwrap())
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
        assert_eq!(vec![(0, 96), (1, 128), (2, 96), (3, 84)], disassemble(&byte_string));
    }

    #[test]
    fn test_disassemble_no_0x() {
        let byte_string = String::from("60806054");
        assert_eq!(vec![(0, 96), (1, 128), (2, 96), (3, 84)], disassemble(&byte_string));
    }
}