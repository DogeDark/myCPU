pub mod operations;
use operations::OpCode;

pub type MemorySize = u32;

pub struct Cpu {
    memory: Vec<u8>,
    ptr: MemorySize,
    ra: u8,
    rb: u8,
    rc: u8,
}

impl Cpu {
    pub fn new(memory: Vec<u8>) -> Self {
        if memory.len() > MemorySize::MAX as usize {
            panic!("memory length cannot be larger than {}", MemorySize::MAX);
        }

        Self {
            memory,
            ptr: 0,
            ra: 0,
            rb: 0,
            rc: 0,
        }
    }

    pub fn run(&mut self) -> u8 {
        loop {
            let opcode: OpCode = self.current_memory().into();
            match opcode {
                OpCode::Nop => {}
                OpCode::Exit => {
                    let code = self.memory_next_byte();
                    return code;
                }
                OpCode::Add => self.rc = self.ra + self.rb,
                OpCode::Subtract => self.rc = self.ra - self.rb,
                OpCode::Multiply => self.rc = self.ra * self.rb,
                OpCode::LoadA => {
                    let address = self.memory_next_address();
                    let value = self.get_memory(address);
                    self.ra = value;
                }
                OpCode::LoadB => {
                    let address = self.memory_next_address();
                    let value = self.get_memory(address);
                    self.rb = value;
                }
                OpCode::LoadC => {
                    let address = self.memory_next_address();
                    let value = self.get_memory(address);
                    self.rc = value;
                }
                OpCode::StoreA => {
                    let address = self.memory_next_address();
                    self.set_memory(address, self.ra);
                }
                OpCode::StoreB => {
                    let address = self.memory_next_address();
                    self.set_memory(address, self.rb);
                }
                OpCode::StoreC => {
                    let address = self.memory_next_address();
                    self.set_memory(address, self.rc);
                }
                OpCode::Jump => {
                    let address = self.memory_next_address();
                    self.set_ptr(address);
                    continue;
                }
                OpCode::Jeq => {
                    if self.ra == self.rb {
                        let address = self.memory_next_address();
                        self.set_ptr(address);
                        continue;
                    }
                }
                OpCode::Jneq => {
                    if self.ra != self.rb {
                        let address = self.memory_next_address();
                        self.set_ptr(address);
                        continue;
                    }
                }
                OpCode::Jgt => {
                    if self.ra > self.rb {
                        let address = self.memory_next_address();
                        self.set_ptr(address);
                        continue;
                    }
                }
                OpCode::Jlt => {
                    if self.ra < self.rb {
                        let address = self.memory_next_address();
                        self.set_ptr(address);
                        continue;
                    }
                }
            }

            self.increment_ptr();
        }
    }

    /// Increment the memory pointer
    pub fn increment_ptr(&mut self) {
        self.ptr += 1;
    }
    /// Set the memory pointer to a new location
    pub fn set_ptr(&mut self, address: u32) {
        self.ptr = address;
    }
    /// Get a value in memory from the current pointer
    pub fn current_memory(&self) -> u8 {
        self.memory[self.ptr as usize]
    }
    /// Get a value in memory from an address
    pub fn get_memory(&self, address: u32) -> u8 {
        self.memory[address as usize]
    }
    /// Get the entire memory
    pub fn memory(&self) -> &Vec<u8> {
        &self.memory
    }
    /// Set a value in memory from an address
    pub fn set_memory(&mut self, address: u32, value: u8) {
        self.memory[address as usize] = value;
    }
    /// Get the next byte in memory
    pub fn memory_next_byte(&self) -> u8 {
        self.memory[self.ptr as usize + 1]
    }
    /// Get the next four bytes in memory
    pub fn memory_next_address(&mut self) -> u32 {
        let bytes = [
            self.memory[self.ptr as usize + 1],
            self.memory[self.ptr as usize + 2],
            self.memory[self.ptr as usize + 3],
            self.memory[self.ptr as usize + 4],
        ];

        self.ptr += 4;
        u32::from_be_bytes(bytes)
    }
}

#[test]
fn test_cpu() {
    let mut bytes = Vec::new();

    // Jump
    bytes.push(12);
    bytes.append(&mut vec![0, 0, 0, 8]);
    // Store values
    bytes.push(10);
    bytes.push(5);
    bytes.push(0);
    // Load register A
    bytes.push(6);
    bytes.append(&mut vec![0, 0, 0, 5]);
    // Load register B
    bytes.push(7);
    bytes.append(&mut vec![0, 0, 0, 6]);
    // Add
    bytes.push(3);
    // Store C
    bytes.push(11);
    bytes.append(&mut vec![0, 0, 0, 7]);
    // Exit
    bytes.push(2);
    bytes.push(0);

    println!("{:?}", bytes);

    let mut cpu = Cpu::new(bytes);
    let code = cpu.run();
    let memory = cpu.memory();
    let data = cpu.get_memory(7);
    println!("{:?}", memory);
    println!("Code {code} | Data {data}");
}
