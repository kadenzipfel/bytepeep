use std::env;

mod assembler;
mod disassembler;
mod evm;
mod peephole;
mod rules;
mod types;

use crate::{assembler::*, disassembler::*, peephole::*, types::*};

fn main() {
    let args: Vec<String> = env::args().collect();

    let bytecode = &args[1];
    println!("\nBytecode: {}", bytecode);

    let bytes: Bytecode = disassemble(bytecode, true);
    let optimized_bytes: Bytecode = optimize(bytes);
    let optimized_bytecode = assemble(&optimized_bytes);

    println!("\n\nOptimized bytecode: {}", optimized_bytecode);
    disassemble(&optimized_bytecode, true);
    println!("\n");
}
