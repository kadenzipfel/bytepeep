use std::env;

mod disassembler;
mod types;
mod evm;
mod peephole;
mod rules;

use crate::disassembler::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let bytecode = &args[1];
    println!("Bytecode: {}", bytecode);

    let bytes = disassemble(bytecode, true);
}