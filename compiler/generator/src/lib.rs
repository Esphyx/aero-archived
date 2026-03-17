use assembler::assembler::{AssemblyProgram, instruction::{Immediate, Instruction, Operand, Reg}};



pub fn generate() -> AssemblyProgram {
    let mut program = AssemblyProgram::new();

    syscall_exit(&mut program, 0);
    program
}

fn syscall_print(program: &mut AssemblyProgram, pointer: Reg, len: u8) {
    program.add_instruction(Instruction::MOV {
        dst: Reg::RAX,
        src: Operand::Imm(Immediate::Byte(1)),
    });

    program.add_instruction(Instruction::MOV {
        dst: Reg::RDI,
        src: Operand::Imm(Immediate::Byte(1)),
    });

    program.add_instruction(Instruction::MOV {
        dst: Reg::RSI,
        src: Operand::Reg(pointer),
    });

    program.add_instruction(Instruction::MOV {
        dst: Reg::RDX,
        src: Operand::Imm(Immediate::Byte(len)),
    });

    program.add_instruction(Instruction::SYSCALL);
}

fn syscall_exit(program: &mut AssemblyProgram, exit_code: u8) {
    program.add_instruction(Instruction::MOV {
        dst: Reg::RAX,
        src: Operand::Imm(Immediate::Byte(60)),
    });

    program.add_instruction(Instruction::MOV {
        dst: Reg::RDI,
        src: Operand::Imm(Immediate::Byte(exit_code)),
    });
    program.add_instruction(Instruction::SYSCALL);
}
