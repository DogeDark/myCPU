use std::fs;

use clap::Parser;

const JUMP_OFFSET: u32 = 5;

#[derive(Debug, Parser)]
struct Args {
    /// Path to the assembly file
    #[arg(short, long)]
    path: String,

    /// The output file name
    #[arg(short, long)]
    output: String,
}

fn main() {
    let args = Args::parse();
    let file = fs::read_to_string(args.path).expect("failed to read file");

    let mut location = Location::Unknown;
    let mut program: Vec<Expr> = Vec::new();
    let mut lines = file.lines();

    loop {
        if let Some(line) = lines.next() {
            match line.trim() {
                "data:" => location = Location::Data,
                "main:" => location = Location::Main,
                "" => {}
                line => match location {
                    Location::Data => {
                        let split: Vec<&str> = line.split(" ").collect();
                        let var = split[0];
                        let value: u8 = split[1].parse().unwrap();
                        println!("{value}");
                        program.push(Expr::Data(var, value))
                    }
                    Location::Main => {
                        let split: Vec<&str> = line.split(" ").collect();
                        match split[0] {
                            "set" => {
                                match split[1] {
                                    "ra" => program.push(Expr::SetRegister(Register::Ra, split[2])),
                                    "rb" => program.push(Expr::SetRegister(Register::Rb, split[2])),
                                    "rc" => program.push(Expr::SetRegister(Register::Rc, split[2])),
                                    var => {
                                        // Set variable (from mem) to register
                                        let reg = match split[2] {
                                            "ra" => Register::Ra,
                                            "rb" => Register::Rb,
                                            "rc" => Register::Rc,
                                            _ => panic!("unknown register: {}", line),
                                        };

                                        program.push(Expr::SetMemory(var, reg));
                                    }
                                }
                            }
                            "add" => program.push(Expr::Add),
                            "exit" => {
                                let value: u8 = split[1].parse().unwrap();
                                program.push(Expr::Exit(value));
                            }
                            "nop" => program.push(Expr::NoOp),
                            _ => {}
                        }
                    }
                    Location::Unknown => {}
                },
            }
        } else {
            break;
        }
    }

    println!("{:?}", program);
    let bytes = build_bytes(program);
    fs::write(args.output, bytes).unwrap();
}

fn build_bytes(program: Vec<Expr>) -> Vec<u8> {
    let mut data: Vec<(&str, u8)> = Vec::new();
    let mut bytes: Vec<u8> = Vec::new();

    for expr in program {
        match expr {
            Expr::Data(var, value) => data.push((var, value)),
            Expr::SetRegister(reg, var) => {
                if let Some(i) = get_var_index(var, &data) {
                    match reg {
                        Register::Ra => bytes.push(0x07),
                        Register::Rb => bytes.push(0x08),
                        Register::Rc => bytes.push(0x09),
                    };
                    let mut addr = (i as u32 + JUMP_OFFSET).to_be_bytes().to_vec();
                    bytes.append(&mut addr);
                } else {
                    panic!("variable {var} doesn't exist");
                }
            }
            Expr::Add => bytes.push(0x03),
            Expr::SetMemory(var, reg) => {
                if let Some(i) = get_var_index(var, &data) {
                    match reg {
                        Register::Ra => bytes.push(0x0A),
                        Register::Rb => bytes.push(0x0B),
                        Register::Rc => bytes.push(0x0C),
                    };
                    let mut addr = (i as u32 + JUMP_OFFSET).to_be_bytes().to_vec();
                    bytes.append(&mut addr);
                } else {
                    panic!("variable {var} doesn't exist");
                }
            }
            Expr::Exit(val) => {
                bytes.push(0x01);
                bytes.push(val);
            }
            Expr::NoOp => bytes.push(0x00),
            Expr::Jump(addr) => {
                bytes.push(0x02);
                let mut addr = addr.to_be_bytes().to_vec();
                bytes.append(&mut addr);
            }
        };
    }

    let mut final_bytes = Vec::new();

    // Jump to the program start
    final_bytes.push(0x02);
    let mut addr = (data.len() as u32 + JUMP_OFFSET).to_be_bytes().to_vec();
    final_bytes.append(&mut addr);

    // Add the variables to front of program
    for (_, value) in data {
        println!("{value}");
        final_bytes.push(value);
    }

    // Add the rest of the program
    final_bytes.append(&mut bytes);

    final_bytes
}

fn get_var_index(var_name: &str, data: &Vec<(&str, u8)>) -> Option<usize> {
    for (i, (var, _)) in data.iter().enumerate() {
        if *var == var_name {
            return Some(i);
        }
    }

    None
}

#[derive(Debug)]
enum Location {
    Data,
    Main,
    Unknown,
}

#[derive(Debug)]
enum Expr<'a> {
    Data(&'a str, u8),
    SetRegister(Register, &'a str),
    Add,
    SetMemory(&'a str, Register),
    Exit(u8),
    NoOp,
    Jump(u32),
}

#[derive(Debug)]
enum Register {
    Ra,
    Rb,
    Rc,
}
