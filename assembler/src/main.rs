// op [dest] [src]
/*
Memory Structure

jump main
data
data
data
..
main
instruction
instruction
instruction other
other
*/
use clap::Parser;
use regex::Regex;
use std::{collections::HashMap, fs};
use tokens::{calc_byte_len, Token};

mod tokens;

const JUMP_OFFSET: u32 = 5;

#[derive(Debug, Parser)]
struct Args {
    /// Path to the assembly file
    #[arg(short, long)]
    path: String,
    /// Output file name/path
    #[arg(short, long)]
    output: String,
}

fn main() {
    let args = Args::parse();
    let content = fs::read_to_string(args.path).expect("failed to read file");
    let lines: Vec<&str> = content.lines().collect();

    let (section_labels, section_tokens) = get_sections(lines);
    println!("{:?}", section_labels);
    println!("{:?}", section_tokens);

    let bytes = build_bytes(section_labels, section_tokens);
    println!("{:?}", bytes);
    //fs::write(args.output, bytes).expect("failed to write bytes to output file");
}

/// Returns section labels and their tokens
fn get_sections(lines: Vec<&str>) -> (Vec<&str>, Vec<Vec<Token>>) {
    let regex = Regex::new("[a-zA-Z]*:").unwrap();

    let mut labels = Vec::new();
    let mut sections = Vec::new();
    let mut tokens = Vec::new();

    for line in lines {
        let line = line.trim();
        if regex.is_match(line) {
            labels.push(line.trim_end_matches(":"));
            if labels.len() > 1 {
                sections.push(tokens);
                tokens = Vec::new();
            }
        } else {
            tokens.push(Token::from(line));
        }
    }
    sections.push(tokens);

    (labels, sections)
}

/// Build a list of executable bytes from a list of labels and tokens
fn build_bytes(section_labels: Vec<&str>, section_tokens: Vec<Vec<Token>>) -> Vec<u8> {
    let mut label_addresses: HashMap<&str, u32> = HashMap::new();
    let mut data_bytes: Vec<u8> = Vec::new();
    let mut code_bytes: Vec<u8> = Vec::new();

    let mut section_sizes = Vec::new();

    // Calculate section lengths
    for (index, _) in section_labels.iter().enumerate() {
        let size = calc_byte_len(&section_tokens[index]);
        section_sizes.push(size);
    }

    for (index, label) in section_labels.iter().enumerate() {
        if index == 0 {
            continue;
        }

        let size = section_sizes[index - 1];
        label_addresses.insert(label, size + 1);
    }

    // Convert data into bytes
    for (index, token) in section_tokens[0].iter().enumerate() {
        match token {
            Token::U8Data(label, value) => {
                label_addresses.insert(label, index as u32 + JUMP_OFFSET);
                data_bytes.push(*value);
            }
            _ => {}
        };
    }

    for (index, label) in section_labels.iter().enumerate() {
        match *label {
            "data" => {}
            _ => {
                for token in &section_tokens[index] {
                    match token {
                        Token::Nop => code_bytes.push(0),
                        Token::Exit(value) => code_bytes.append(&mut vec![1, *value]),
                        Token::U8Data(_, _) => panic!("data types should be in the data section"),
                        Token::Add => code_bytes.push(2),
                        Token::Subtract => code_bytes.push(3),
                        Token::Multiply => code_bytes.push(4),
                        Token::LoadA(label) => {
                            let addr = label_addresses
                                .get(label.as_str())
                                .expect("label {label} doesn't exist");
                            code_bytes.push(5);
                            code_bytes.append(&mut addr.to_be_bytes().to_vec());
                        }
                        Token::LoadB(label) => {
                            let addr = label_addresses
                                .get(label.as_str())
                                .expect("label {label} doesn't exist");
                            code_bytes.push(6);
                            code_bytes.append(&mut addr.to_be_bytes().to_vec());
                        }
                        Token::LoadC(label) => {
                            let addr = label_addresses
                                .get(label.as_str())
                                .expect("label {label} doesn't exist");
                            code_bytes.push(7);
                            code_bytes.append(&mut addr.to_be_bytes().to_vec());
                        }
                        Token::StoreA(label) => {
                            let addr = label_addresses
                                .get(label.as_str())
                                .expect("label {label} doesn't exist");
                            code_bytes.push(8);
                            code_bytes.append(&mut addr.to_be_bytes().to_vec());
                        }
                        Token::StoreB(label) => {
                            let addr = label_addresses
                                .get(label.as_str())
                                .expect("label {label} doesn't exist");
                            code_bytes.push(9);
                            code_bytes.append(&mut addr.to_be_bytes().to_vec());
                        }
                        Token::StoreC(label) => {
                            let addr = label_addresses
                                .get(label.as_str())
                                .expect("label {label} doesn't exist");
                            code_bytes.push(10);
                            code_bytes.append(&mut addr.to_be_bytes().to_vec());
                        }
                        Token::Jump(label) => {
                            let addr = label_addresses
                                .get(label.as_str())
                                .expect("label {label} doesn't exist");
                            code_bytes.push(11);
                            code_bytes.append(&mut addr.to_be_bytes().to_vec());
                        }
                        Token::Jeq(label) => {
                            let addr = label_addresses
                                .get(label.as_str())
                                .expect("label {label} doesn't exist");
                            code_bytes.push(12);
                            code_bytes.append(&mut addr.to_be_bytes().to_vec());
                        }
                        Token::Jneq(label) => {
                            let addr = label_addresses
                                .get(label.as_str())
                                .expect("label {label} doesn't exist");
                            code_bytes.push(13);
                            code_bytes.append(&mut addr.to_be_bytes().to_vec());
                        }
                        Token::Jgt(label) => {
                            let addr = label_addresses
                                .get(label.as_str())
                                .expect("label {label} doesn't exist");
                            code_bytes.push(14);
                            code_bytes.append(&mut addr.to_be_bytes().to_vec());
                        }
                        Token::Jlt(label) => {
                            let addr = label_addresses
                                .get(label.as_str())
                                .expect("label {label} doesn't exist");
                            code_bytes.push(15);
                            code_bytes.append(&mut addr.to_be_bytes().to_vec());
                        }
                        Token::Ignore => {}
                    }
                }
            }
        };
    }

    let mut bytes = Vec::new();

    // Put jump instruction before data (5 bytes)
    bytes.push(11);
    bytes.append(&mut (section_sizes[0] + JUMP_OFFSET).to_be_bytes().to_vec());

    // Put data section (1 * token bytes)
    bytes.append(&mut data_bytes);

    // Put code bytes
    bytes.append(&mut code_bytes);

    bytes
}
