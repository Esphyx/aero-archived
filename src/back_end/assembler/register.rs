#[derive(Debug, Clone, Copy)]
pub enum Register {
    RAX, // ACCUMULATOR
    RCX, // COUNT
    RDX, // DATA
    RBX, // BASE
    RSP, // STACK       POINTER
    RBP, // BASE        POINTER
    RSI, // SOURCE      INDEX
    RDI, // DESTINATION INDEX
}

impl Register {
    pub fn reg_code(&self) -> u8 {
        match *self {
            Register::RAX => 0,
            Register::RCX => 1,
            Register::RDX => 2,
            Register::RBX => 3,
            Register::RSP => 4,
            Register::RBP => 5,
            Register::RSI => 6,
            Register::RDI => 7,
        }
    }
}
