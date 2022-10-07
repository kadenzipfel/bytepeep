use colored::Colorize;
use std::env;

mod assembler;
mod checks;
mod disassembler;
mod evm;
mod peephole;
mod rules;
mod types;
mod utils;

use crate::{assembler::*, checks::contains_jumps, disassembler::*, peephole::*, types::*};

fn main() {
    let args: Vec<String> = env::args().collect();

    let bytecode = &args[1];
    println!("\nBytecode: {}", bytecode);

    let bytes: Bytecode = disassemble(bytecode, true);
    let jump_warning: bool = contains_jumps(&bytes);
    let optimized_bytes: Bytecode = optimize(&bytes);
    let optimized_bytecode = assemble(&optimized_bytes);

    println!("\n\nOptimized bytecode: {}", optimized_bytecode);
    disassemble(&optimized_bytecode, true);
    println!("\n");

    if jump_warning {
        println!(
            "{}",
            format!("WARNING: Jumps are not yet supported. Output jumps are likely invalid.").yellow()
        );
    }
}
