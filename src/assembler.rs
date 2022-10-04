use crate::types::*;
use crate::evm::*;

pub fn assemble(bytecode: &Bytecode) -> String {
    let mut byte_string: String = String::from("0x");
    for byte in bytecode {
        if byte.kind == ByteKind::Opcode {
            byte_string.push_str(String::from(byte.opcode.unwrap()).as_str());
        } else {
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
            ByteData { pc: 0, opcode: Some(Opcode::Push1), pushdata: None, kind: ByteKind::Opcode }, 
            ByteData { pc: 1, opcode: None, pushdata: Some(String::from("80")), kind: ByteKind::PushData }, 
            ByteData { pc: 2, opcode: Some(Opcode::Push1), pushdata: None, kind: ByteKind::Opcode }, 
            ByteData { pc: 3, opcode: None, pushdata: Some(String::from("54")), kind: ByteKind::PushData }
        ];
        let byte_string = String::from("0x60806054");
        assert_eq!(byte_string, assemble(&bytecode));
    }
}