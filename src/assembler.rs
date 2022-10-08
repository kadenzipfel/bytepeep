use crate::{evm::*, types::*};

pub fn assemble(bytecode: &Bytecode) -> String {
    let mut byte_string: String = String::from("0x");
    for byte in bytecode {
        byte_string.push_str(String::from(byte.opcode).as_str());

        if !byte.pushdata.is_none() {
            byte_string.push_str(byte.pushdata.clone().unwrap().as_str());
        }
    }
    byte_string
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_assemble() {
        let bytecode: Bytecode = vec![
            ByteData {
                code_index: 0,
                opcode: Opcode::Push1,
                pushdata: Some(String::from("80")),
            },
            ByteData {
                code_index: 2,
                opcode: Opcode::Push1,
                pushdata: Some(String::from("54")),
            },
        ];
        let byte_string = String::from("0x60806054");
        assert_eq!(byte_string, assemble(&bytecode));
    }
}
