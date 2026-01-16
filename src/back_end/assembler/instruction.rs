use crate::back_end::assembler::{Operand, register::Register};

#[derive(Debug)]
pub enum Instruction {
    MOV { dst: Register, src: Operand },
    ADD { dst: Register, src: Operand },
    SUB { dst: Register, src: Operand },
    PUSH { src: Register },
    POP { dst: Register },
}

fn imm32(value: u32) -> [u8; 4] {
    value.to_le_bytes()
}

fn modrm(reg: u8, rm: u8) -> u8 {
    0b11_000_000 | (reg << 3) | rm
}

impl Instruction {
    pub fn encode(&self) -> Vec<u8> {
        const REX_W: u8 = 0x48;

        let mut out = Vec::new();

        match self {
            Instruction::MOV { dst, src } => match src {
                Operand::Imm(value) => {
                    out.push(REX_W);
                    out.push(0xC7); // OPCODE
                    out.push(modrm(0, dst.reg_code()));
                    out.extend(imm32(*value));
                }
                Operand::Reg(reg) => {
                    out.push(REX_W);
                    out.push(0x89); // OPCODE
                    out.push(modrm(reg.reg_code(), dst.reg_code()));
                }
            },
            Instruction::ADD { dst, src } => match src {
                Operand::Imm(value) => {
                    out.push(REX_W);
                    out.push(0x81); // OPCODE
                    out.push(modrm(0, dst.reg_code()));
                    out.extend(imm32(*value));
                }
                Operand::Reg(reg) => {
                    out.push(REX_W);
                    out.push(0x01); // OPCODE
                    out.push(modrm(reg.reg_code(), dst.reg_code()));
                }
            },
            Instruction::SUB { dst, src } => {
                match src {
                    Operand::Imm(value) => {
                        out.push(REX_W);
                        out.push(0x81); // OPCODE
                        out.push(modrm(5, dst.reg_code())); // OPCODE EXTENSION
                        out.extend(imm32(*value));
                    }
                    Operand::Reg(reg) => {
                        out.push(REX_W);
                        out.push(0x29); // OPCODE
                        out.push(modrm(reg.reg_code(), dst.reg_code()));
                    }
                }
            }
            Instruction::PUSH { src } => {
                out.push(REX_W);
                out.push(0x50 + src.reg_code()); // OPCODE
            }
            Instruction::POP { dst } => {
                out.push(REX_W);
                out.push(0x58 + dst.reg_code()); // OPCODE
            }
        }
        out
    }
}
