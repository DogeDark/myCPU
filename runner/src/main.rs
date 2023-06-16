use std::fs;

use clap::Parser;

#[derive(Debug, Parser)]
struct Args {
    /// Path to the binary file
    #[arg(short, long)]
    path: String,

    #[arg(short, long)]
    read_mem: Option<u32>,
}

fn main() {
    let args = Args::parse();
    let program = fs::read(args.path).expect("failed to read file");
    
    // Create cpu
    let mut cpu = cpu::Cpu::new(program);
    let code = cpu.run();

    println!("Emulator exited with code {code}");

    if let Some(addr) = args.read_mem {
        let val = cpu.memory_get(addr);
        println!("{addr} = {val}");
    }
}