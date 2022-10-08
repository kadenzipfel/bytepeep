use crate::{evm::*, types::*};

pub fn check_rules(peephole: &Bytecode) -> Bytecode {
    let new_bytecode: Bytecode = match peephole[..] {
        // Redundant swaps on commutative operations
        [ByteData {
            opcode: Opcode::Swap1,
            ..
        }, ByteData {
            opcode: Opcode::Add,
            ..
        }] => peephole[1..].to_vec(),
        [ByteData {
            opcode: Opcode::Swap1,
            ..
        }, ByteData {
            opcode: Opcode::Mul,
            ..
        }] => peephole[1..].to_vec(),
        [ByteData {
            opcode: Opcode::Swap1,
            ..
        }, ByteData {
            opcode: Opcode::Eq,
            ..
        }] => peephole[1..].to_vec(),
        [ByteData {
            opcode: Opcode::Swap1,
            ..
        }, ByteData {
            opcode: Opcode::And,
            ..
        }] => peephole[1..].to_vec(),
        [ByteData {
            opcode: Opcode::Swap1,
            ..
        }, ByteData {
            opcode: Opcode::Or,
            ..
        }] => peephole[1..].to_vec(),
        [ByteData {
            opcode: Opcode::Swap1,
            ..
        }, ByteData {
            opcode: Opcode::Xor,
            ..
        }] => peephole[1..].to_vec(),

        // Operations involving an expression and itself
        [ByteData {
            opcode: Opcode::Dup1,
            ..
        }, ByteData {
            opcode: Opcode::And,
            ..
        }] => [].to_vec(),
        [ByteData {
            opcode: Opcode::Dup1,
            ..
        }, ByteData {
            opcode: Opcode::Or,
            ..
        }] => [].to_vec(),
        [ByteData {
            opcode: Opcode::Dup1,
            ..
        }, ByteData {
            opcode: Opcode::Xor,
            ..
        }] => [
            ByteData {
                code_index: peephole[0].code_index,
                opcode: Opcode::Push1,
                pushdata: Some(String::from("00")),
            }
        ].to_vec(),
        [ByteData {
            opcode: Opcode::Dup1,
            ..
        }, ByteData {
            opcode: Opcode::Sub,
            ..
        }] => [
            ByteData {
                code_index: peephole[0].code_index,
                opcode: Opcode::Push1,
                pushdata: Some(String::from("00")),
            }
        ].to_vec(),
        [ByteData {
            opcode: Opcode::Dup1,
            ..
        }, ByteData {
            opcode: Opcode::Eq,
            ..
        }] => [
            ByteData {
                code_index: peephole[0].code_index,
                opcode: Opcode::Push1,
                pushdata: Some(String::from("01")),
            },
        ].to_vec(),
        [ByteData {
            opcode: Opcode::Dup1,
            ..
        }, ByteData {
            opcode: Opcode::Lt,
            ..
        }] => [
            ByteData {
                code_index: peephole[0].code_index,
                opcode: Opcode::Push1,
                pushdata: Some(String::from("00")),
            },
        ].to_vec(),
        [ByteData {
            opcode: Opcode::Dup1,
            ..
        }, ByteData {
            opcode: Opcode::Slt,
            ..
        }] => [
            ByteData {
                code_index: peephole[0].code_index,
                opcode: Opcode::Push1,
                pushdata: Some(String::from("00")),
            },
        ].to_vec(),
        [ByteData {
            opcode: Opcode::Dup1,
            ..
        }, ByteData {
            opcode: Opcode::Gt,
            ..
        }] => [
            ByteData {
                code_index: peephole[0].code_index,
                opcode: Opcode::Push1,
                pushdata: Some(String::from("00")),
            },
        ].to_vec(),
        [ByteData {
            opcode: Opcode::Dup1,
            ..
        }, ByteData {
            opcode: Opcode::Sgt,
            ..
        }] => [
            ByteData {
                code_index: peephole[0].code_index,
                opcode: Opcode::Push1,
                pushdata: Some(String::from("00")),
            },
        ].to_vec(),
        [ByteData {
            opcode: Opcode::Dup1,
            ..
        }, ByteData {
            opcode: Opcode::Mod,
            ..
        }] => [
            ByteData {
                code_index: peephole[0].code_index,
                opcode: Opcode::Push1,
                pushdata: Some(String::from("00")),
            },
        ].to_vec(),

        // Duplicate pushes
        [ByteData {
            opcode: Opcode::Push1,
            ..
        }, ByteData {
            opcode: Opcode::Push1,
            ..
        }] if peephole[0].pushdata.as_ref().unwrap() == peephole[1].pushdata.as_ref().unwrap() => [
            ByteData {
                code_index: peephole[0].code_index,
                opcode: peephole[0].opcode,
                pushdata: Some(peephole[0].pushdata.as_ref().unwrap().to_string()),
            },
            ByteData {
                code_index: peephole[1].code_index + 1,
                opcode: Opcode::Dup1,
                pushdata: None,
            },
        ].to_vec(),
        _ => peephole[..].to_vec(),
    };
    new_bytecode
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_commutative_swaps() {
        // Swap1, Add => Add
        let peephole: Bytecode = vec![
            ByteData {
                code_index: 4,
                opcode: Opcode::Swap1,
                pushdata: None,
            },
            ByteData {
                code_index: 5,
                opcode: Opcode::Add,
                pushdata: None,
            },
        ];
        let optimized_peephole: Bytecode = vec![ByteData {
            code_index: 5,
            opcode: Opcode::Add,
            pushdata: None,
        }];
        assert_eq!(optimized_peephole, check_rules(&peephole));

        // Swap1, Mul => Mul
        let peephole: Bytecode = vec![
            ByteData {
                code_index: 4,
                opcode: Opcode::Swap1,
                pushdata: None,
            },
            ByteData {
                code_index: 5,
                opcode: Opcode::Mul,
                pushdata: None,
            },
        ];
        let optimized_peephole: Bytecode = vec![ByteData {
            code_index: 5,
            opcode: Opcode::Mul,
            pushdata: None,
        }];
        assert_eq!(optimized_peephole, check_rules(&peephole));

        // Swap1, Eq => Eq
        let peephole: Bytecode = vec![
            ByteData {
                code_index: 4,
                opcode: Opcode::Swap1,
                pushdata: None,
            },
            ByteData {
                code_index: 5,
                opcode: Opcode::Eq,
                pushdata: None,
            },
        ];
        let optimized_peephole: Bytecode = vec![ByteData {
            code_index: 5,
            opcode: Opcode::Eq,
            pushdata: None,
        }];
        assert_eq!(optimized_peephole, check_rules(&peephole));

        // Swap1, And => And
        let peephole: Bytecode = vec![
            ByteData {
                code_index: 4,
                opcode: Opcode::Swap1,
                pushdata: None,
            },
            ByteData {
                code_index: 5,
                opcode: Opcode::And,
                pushdata: None,
            },
        ];
        let optimized_peephole: Bytecode = vec![ByteData {
            code_index: 5,
            opcode: Opcode::And,
            pushdata: None,
        }];
        assert_eq!(optimized_peephole, check_rules(&peephole));

        // Swap1, Or => Or
        let peephole: Bytecode = vec![
            ByteData {
                code_index: 4,
                opcode: Opcode::Swap1,
                pushdata: None,
            },
            ByteData {
                code_index: 5,
                opcode: Opcode::Or,
                pushdata: None,
            },
        ];
        let optimized_peephole: Bytecode = vec![ByteData {
            code_index: 5,
            opcode: Opcode::Or,
            pushdata: None,
        }];
        assert_eq!(optimized_peephole, check_rules(&peephole));

        // Swap1, Xor => Xor
        let peephole: Bytecode = vec![
            ByteData {
                code_index: 4,
                opcode: Opcode::Swap1,
                pushdata: None,
            },
            ByteData {
                code_index: 5,
                opcode: Opcode::Xor,
                pushdata: None,
            },
        ];
        let optimized_peephole: Bytecode = vec![ByteData {
            code_index: 5,
            opcode: Opcode::Xor,
            pushdata: None,
        }];
        assert_eq!(optimized_peephole, check_rules(&peephole));
    }

    #[test]
    fn test_dup_expression_operations() {
        // Dup1, And => []
        let peephole = vec![ByteData {
            code_index: 4,
            opcode: Opcode::Dup1,
            pushdata: None,
        }, ByteData {
            code_index: 5,
            opcode: Opcode::And,
            pushdata: None,
        }];
        let optimized_peephole: Vec<ByteData> = Vec::new();
        assert_eq!(optimized_peephole, check_rules(&peephole));

        // Dup1, Or => []
        let peephole = vec![ByteData {
            code_index: 4,
            opcode: Opcode::Dup1,
            pushdata: None,
        }, ByteData {
            code_index: 5,
            opcode: Opcode::Or,
            pushdata: None,
        }];
        let optimized_peephole: Vec<ByteData> = Vec::new();
        assert_eq!(optimized_peephole, check_rules(&peephole));

        // Dup1, Xor => Push1, 00
        let peephole = vec![ByteData {
            code_index: 4,
            opcode: Opcode::Dup1,
            pushdata: None,
        }, ByteData {
            code_index: 5,
            opcode: Opcode::Xor,
            pushdata: None,
        }];
        let optimized_peephole = vec![ByteData {
            code_index: 4,
            opcode: Opcode::Push1,
            pushdata: Some(String::from("00")),
        }];
        assert_eq!(optimized_peephole, check_rules(&peephole));

        // Dup1, Sub => Push1, 00
        let peephole = vec![ByteData {
            code_index: 4,
            opcode: Opcode::Dup1,
            pushdata: None,
        }, ByteData {
            code_index: 5,
            opcode: Opcode::Sub,
            pushdata: None,
        }];
        let optimized_peephole = vec![ByteData {
            code_index: 4,
            opcode: Opcode::Push1,
            pushdata: Some(String::from("00")),
        }];
        assert_eq!(optimized_peephole, check_rules(&peephole));

        // Dup1, Eq => Push1, 01
        let peephole = vec![ByteData {
            code_index: 4,
            opcode: Opcode::Dup1,
            pushdata: None,
        }, ByteData {
            code_index: 5,
            opcode: Opcode::Eq,
            pushdata: None,
        }];
        let optimized_peephole = vec![ByteData {
            code_index: 4,
            opcode: Opcode::Push1,
            pushdata: Some(String::from("01")),
        }];
        assert_eq!(optimized_peephole, check_rules(&peephole));

        // Dup1, Lt => Push1, 00
        let peephole = vec![ByteData {
            code_index: 4,
            opcode: Opcode::Dup1,
            pushdata: None,
        }, ByteData {
            code_index: 5,
            opcode: Opcode::Lt,
            pushdata: None,
        }];
        let optimized_peephole = vec![ByteData {
            code_index: 4,
            opcode: Opcode::Push1,
            pushdata: Some(String::from("00")),
        }];
        assert_eq!(optimized_peephole, check_rules(&peephole));

        // Dup1, Slt => Push1, 00
        let peephole = vec![ByteData {
            code_index: 4,
            opcode: Opcode::Dup1,
            pushdata: None,
        }, ByteData {
            code_index: 5,
            opcode: Opcode::Slt,
            pushdata: None,
        }];
        let optimized_peephole = vec![ByteData {
            code_index: 4,
            opcode: Opcode::Push1,
            pushdata: Some(String::from("00")),
        }];
        assert_eq!(optimized_peephole, check_rules(&peephole));

        // Dup1, Gt => Push1, 00
        let peephole = vec![ByteData {
            code_index: 4,
            opcode: Opcode::Dup1,
            pushdata: None,
        }, ByteData {
            code_index: 5,
            opcode: Opcode::Gt,
            pushdata: None,
        }];
        let optimized_peephole = vec![ByteData {
            code_index: 4,
            opcode: Opcode::Push1,
            pushdata: Some(String::from("00")),
        }];
        assert_eq!(optimized_peephole, check_rules(&peephole));

        // Dup1, Sgt => Push1, 00
        let peephole = vec![ByteData {
            code_index: 4,
            opcode: Opcode::Dup1,
            pushdata: None,
        }, ByteData {
            code_index: 5,
            opcode: Opcode::Sgt,
            pushdata: None,
        }];
        let optimized_peephole = vec![ByteData {
            code_index: 4,
            opcode: Opcode::Push1,
            pushdata: Some(String::from("00")),
        }];
        assert_eq!(optimized_peephole, check_rules(&peephole));

        // Dup1, Mod => Push1, 00
        let peephole = vec![ByteData {
            code_index: 4,
            opcode: Opcode::Dup1,
            pushdata: None,
        }, ByteData {
            code_index: 5,
            opcode: Opcode::Mod,
            pushdata: None,
        }];
        let optimized_peephole = vec![ByteData {
            code_index: 4,
            opcode: Opcode::Push1,
            pushdata: Some(String::from("00")),
        }];
        assert_eq!(optimized_peephole, check_rules(&peephole));
    }

    #[test]
    fn test_duplicate_pushes() {
        // Push1, X, Push1, X => Push1, X, Dup1
        let peephole = vec![ByteData {
            code_index: 4,
            opcode: Opcode::Push1,
            pushdata: Some(String::from("00")),
        }, ByteData {
            code_index: 5,
            opcode: Opcode::Push1,
            pushdata: Some(String::from("00")),
        }];
        let optimized_peephole = vec![ByteData {
            code_index: 4,
            opcode: Opcode::Push1,
            pushdata: Some(String::from("00")),
        }, ByteData {
            code_index: 6,
            opcode: Opcode::Dup1,
            pushdata: None,
        }];
        assert_eq!(optimized_peephole, check_rules(&peephole));
    }
}
