#[derive(Debug)]
pub enum Token {
    Nop,
    Exit(u8),
    U8Data(String, u8),
    Add,
    Subtract,
    Multiply,
    LoadA(String),
    LoadB(String),
    LoadC(String),
    StoreA(String),
    StoreB(String),
    StoreC(String),
    Jump(String),
    Jeq(String),
    Jneq(String),
    Jgt(String),
    Jlt(String),
    Ignore,
}

impl From<&str> for Token {
    fn from(value: &str) -> Self {
        let split: Vec<&str> = value.split(" ").collect();
        match split[0].trim() {
            "nop" => Self::Nop,
            "exit" => {
                let value: u8 = split[1]
                    .trim()
                    .parse()
                    .expect("data value must fit in a u8");
                Self::Exit(value)
            }
            "add" => Self::Add,
            "subtract" => Self::Subtract,
            "multiply" => Self::Multiply,
            "loada" => {
                let label = split[1].trim().to_string();
                Self::LoadA(label)
            }
            "loadb" => {
                let label = split[1].trim().to_string();
                Self::LoadB(label)
            }
            "loadc" => {
                let label = split[1].trim().to_string();
                Self::LoadC(label)
            }
            "storea" => {
                let label = split[1].trim().to_string();
                Self::StoreA(label)
            }
            "storeb" => {
                let label = split[1].trim().to_string();
                Self::StoreB(label)
            }
            "storec" => {
                let label = split[1].trim().to_string();
                Self::StoreC(label)
            }
            "jump" => {
                let label = split[1].trim().to_string();
                Self::Jump(label)
            }
            "jeq" => {
                let label = split[1].trim().to_string();
                Self::Jeq(label)
            }
            "jneq" => {
                let label = split[1].trim().to_string();
                Self::Jneq(label)
            }
            "jgt" => {
                let label = split[1].trim().to_string();
                Self::Jgt(label)
            }
            "jlt" => {
                let label = split[1].trim().to_string();
                Self::Jlt(label)
            }
            "u8" => {
                let name = split[1].trim();
                let value: u8 = split[2]
                    .trim()
                    .parse()
                    .expect("data value must fit in a u8");
                Self::U8Data(name.to_string(), value)
            }
            "//" => Self::Ignore,
            "" => Self::Ignore,
            _ => panic!("unknown token: {}", split[0].trim()),
        }
    }
}

pub fn calc_byte_len(tokens: &Vec<Token>) -> u32 {
    let mut size = 0;

    for token in tokens {
        match token {
            Token::Nop => size += 1,
            Token::Exit(_) => size += 2,
            Token::U8Data(_, _) => size += 1,
            Token::Add => size += 1,
            Token::Subtract => size += 1,
            Token::Multiply => size += 1,
            Token::LoadA(_) => size += 5,
            Token::LoadB(_) => size += 5,
            Token::LoadC(_) => size += 5,
            Token::StoreA(_) => size += 5,
            Token::StoreB(_) => size += 5,
            Token::StoreC(_) => size += 5,
            Token::Jump(_) => size += 5,
            Token::Jeq(_) => size += 5,
            Token::Jneq(_) => size += 5,
            Token::Jgt(_) => size += 5,
            Token::Jlt(_) => size += 5,
            Token::Ignore => {},
        }
    }

    size
}