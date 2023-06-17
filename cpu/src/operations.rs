#[derive(Debug)]
pub enum OpCode {
    // Generic
    /// No operation
    Nop,
    /// Exit with code
    Exit,

    // Math
    /// Add reg A to reg B and store in reg C
    Add,
    /// Subtract reg B from reg A and store in reg C
    Subtract,
    /// Multiply reg A with reg B and store in reg C
    Multiply,
    // Divide,

    // Load value at address into register
    /// Load value from memory address into reg A
    LoadA,
    /// Load value from memory address into reg B
    LoadB,
    /// Load value from memory address into reg C
    LoadC,

    // Save value in register to address
    /// Store value in reg A to memory address
    StoreA,
    /// Store value in reg A to memory address
    StoreB,
    /// Store value in reg A to memory address
    StoreC,

    // Control Flow
    /// Jump to address in memory
    Jump,
    /// Jump to address in memory if reg A = reg B
    Jeq,
    /// Jump to address in memory if reg A != reg B
    Jneq,
    /// Jump to address in memory if reg A > reg B
    Jgt,
    /// Jump to address in memory if reg A < reg B
    Jlt,
}

impl From<u8> for OpCode {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Nop,
            1 => Self::Exit,
            2 => Self::Add,
            3 => Self::Subtract,
            4 => Self::Multiply,
            5 => Self::LoadA,
            6 => Self::LoadB,
            7 => Self::LoadC,
            8 => Self::StoreA,
            9 => Self::StoreB,
            10 => Self::StoreC,
            11 => Self::Jump,
            12 => Self::Jeq,
            13 => Self::Jneq,
            14 => Self::Jgt,
            15 => Self::Jlt,
            _ => panic!("unknown opcode: {}", value),
        }
    }
}
