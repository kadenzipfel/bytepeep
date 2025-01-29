use colored::Colorize;
use std::iter;
use clap::Parser;

use crate::{
    assembler::*,
    checks::contains_jumps,
    disassembler::*,
    peephole::*,
    types::*,
    utils::{Source, compile_huff},
};

mod assembler;
mod checks;
mod disassembler;
mod evm;
mod peephole;
mod rules;
mod types;
mod utils;

#[derive(Parser)]
pub struct Cli {
    bytecode: String,

    #[clap(long, help = "Source type (raw/huff)", default_value = "raw")]
    source: Source,
}

fn main() {
    let args: Cli = Cli::parse();

    let bytecode = match args.source {
        Source::Raw => args.bytecode.clone(),
        Source::Huff => match compile_huff(&args.bytecode) {
            Ok(code) => code,
            Err(e) => {
                eprintln!("Error: {}", e.red());
                std::process::exit(1);
            }
        }
    };
    println!("Bytecode: {}", bytecode);

    let bytes: Bytecode = disassemble(&bytecode);
    let output_bytes = output(&bytes);

    let jump_warning: bool = contains_jumps(&bytes);

    let optimized_bytes: Bytecode = optimize(&bytes);
    let output_optimized_bytes = output(&optimized_bytes);
    let optimized_bytecode = assemble(&optimized_bytes);

    // Pretty print unoptimized and optimized bytecode
    let left_pad = output_bytes.lines().map(|l| l.len()).max().unwrap_or(0) + 2;
    println!("\n{:width$} {}", "Unoptimized", "Optimized", width = left_pad);
    for (output_bytes, output_optimized_bytes) in output_bytes.lines().zip(output_optimized_bytes.lines().chain(iter::repeat(""))) {
        println!("{:width$} {}", output_bytes, output_optimized_bytes, width = left_pad);
    }

    println!("\nOptimized bytecode: {}", optimized_bytecode);
    disassemble(&optimized_bytecode);

    // Warn if jumps present
    if jump_warning {
        println!(
            "{}",
            format!("WARNING: Jumps are not yet supported. Output jumps are likely invalid.").yellow()
        );
    }
}
