

#[derive(Clone, Copy)]
pub enum Reg {
    // 64 bit
    RAX, // ACCUMULATOR
    RCX, // COUNT
    RDX, // DATA
    RBX, // BASE
    RSP, // STACK       POINTER
    RBP, // BASE        POINTER
    RSI, // SOURCE      INDEX
    RDI, // DESTINATION INDEX

    EAX,
    ECX,
    EDX,
    EBX,
    ESP,
    EBP,
    ESI,
    EDI,
}

impl Reg {
    pub fn reg_code(&self) -> u8 {
        match *self {
            Self::RAX => 0,
            Self::RCX => 1,
            Self::RDX => 2,
            Self::RBX => 3,
            Self::RSP => 4,
            Self::RBP => 5,
            Self::RSI => 6,
            Self::RDI => 7,

            Self::EAX => 0,
            Self::ECX => 1,
            Self::EDX => 2,
            Self::EBX => 3,
            Self::ESP => 4,
            Self::EBP => 5,
            Self::ESI => 6,
            Self::EDI => 7,
        }
    }
}

pub enum Operand {
    Imm(Immediate),
    Reg(Reg),
}

pub enum Immediate {
    Byte(u8),
    Word(u16),
    DoubleWord(u32),
    QuadWord(u64),
}

pub enum Instruction {
    MOV { dst: Reg, src: Operand },
    ADD { dst: Reg, src: Operand },
    SUB { dst: Reg, src: Operand },
    PUSH { src: Reg },
    POP { dst: Reg },
    CALL { target: Operand },
    SYSCALL,
}

fn rex_w() -> u8 {
    0x48
}

fn modrm(reg: u8, rm: u8) -> u8 {
    0b11_000_000 | (reg << 3) | rm
}

impl Instruction {
    pub fn encode(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        bytes.push(rex_w());

        match self {
            Instruction::MOV { dst, src } => {
                match src {
                    Operand::Imm(imm) => match imm {
                        Immediate::Byte(value) => {
                            bytes.push(0xC6); // OPCODE
                            bytes.push(modrm(0, dst.reg_code()));
                            bytes.push(*value);
                        }
                        Immediate::Word(value) => unimplemented!(),
                        Immediate::DoubleWord(value) => unimplemented!(),
                        Immediate::QuadWord(value) => unimplemented!(),
                    },
                    Operand::Reg(reg) => {
                        bytes.push(0x89); // OPCODE
                        bytes.push(modrm(reg.reg_code(), dst.reg_code()));
                    }
                }
            }
            Instruction::ADD { dst, src } => match src {
                Operand::Imm(imm) => unimplemented!(),
                Operand::Reg(reg) => {
                    bytes.push(0x01); // OPCODE
                    bytes.push(modrm(reg.reg_code(), dst.reg_code()));
                }
            },
            Instruction::SUB { dst, src } => {
                match src {
                    Operand::Imm(imm) => unimplemented!(),
                    Operand::Reg(reg) => {
                        bytes.push(0x29); // OPCODE
                        bytes.push(modrm(reg.reg_code(), dst.reg_code()));
                    }
                }
            }
            Instruction::PUSH { src } => {
                bytes.push(0x50 + src.reg_code()); // OPCODE
            }
            Instruction::POP { dst } => {
                bytes.push(0x58 + dst.reg_code()); // OPCODE
            }
            Instruction::CALL { target } => match target {
                Operand::Imm(_) => unimplemented!(),
                Operand::Reg(_) => unimplemented!(),
            },
            Instruction::SYSCALL => {
                bytes.push(0x0F);
                bytes.push(0x05);
            }
        }

        bytes
    }
}
