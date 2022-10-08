use crate::evm::*;

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