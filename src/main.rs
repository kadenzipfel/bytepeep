use std::env;

mod disassembler;
mod types;
mod evm;
mod peephole;
mod rules;
mod assembler;

use crate::{disassembler::*, types::*, peephole::*, assembler::*};

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let bytecode = &args[1];
    println!("Bytecode: {}", bytecode);

    let bytes: Bytecode = disassemble(bytecode, true);
    let optimized_bytes: Bytecode = optimize(bytes);
    println!("here");
    let optimized_bytecode = assemble(&optimized_bytes);

    println!("Optimized bytecode: {}", optimized_bytecode);
}