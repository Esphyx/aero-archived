use crate::back_end::assembler::{instruction::Instruction, operand::Operand};

pub mod instruction;
pub mod operand;
pub mod register;

pub fn assemble(instructions: Vec<Instruction>) -> Vec<u8> {
    let mut out = Vec::new();

    for instruction in &instructions {
        out.extend(instruction.encode());
    }

    out
}
