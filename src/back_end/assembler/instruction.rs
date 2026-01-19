use crate::back_end::assembler::{Operand, Relocation, register::Register};

pub struct EncodedInstruction {
    pub bytes: Vec<u8>,
    pub relocations: Vec<Relocation>,
}

#[derive(Debug)]
pub enum Instruction {
    MOV { dst: Register, src: Operand },
    ADD { dst: Register, src: Operand },
    SUB { dst: Register, src: Operand },
    PUSH { src: Register },
    POP { dst: Register },
    CALL { target: Operand },
}

fn imm32(value: u32) -> [u8; 4] {
    value.to_le_bytes()
}

fn modrm(reg: u8, rm: u8) -> u8 {
    0b11_000_000 | (reg << 3) | rm
}

impl Instruction {
    pub fn encode(&self, current_offset: usize) -> EncodedInstruction {
        const REX_W: u8 = 0x48;

        let mut bytes = Vec::new();
        let mut relocations = Vec::new();

        match self {
            Instruction::MOV { dst, src } => match src {
                Operand::Imm(value) => {
                    bytes.push(REX_W);
                    bytes.push(0xC7); // OPCODE
                    bytes.push(modrm(0, dst.reg_code()));
                    bytes.extend(imm32(*value));
                }
                Operand::Reg(reg) => {
                    bytes.push(REX_W);
                    bytes.push(0x89); // OPCODE
                    bytes.push(modrm(reg.reg_code(), dst.reg_code()));
                }
                Operand::Symbol(name) => {
                    bytes.push(REX_W);
                    bytes.push(0xC7); // OPCODE
                    bytes.push(modrm(0, dst.reg_code()));
                    bytes.extend(&0u32.to_le_bytes());
                    relocations.push(Relocation {
                        offset: current_offset + bytes.len() - 4,
                        symbol: name.clone(),
                        addend: 0,
                    });
                }
            },
            Instruction::ADD { dst, src } => match src {
                Operand::Imm(value) => {
                    bytes.push(REX_W);
                    bytes.push(0x81); // OPCODE
                    bytes.push(modrm(0, dst.reg_code()));
                    bytes.extend(imm32(*value));
                }
                Operand::Reg(reg) => {
                    bytes.push(REX_W);
                    bytes.push(0x01); // OPCODE
                    bytes.push(modrm(reg.reg_code(), dst.reg_code()));
                }
                Operand::Symbol(_name) => {
                    todo!()
                }
            },
            Instruction::SUB { dst, src } => {
                match src {
                    Operand::Imm(value) => {
                        bytes.push(REX_W);
                        bytes.push(0x81); // OPCODE
                        bytes.push(modrm(5, dst.reg_code())); // OPCODE EXTENSION
                        bytes.extend(imm32(*value));
                    }
                    Operand::Reg(reg) => {
                        bytes.push(REX_W);
                        bytes.push(0x29); // OPCODE
                        bytes.push(modrm(reg.reg_code(), dst.reg_code()));
                    }
                    Operand::Symbol(_name) => {
                        todo!()
                    }
                }
            }
            Instruction::PUSH { src } => {
                bytes.push(REX_W);
                bytes.push(0x50 + src.reg_code()); // OPCODE
            }
            Instruction::POP { dst } => {
                bytes.push(REX_W);
                bytes.push(0x58 + dst.reg_code()); // OPCODE
            }
            Instruction::CALL { target } => match target {
                Operand::Imm(_) => todo!(),
                Operand::Reg(_) => todo!(),
                Operand::Symbol(name) => {
                    bytes.push(0xE8);
                    bytes.extend(&0u32.to_le_bytes()); // PLACEHOLDER FOR LINKER

                    relocations.push(Relocation {
                        offset: current_offset + 1,
                        symbol: name.clone(),
                        addend: -4,
                    });
                }
            },
        }

        EncodedInstruction { bytes, relocations }
    }
}
