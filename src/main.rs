use colored::Colorize;
use std::{env, iter};

use crate::{assembler::*, checks::contains_jumps, disassembler::*, peephole::*, types::*};

mod assembler;
mod checks;
mod disassembler;
mod evm;
mod peephole;
mod rules;
mod types;
mod utils;

fn main() {
    let args: Vec<String> = env::args().collect();

    let bytecode = &args[1];
    println!("Bytecode: {}", bytecode);

    let bytes: Bytecode = disassemble(bytecode);
    let output_bytes = output(&bytes);

    let jump_warning: bool = contains_jumps(&bytes);

    let optimized_bytes: Bytecode = optimize(&bytes);
    let output_optimized_bytes = output(&optimized_bytes);
    let optimized_bytecode = assemble(&optimized_bytes);

    let left_pad = output_bytes.lines().map(|l| l.len()).max().unwrap_or(0) + 2;
    println!("\n{:width$} {}", "Unoptimized", "Optimized", width = left_pad);
    for (output_bytes, output_optimized_bytes) in output_bytes.lines().zip(output_optimized_bytes.lines().chain(iter::repeat(""))) {
        println!("{:width$} {}", output_bytes, output_optimized_bytes, width = left_pad);
    }

    println!("\nOptimized bytecode: {}", optimized_bytecode);
    disassemble(&optimized_bytecode);

    if jump_warning {
        println!(
            "{}",
            format!("WARNING: Jumps are not yet supported. Output jumps are likely invalid.").yellow()
        );
    }
}
