use std::env;
use colored::Colorize;

mod assembler;
mod disassembler;
mod evm;
mod peephole;
mod rules;
mod types;
mod checks;

use crate::{assembler::*, disassembler::*, peephole::*, types::*, checks::contains_jumps};

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
        println!("{}", format!("WARNING: Jumps are not supported. Output jumps are likely invalid.").yellow());
    }
}
