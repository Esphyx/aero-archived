use instruction::Instruction;

pub mod instruction;
pub mod isa;
pub mod operand;
pub mod register;

pub struct AssemblyProgram {
    pub bytes: Vec<u8>,
}

impl AssemblyProgram {
    pub fn new() -> Self {
        Self { bytes: Vec::new() }
    }

    pub fn add_instruction(&mut self, instr: Instruction) {
        self.bytes.extend(instr.encode());
    }
}
