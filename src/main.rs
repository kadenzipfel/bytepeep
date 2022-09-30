use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let bytecode = &args[1];

    println!("Bytecode: {}", bytecode);
}