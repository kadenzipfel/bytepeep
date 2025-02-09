use crate::{evm::*, types::*, utils::*};

// Check rules against provided peephole
pub fn check_rules(peephole: &mut Bytecode) -> Bytecode {
    // Individual op checks
    for i in 0..2 {
        let mut byte: ByteData = peephole[i].clone();
            // Reducable push size
            for j in 1..32 {
                if byte.opcode == PUSH_OPS[j] {
                    let (min_len, min_string) = min_pushdata_len(&peephole[i].clone().pushdata.as_ref().unwrap());
                    if min_len == 0 {
                        byte = ByteData {
                            code_index: byte.code_index,
                            opcode: Opcode::Push0,
                            pushdata: None,
                        };
                    } else if min_len - 1 < j {
                        byte = ByteData {
                            code_index: byte.code_index,
                            opcode: PUSH_OPS[min_len],
                            pushdata: Some(min_string),
                        };
                    }
                }
            }

        peephole[i] = byte;
    }

    // Peephole (2 op) checks
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
        }] | [ByteData {
            opcode: Opcode::Push2,
            ..
        }, ByteData {
            opcode: Opcode::Push2,
            ..
        }] | [ByteData {
            opcode: Opcode::Push3,
            ..
        }, ByteData {
            opcode: Opcode::Push3,
            ..
        }] | [ByteData {
            opcode: Opcode::Push4,
            ..
        }, ByteData {
            opcode: Opcode::Push4,
            ..
        }] | [ByteData {
            opcode: Opcode::Push5,
            ..
        }, ByteData {
            opcode: Opcode::Push5,
            ..
        }] | [ByteData {
            opcode: Opcode::Push6,
            ..
        }, ByteData {
            opcode: Opcode::Push6,
            ..
        }] | [ByteData {
            opcode: Opcode::Push7,
            ..
        }, ByteData {
            opcode: Opcode::Push7,
            ..
        }] | [ByteData {
            opcode: Opcode::Push8,
            ..
        }, ByteData {
            opcode: Opcode::Push8,
            ..
        }] | [ByteData {
            opcode: Opcode::Push9,
            ..
        }, ByteData {
            opcode: Opcode::Push9,
            ..
        }] | [ByteData {
            opcode: Opcode::Push10,
            ..
        }, ByteData {
            opcode: Opcode::Push10,
            ..
        }] | [ByteData {
            opcode: Opcode::Push11,
            ..
        }, ByteData {
            opcode: Opcode::Push11,
            ..
        }] | [ByteData {
            opcode: Opcode::Push12,
            ..
        }, ByteData {
            opcode: Opcode::Push12,
            ..
        }] | [ByteData {
            opcode: Opcode::Push13,
            ..
        }, ByteData {
            opcode: Opcode::Push13,
            ..
        }] | [ByteData {
            opcode: Opcode::Push14,
            ..
        }, ByteData {
            opcode: Opcode::Push14,
            ..
        }] | [ByteData {
            opcode: Opcode::Push15,
            ..
        }, ByteData {
            opcode: Opcode::Push15,
            ..
        }] | [ByteData {
            opcode: Opcode::Push16,
            ..
        }, ByteData {
            opcode: Opcode::Push16,
            ..
        }] | [ByteData {
            opcode: Opcode::Push17,
            ..
        }, ByteData {
            opcode: Opcode::Push17,
            ..
        }] | [ByteData {
            opcode: Opcode::Push18,
            ..
        }, ByteData {
            opcode: Opcode::Push18,
            ..
        }] | [ByteData {
            opcode: Opcode::Push19,
            ..
        }, ByteData {
            opcode: Opcode::Push19,
            ..
        }] | [ByteData {
            opcode: Opcode::Push20,
            ..
        }, ByteData {
            opcode: Opcode::Push20,
            ..
        }] | [ByteData {
            opcode: Opcode::Push21,
            ..
        }, ByteData {
            opcode: Opcode::Push21,
            ..
        }] | [ByteData {
            opcode: Opcode::Push22,
            ..
        }, ByteData {
            opcode: Opcode::Push22,
            ..
        }] | [ByteData {
            opcode: Opcode::Push23,
            ..
        }, ByteData {
            opcode: Opcode::Push23,
            ..
        }] | [ByteData {
            opcode: Opcode::Push24,
            ..
        }, ByteData {
            opcode: Opcode::Push24,
            ..
        }] | [ByteData {
            opcode: Opcode::Push25,
            ..
        }, ByteData {
            opcode: Opcode::Push25,
            ..
        }] | [ByteData {
            opcode: Opcode::Push26,
            ..
        }, ByteData {
            opcode: Opcode::Push26,
            ..
        }] | [ByteData {
            opcode: Opcode::Push27,
            ..
        }, ByteData {
            opcode: Opcode::Push27,
            ..
        }] | [ByteData {
            opcode: Opcode::Push28,
            ..
        }, ByteData {
            opcode: Opcode::Push28,
            ..
        }] | [ByteData {
            opcode: Opcode::Push29,
            ..
        }, ByteData {
            opcode: Opcode::Push29,
            ..
        }] | [ByteData {
            opcode: Opcode::Push30,
            ..
        }, ByteData {
            opcode: Opcode::Push30,
            ..
        }] | [ByteData {
            opcode: Opcode::Push31,
            ..
        }, ByteData {
            opcode: Opcode::Push31,
            ..
        }] | [ByteData {
            opcode: Opcode::Push32,
            ..
        }, ByteData {
            opcode: Opcode::Push32,
            ..
        }] if peephole[0].pushdata.as_ref().unwrap() == peephole[1].pushdata.as_ref().unwrap() => [
            ByteData {
                code_index: peephole[0].code_index,
                opcode: peephole[0].opcode,
                pushdata: Some(peephole[0].pushdata.as_ref().unwrap().to_string()),
            },
            ByteData {
                code_index: peephole[1].code_index,
                opcode: Opcode::Dup1,
                pushdata: None,
            },
        ].to_vec(),

        // Logical instruction combinations
        [ByteData {
            opcode: Opcode::Not,
            ..
        }, ByteData {
            opcode: Opcode::Not,
            ..
        }] => [].to_vec(),

        // Double negation resulting in boolean
        [ByteData {
            opcode: Opcode::Xor,
            ..
        }, ByteData {
            opcode: Opcode::Iszero,
            ..
        }] => [
            ByteData {
                code_index: peephole[0].code_index,
                opcode: Opcode::Eq,
                pushdata: None,
            }
        ].to_vec(),
        [ByteData {
            opcode: Opcode::Sub,
            ..
        }, ByteData {
            opcode: Opcode::Iszero,
            ..
        }] => [
            ByteData {
                code_index: peephole[0].code_index,
                opcode: Opcode::Eq,
                pushdata: None,
            }
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
        let mut peephole: Bytecode = vec![
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
        assert_eq!(optimized_peephole, check_rules(&mut peephole));

        // Swap1, Mul => Mul
        let mut peephole: Bytecode = vec![
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
        assert_eq!(optimized_peephole, check_rules(&mut peephole));

        // Swap1, Eq => Eq
        let mut peephole: Bytecode = vec![
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
        assert_eq!(optimized_peephole, check_rules(&mut peephole));

        // Swap1, And => And
        let mut peephole: Bytecode = vec![
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
        assert_eq!(optimized_peephole, check_rules(&mut peephole));

        // Swap1, Or => Or
        let mut peephole: Bytecode = vec![
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
        assert_eq!(optimized_peephole, check_rules(&mut peephole));

        // Swap1, Xor => Xor
        let mut peephole: Bytecode = vec![
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
        assert_eq!(optimized_peephole, check_rules(&mut peephole));
    }

    #[test]
    fn test_dup_expression_operations() {
        // Dup1, And => []
        let mut peephole = vec![ByteData {
            code_index: 4,
            opcode: Opcode::Dup1,
            pushdata: None,
        }, ByteData {
            code_index: 5,
            opcode: Opcode::And,
            pushdata: None,
        }];
        let optimized_peephole: Vec<ByteData> = Vec::new();
        assert_eq!(optimized_peephole, check_rules(&mut peephole));

        // Dup1, Or => []
        let mut peephole = vec![ByteData {
            code_index: 4,
            opcode: Opcode::Dup1,
            pushdata: None,
        }, ByteData {
            code_index: 5,
            opcode: Opcode::Or,
            pushdata: None,
        }];
        let optimized_peephole: Vec<ByteData> = Vec::new();
        assert_eq!(optimized_peephole, check_rules(&mut peephole));

        // Dup1, Xor => Push1, 00
        let mut peephole = vec![ByteData {
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
        assert_eq!(optimized_peephole, check_rules(&mut peephole));

        // Dup1, Sub => Push1, 00
        let mut peephole = vec![ByteData {
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
        assert_eq!(optimized_peephole, check_rules(&mut peephole));

        // Dup1, Eq => Push1, 01
        let mut peephole = vec![ByteData {
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
        assert_eq!(optimized_peephole, check_rules(&mut peephole));

        // Dup1, Lt => Push1, 00
        let mut peephole = vec![ByteData {
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
        assert_eq!(optimized_peephole, check_rules(&mut peephole));

        // Dup1, Slt => Push1, 00
        let mut peephole = vec![ByteData {
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
        assert_eq!(optimized_peephole, check_rules(&mut peephole));

        // Dup1, Gt => Push1, 00
        let mut peephole = vec![ByteData {
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
        assert_eq!(optimized_peephole, check_rules(&mut peephole));

        // Dup1, Sgt => Push1, 00
        let mut peephole = vec![ByteData {
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
        assert_eq!(optimized_peephole, check_rules(&mut peephole));

        // Dup1, Mod => Push1, 00
        let mut peephole = vec![ByteData {
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
        assert_eq!(optimized_peephole, check_rules(&mut peephole));
    }

    #[test]
    fn test_duplicate_pushes() {
        // Test Push1 through Push32 (skip Push0 since it has no pushdata)
        for i in 1..32 {
            // PushN, X, PushN, X => PushN, X, Dup1
            let mut peephole = vec![ByteData {
                code_index: 4,
                opcode: PUSH_OPS[i],
                pushdata: Some(std::iter::repeat("10").take(i).collect::<String>()),
            }, ByteData {
                code_index: 5,
                opcode: PUSH_OPS[i],
                pushdata: Some(std::iter::repeat("10").take(i).collect::<String>()),
            }];
            let optimized_peephole = vec![ByteData {
                code_index: 4,
                opcode: PUSH_OPS[i],
                pushdata: Some(std::iter::repeat("10").take(i).collect::<String>()),
            }, ByteData {
                code_index: 5,
                opcode: Opcode::Dup1,
                pushdata: None,
            }];
            assert_eq!(optimized_peephole, check_rules(&mut peephole));
        }

        // Push0 is more efficient than dup1 so don't optimize
        let mut peephole = vec![ByteData {
            code_index: 4,
            opcode: Opcode::Push0,
            pushdata: None,
        }, ByteData {
            code_index: 5,
            opcode: Opcode::Push0,
            pushdata: None,
        }];
        let optimized_peephole = vec![ByteData {
            code_index: 4,
            opcode: Opcode::Push0,
            pushdata: None,
        }, ByteData {
            code_index: 5,
            opcode: Opcode::Push0,
            pushdata: None,
        }];
        assert_eq!(optimized_peephole, check_rules(&mut peephole));
    }

    #[test]
    fn test_reduced_push_size() {
        let mut peephole = vec![ByteData {
            code_index: 4,
            opcode: Opcode::Push2,
            pushdata: Some(String::from("0080")),
        }, ByteData {
            code_index: 5,
            opcode: Opcode::Push18,
            pushdata: Some(String::from("000000000000000000002030000000004040")),
        }];
        let optimized_peephole = vec![ByteData {
            code_index: 4,
            opcode: Opcode::Push1,
            pushdata: Some(String::from("80")),
        }, ByteData {
            code_index: 5,
            opcode: Opcode::Push8,
            pushdata: Some(String::from("2030000000004040")),
        }];
        assert_eq!(optimized_peephole, check_rules(&mut peephole));
    }

    #[test]
    fn test_logical_instruction_combinations() {
        let mut peephole = vec![ByteData {
            code_index: 4,
            opcode: Opcode::Not,
            pushdata: None,
        }, ByteData {
            code_index: 5,
            opcode: Opcode::Not,
            pushdata: None,
        }];
        let optimized_peephole: Bytecode = [].to_vec();
        assert_eq!(optimized_peephole, check_rules(&mut peephole));
    }

    #[test]
    fn test_double_negation() {
        
        // Xor, Iszero => Eq
        let mut peephole = vec![ByteData {
            code_index: 4,
            opcode: Opcode::Xor,
            pushdata: None,
        }, ByteData {
            code_index: 5,
            opcode: Opcode::Iszero,
            pushdata: None,
        }];
        let optimized_peephole = [
            ByteData {
                code_index: 4,
                opcode: Opcode::Eq,
                pushdata: None,
            }
        ].to_vec();
        assert_eq!(optimized_peephole, check_rules(&mut peephole));

        // Sub, Iszero => Eq
        let mut peephole = vec![ByteData {
            code_index: 4,
            opcode: Opcode::Sub,
            pushdata: None,
        }, ByteData {
            code_index: 5,
            opcode: Opcode::Iszero,
            pushdata: None,
        }];
        let optimized_peephole = [
            ByteData {
                code_index: 4,
                opcode: Opcode::Eq,
                pushdata: None,
            }
        ].to_vec();
        assert_eq!(optimized_peephole, check_rules(&mut peephole));
    }
}
