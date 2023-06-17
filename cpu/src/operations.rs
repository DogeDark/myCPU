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
            1 => Self::Nop,
            2 => Self::Exit,
            3 => Self::Add,
            4 => Self::Subtract,
            5 => Self::Multiply,
            6 => Self::LoadA,
            7 => Self::LoadB,
            8 => Self::LoadC,
            9 => Self::StoreA,
            10 => Self::StoreB,
            11 => Self::StoreC,
            12 => Self::Jump,
            13 => Self::Jeq,
            14 => Self::Jneq,
            15 => Self::Jgt,
            16 => Self::Jlt,
            _ => panic!("unknown opcode: {}", value),
        }
    }
}
