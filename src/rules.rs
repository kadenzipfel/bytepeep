use crate::{evm::*, types::*};

pub fn check_rules(peephole: &Bytecode) -> Bytecode {
    let new_bytecode: Bytecode = match peephole[..] {
        // Redundant swaps on commutative operations
        [ByteData {
            opcode: Some(Opcode::Swap1),
            ..
        }, ByteData {
            opcode: Some(Opcode::Add),
            ..
        }] => peephole[1..].to_vec(),
        [ByteData {
            opcode: Some(Opcode::Swap1),
            ..
        }, ByteData {
            opcode: Some(Opcode::Mul),
            ..
        }] => peephole[1..].to_vec(),
        [ByteData {
            opcode: Some(Opcode::Swap1),
            ..
        }, ByteData {
            opcode: Some(Opcode::Eq),
            ..
        }] => peephole[1..].to_vec(),
        [ByteData {
            opcode: Some(Opcode::Swap1),
            ..
        }, ByteData {
            opcode: Some(Opcode::And),
            ..
        }] => peephole[1..].to_vec(),
        [ByteData {
            opcode: Some(Opcode::Swap1),
            ..
        }, ByteData {
            opcode: Some(Opcode::Or),
            ..
        }] => peephole[1..].to_vec(),
        [ByteData {
            opcode: Some(Opcode::Swap1),
            ..
        }, ByteData {
            opcode: Some(Opcode::Xor),
            ..
        }] => peephole[1..].to_vec(),

        // Operations involving an expression and itself
        [ByteData {
            opcode: Some(Opcode::Dup1),
            ..
        }, ByteData {
            opcode: Some(Opcode::And),
            ..
        }] => [].to_vec(),
        [ByteData {
            opcode: Some(Opcode::Dup1),
            ..
        }, ByteData {
            opcode: Some(Opcode::Or),
            ..
        }] => [].to_vec(),
        [ByteData {
            opcode: Some(Opcode::Dup1),
            ..
        }, ByteData {
            opcode: Some(Opcode::Xor),
            ..
        }] => [
            ByteData {
                code_index: peephole[0].code_index,
                opcode: Some(Opcode::Push1),
                pushdata: None,
            },
            ByteData {
                code_index: peephole[1].code_index,
                opcode: None,
                pushdata: Some(String::from("00")),
            }
        ].to_vec(),
        [ByteData {
            opcode: Some(Opcode::Dup1),
            ..
        }, ByteData {
            opcode: Some(Opcode::Sub),
            ..
        }] => [
            ByteData {
                code_index: peephole[0].code_index,
                opcode: Some(Opcode::Push1),
                pushdata: None,
            },
            ByteData {
                code_index: peephole[1].code_index,
                opcode: None,
                pushdata: Some(String::from("00")),
            }
        ].to_vec(),
        [ByteData {
            opcode: Some(Opcode::Dup1),
            ..
        }, ByteData {
            opcode: Some(Opcode::Eq),
            ..
        }] => [
            ByteData {
                code_index: peephole[0].code_index,
                opcode: Some(Opcode::Push1),
                pushdata: None,
            },
            ByteData {
                code_index: peephole[1].code_index,
                opcode: None,
                pushdata: Some(String::from("01")),
            }
        ].to_vec(),
        [ByteData {
            opcode: Some(Opcode::Dup1),
            ..
        }, ByteData {
            opcode: Some(Opcode::Lt),
            ..
        }] => [
            ByteData {
                code_index: peephole[0].code_index,
                opcode: Some(Opcode::Push1),
                pushdata: None,
            },
            ByteData {
                code_index: peephole[1].code_index,
                opcode: None,
                pushdata: Some(String::from("00")),
            }
        ].to_vec(),
        [ByteData {
            opcode: Some(Opcode::Dup1),
            ..
        }, ByteData {
            opcode: Some(Opcode::Slt),
            ..
        }] => [
            ByteData {
                code_index: peephole[0].code_index,
                opcode: Some(Opcode::Push1),
                pushdata: None,
            },
            ByteData {
                code_index: peephole[1].code_index,
                opcode: None,
                pushdata: Some(String::from("00")),
            }
        ].to_vec(),
        [ByteData {
            opcode: Some(Opcode::Dup1),
            ..
        }, ByteData {
            opcode: Some(Opcode::Gt),
            ..
        }] => [
            ByteData {
                code_index: peephole[0].code_index,
                opcode: Some(Opcode::Push1),
                pushdata: None,
            },
            ByteData {
                code_index: peephole[1].code_index,
                opcode: None,
                pushdata: Some(String::from("00")),
            }
        ].to_vec(),
        [ByteData {
            opcode: Some(Opcode::Dup1),
            ..
        }, ByteData {
            opcode: Some(Opcode::Sgt),
            ..
        }] => [
            ByteData {
                code_index: peephole[0].code_index,
                opcode: Some(Opcode::Push1),
                pushdata: None,
            },
            ByteData {
                code_index: peephole[1].code_index,
                opcode: None,
                pushdata: Some(String::from("00")),
            }
        ].to_vec(),
        [ByteData {
            opcode: Some(Opcode::Dup1),
            ..
        }, ByteData {
            opcode: Some(Opcode::Mod),
            ..
        }] => [
            ByteData {
                code_index: peephole[0].code_index,
                opcode: Some(Opcode::Push1),
                pushdata: None,
            },
            ByteData {
                code_index: peephole[1].code_index,
                opcode: None,
                pushdata: Some(String::from("00")),
            }
        ].to_vec(),

        // Duplicate pushes
        [ByteData {
            opcode: Some(Opcode::Push1),
            ..
        }, ByteData {
            opcode: Some(Opcode::Push1),
            ..
        }] if peephole[0].pushdata.as_ref().unwrap() == peephole[1].pushdata.as_ref().unwrap() => [
            ByteData {
                code_index: peephole[0].code_index,
                opcode: peephole[0].opcode,
                pushdata: peephole[0].pushdata,
            },
            ByteData {
                code_index: peephole[1].code_index + 1,
                opcode: Some(Opcode::Dup1),
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
                opcode: Some(Opcode::Swap1),
                pushdata: None,
            },
            ByteData {
                code_index: 5,
                opcode: Some(Opcode::Add),
                pushdata: None,
            },
        ];
        let optimized_peephole: Bytecode = vec![ByteData {
            code_index: 5,
            opcode: Some(Opcode::Add),
            pushdata: None,
        }];
        assert_eq!(optimized_peephole, check_rules(&peephole));

        // Swap1, Mul => Mul
        let peephole: Bytecode = vec![
            ByteData {
                code_index: 4,
                opcode: Some(Opcode::Swap1),
                pushdata: None,
            },
            ByteData {
                code_index: 5,
                opcode: Some(Opcode::Mul),
                pushdata: None,
            },
        ];
        let optimized_peephole: Bytecode = vec![ByteData {
            code_index: 5,
            opcode: Some(Opcode::Mul),
            pushdata: None,
        }];
        assert_eq!(optimized_peephole, check_rules(&peephole));

        // Swap1, Eq => Eq
        let peephole: Bytecode = vec![
            ByteData {
                code_index: 4,
                opcode: Some(Opcode::Swap1),
                pushdata: None,
            },
            ByteData {
                code_index: 5,
                opcode: Some(Opcode::Eq),
                pushdata: None,
            },
        ];
        let optimized_peephole: Bytecode = vec![ByteData {
            code_index: 5,
            opcode: Some(Opcode::Eq),
            pushdata: None,
        }];
        assert_eq!(optimized_peephole, check_rules(&peephole));

        // Swap1, And => And
        let peephole: Bytecode = vec![
            ByteData {
                code_index: 4,
                opcode: Some(Opcode::Swap1),
                pushdata: None,
            },
            ByteData {
                code_index: 5,
                opcode: Some(Opcode::And),
                pushdata: None,
            },
        ];
        let optimized_peephole: Bytecode = vec![ByteData {
            code_index: 5,
            opcode: Some(Opcode::And),
            pushdata: None,
        }];
        assert_eq!(optimized_peephole, check_rules(&peephole));

        // Swap1, Or => Or
        let peephole: Bytecode = vec![
            ByteData {
                code_index: 4,
                opcode: Some(Opcode::Swap1),
                pushdata: None,
            },
            ByteData {
                code_index: 5,
                opcode: Some(Opcode::Or),
                pushdata: None,
            },
        ];
        let optimized_peephole: Bytecode = vec![ByteData {
            code_index: 5,
            opcode: Some(Opcode::Or),
            pushdata: None,
        }];
        assert_eq!(optimized_peephole, check_rules(&peephole));

        // Swap1, Xor => Xor
        let peephole: Bytecode = vec![
            ByteData {
                code_index: 4,
                opcode: Some(Opcode::Swap1),
                pushdata: None,
            },
            ByteData {
                code_index: 5,
                opcode: Some(Opcode::Xor),
                pushdata: None,
            },
        ];
        let optimized_peephole: Bytecode = vec![ByteData {
            code_index: 5,
            opcode: Some(Opcode::Xor),
            pushdata: None,
        }];
        assert_eq!(optimized_peephole, check_rules(&peephole));
    }

    #[test]
    fn test_dup_expression_operations() {
        // Dup1, And => []
        let peephole = vec![ByteData {
            code_index: 4,
            opcode: Some(Opcode::Dup1),
            pushdata: None,
        }, ByteData {
            code_index: 5,
            opcode: Some(Opcode::And),
            pushdata: None,
        }];
        let optimized_peephole: Vec<ByteData> = Vec::new();
        assert_eq!(optimized_peephole, check_rules(&peephole));

        // Dup1, Or => []
        let peephole = vec![ByteData {
            code_index: 4,
            opcode: Some(Opcode::Dup1),
            pushdata: None,
        }, ByteData {
            code_index: 5,
            opcode: Some(Opcode::Or),
            pushdata: None,
        }];
        let optimized_peephole: Vec<ByteData> = Vec::new();
        assert_eq!(optimized_peephole, check_rules(&peephole));

        // Dup1, Xor => Push1, 00
        let peephole = vec![ByteData {
            code_index: 4,
            opcode: Some(Opcode::Dup1),
            pushdata: None,
        }, ByteData {
            code_index: 5,
            opcode: Some(Opcode::Xor),
            pushdata: None,
        }];
        let optimized_peephole = vec![ByteData {
            code_index: 4,
            opcode: Some(Opcode::Push1),
            pushdata: Some(String::from("00")),
        }];
        assert_eq!(optimized_peephole, check_rules(&peephole));

        // Dup1, Sub => Push1, 00
        let peephole = vec![ByteData {
            code_index: 4,
            opcode: Some(Opcode::Dup1),
            pushdata: None,
        }, ByteData {
            code_index: 5,
            opcode: Some(Opcode::Sub),
            pushdata: None,
        }];
        let optimized_peephole = vec![ByteData {
            code_index: 4,
            opcode: Some(Opcode::Push1),
            pushdata: Some(String::from("00")),
        }];
        assert_eq!(optimized_peephole, check_rules(&peephole));

        // Dup1, Eq => Push1, 01
        let peephole = vec![ByteData {
            code_index: 4,
            opcode: Some(Opcode::Dup1),
            pushdata: None,
        }, ByteData {
            code_index: 5,
            opcode: Some(Opcode::Eq),
            pushdata: None,
        }];
        let optimized_peephole = vec![ByteData {
            code_index: 4,
            opcode: Some(Opcode::Push1),
            pushdata: Some(String::from("01")),
        }];
        assert_eq!(optimized_peephole, check_rules(&peephole));

        // Dup1, Lt => Push1, 00
        let peephole = vec![ByteData {
            code_index: 4,
            opcode: Some(Opcode::Dup1),
            pushdata: None,
        }, ByteData {
            code_index: 5,
            opcode: Some(Opcode::Lt),
            pushdata: None,
        }];
        let optimized_peephole = vec![ByteData {
            code_index: 4,
            opcode: Some(Opcode::Push1),
            pushdata: Some(String::from("00")),
        }];
        assert_eq!(optimized_peephole, check_rules(&peephole));

        // Dup1, Slt => Push1, 00
        let peephole = vec![ByteData {
            code_index: 4,
            opcode: Some(Opcode::Dup1),
            pushdata: None,
        }, ByteData {
            code_index: 5,
            opcode: Some(Opcode::Slt),
            pushdata: None,
        }];
        let optimized_peephole = vec![ByteData {
            code_index: 4,
            opcode: Some(Opcode::Push1),
            pushdata: Some(String::from("00")),
        }];
        assert_eq!(optimized_peephole, check_rules(&peephole));

        // Dup1, Gt => Push1, 00
        let peephole = vec![ByteData {
            code_index: 4,
            opcode: Some(Opcode::Dup1),
            pushdata: None,
        }, ByteData {
            code_index: 5,
            opcode: Some(Opcode::Gt),
            pushdata: None,
        }];
        let optimized_peephole = vec![ByteData {
            code_index: 4,
            opcode: Some(Opcode::Push1),
            pushdata: Some(String::from("00")),
        }];
        assert_eq!(optimized_peephole, check_rules(&peephole));

        // Dup1, Sgt => Push1, 00
        let peephole = vec![ByteData {
            code_index: 4,
            opcode: Some(Opcode::Dup1),
            pushdata: None,
        }, ByteData {
            code_index: 5,
            opcode: Some(Opcode::Sgt),
            pushdata: None,
        }];
        let optimized_peephole = vec![ByteData {
            code_index: 4,
            opcode: Some(Opcode::Push1),
            pushdata: Some(String::from("00")),
        }];
        assert_eq!(optimized_peephole, check_rules(&peephole));

        // Dup1, Mod => Push1, 00
        let peephole = vec![ByteData {
            code_index: 4,
            opcode: Some(Opcode::Dup1),
            pushdata: None,
        }, ByteData {
            code_index: 5,
            opcode: Some(Opcode::Mod),
            pushdata: None,
        }];
        let optimized_peephole = vec![ByteData {
            code_index: 4,
            opcode: Some(Opcode::Push1),
            pushdata: Some(String::from("00")),
        }];
        assert_eq!(optimized_peephole, check_rules(&peephole));
    }

    #[test]
    fn test_duplicate_pushes() {
        // Push1, X, Push1, X => Push1, X, Dup1
        let peephole = vec![ByteData {
            code_index: 4,
            opcode: Some(Opcode::Push1),
            pushdata: Some(String::from("00")),
        }, ByteData {
            code_index: 5,
            opcode: Some(Opcode::Push1),
            pushdata: Some(String::from("00")),
        }];
        let optimized_peephole = vec![ByteData {
            code_index: 4,
            opcode: Some(Opcode::Push1),
            pushdata: Some(String::from("00")),
        }, ByteData {
            code_index: 6,
            opcode: Some(Opcode::Dup1),
            pushdata: None,
        }];
        assert_eq!(optimized_peephole, check_rules(&peephole));
    }
}
