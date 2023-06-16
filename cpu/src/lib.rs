type MemorySize = u32;

pub struct Cpu {
    memory: Vec<u8>,
    memory_ptr: MemorySize,
    ra: u8,
    rb: u8,
    rc: u8,
}

impl Cpu {
    pub fn new(program: Vec<u8>) -> Self {
        Self {
            memory: program,
            memory_ptr: 0,
            ra: 0,
            rb: 0,
            rc: 0,
        }
    }

    pub fn run(&mut self) -> u8 {
        loop {
            let op = OpCode::from(self.memory_current());
            println!("{:?}", op);

            match op {
                OpCode::NoOp => {}
                OpCode::Exit => {
                    let value = self.memory_next();
                    return value;
                }
                OpCode::Jump => {
                    let address = self.memory_next_u32();
                    println!("JUMPING TO {:?}", address);
                    self.memory_set_ptr(address);
                    // Avoid calling memory_next again
                    continue;
                }
                OpCode::Add => {
                    println!("Adding: {} + {}", self.ra, self.rb);
                    self.rc = self.ra + self.rb;
                }
                OpCode::Subtract => {
                    self.rc = self.ra - self.rb;
                }
                OpCode::Multiply => {
                    self.rc = self.ra * self.rb;
                }
                OpCode::SetRegA => {
                    let address = self.memory_next_u32();
                    let value = self.memory_get(address);
                    println!("Setting reg A: {} = {}", address, value);
                    self.ra = value;
                }
                OpCode::SetRegB => {
                    let address = self.memory_next_u32();
                    let value = self.memory_get(address);
                    println!("Setting reg B: {} = {}", address, value);
                    self.rb = value;
                }
                OpCode::SetRegC => {
                    let address = self.memory_next_u32();
                    let value = self.memory_get(address);
                    self.rc = value;
                }
                OpCode::SetMemA => {
                    let address = self.memory_next_u32();
                    let value = self.ra;
                    self.memory_set(address, value);
                }
                OpCode::SetMemB => {
                    let address = self.memory_next_u32();
                    let value = self.rb;
                    self.memory_set(address, value);
                }
                OpCode::SetMemC => {
                    let address = self.memory_next_u32();
                    let value = self.rc;
                    println!("Setting mem from reg C: {} = {}", address, value);
                    self.memory_set(address, value);
                }
            }

            // Done
            self.memory_next();
        }
    }

    pub fn memory_get(&mut self, address: MemorySize) -> u8 {
        self.memory[address as usize]
    }
    pub fn memory_set(&mut self, address: MemorySize, value: u8) {
        self.memory[address as usize] = value;
    }
    pub fn memory_current(&self) -> u8 {
        self.memory[self.memory_ptr as usize]
    }
    pub fn memory_next(&mut self) -> u8 {
        if self.memory_ptr == <MemorySize>::MAX {
            self.memory_ptr = 0;
        } else {
            self.memory_ptr += 1;
        }
        self.memory[self.memory_ptr as usize]
    }
    pub fn memory_next_u32(&mut self) -> u32 {
        let s1 = self.memory_next();
        let s2 = self.memory_next();
        let s3 = self.memory_next();
        let s4 = self.memory_next();
        let slice = [s1, s2, s3, s4];
        u32::from_be_bytes(slice)
    }
    pub fn memory_set_ptr(&mut self, address: MemorySize) {
        self.memory_ptr = address;
    }
}

#[derive(Debug)]
pub enum OpCode {
    /// No operation
    NoOp,
    /// Exit with code
    Exit,
    /// Jump to a location in memory
    Jump,
    /// Add the two values (ra + rb) and store it in rc
    Add,
    /// Subtract the two values (ra - rb) and store it in rc
    Subtract,
    /// Multiply the two values (ra * rb) and store it in rc
    Multiply,
    // Divide the two values (ra / rb) and store it in rc
    //Divide,
    /// Set registers from memory address
    SetRegA,
    SetRegB,
    SetRegC,
    /// Set memory from registers
    SetMemA,
    SetMemB,
    SetMemC,
}

impl From<u8> for OpCode {
    fn from(value: u8) -> Self {
        match value {
            0x00 => Self::NoOp,
            0x01 => Self::Exit,
            0x02 => Self::Jump,
            // Operations
            0x03 => Self::Add,
            0x04 => Self::Subtract,
            0x05 => Self::Multiply,
            //0x6 => Self::Divide,
            // Set registers
            0x07 => Self::SetRegA,
            0x08 => Self::SetRegB,
            0x09 => Self::SetRegC,
            // Set memory from registers
            0x0A => Self::SetMemA,
            0x0B => Self::SetMemB,
            0x0C => Self::SetMemC,
            // Unknown
            _ => panic!("unknown operation"),
        }
    }
}

impl Into<u8> for OpCode {
    fn into(self) -> u8 {
        match self {
            Self::NoOp => 0x00,
            Self::Exit => 0x01,
            Self::Jump => 0x02,
            Self::Add => 0x03,
            Self::Subtract => 0x04,
            Self::Multiply => 0x05,
            Self::SetRegA => 0x07,
            Self::SetRegB => 0x08,
            Self::SetRegC => 0x09,
            Self::SetMemA => 0x0A,
            Self::SetMemB => 0x0B,
            Self::SetMemC => 0x0C,
        }
    }
}

#[test]
fn test_cpu() {
    let mut program = vec![0; MemorySize::MAX as usize];

    // Set values
    program[253] = 10;
    program[254] = 5;

    // Set reg A
    program[0] = 0x07;
    // From mem address
    program[1] = 0b0000_0000;
    program[2] = 0b0000_0000;
    program[3] = 0b0000_0000;
    program[4] = 0b1111_1101;

    // Set reg B
    program[5] = 0x08;
    // From mem address
    program[6] = 0b0000_0000;
    program[7] = 0b0000_0000;
    program[8] = 0b0000_0000;
    program[9] = 0b1111_1110;

    // Add reg A & B setting reg C
    program[10] = 0x03;

    // Set mem from reg C
    program[11] = 0x0C;
    // at address
    program[12] = 0b0000_0000;
    program[13] = 0b0000_0000;
    program[14] = 0b0000_0000;
    program[15] = 0b1111_1111;

    // exit program
    program[16] = 0x01;
    program[17] = 0;

    let mut cpu = Cpu::new(program);
    let res = cpu.run();
    let data = cpu.memory_get(255);

    println!("");
    println!("Code: {res} | 255: {data}");
    println!("");
}
