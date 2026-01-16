use crate::{
    back_end::assembler::{instruction::Instruction, operand::Operand, register::Register},
    front_end::ast::{AbstractSyntaxTree, BinaryOperator, Expression},
};

pub fn generate(ast: &AbstractSyntaxTree) -> Vec<Instruction> {
    let mut instructions = Vec::new();

    generate_expression(&ast.expression, &mut instructions);

    instructions.push(Instruction::MOV {
        dst: Register::RCX,
        src: Operand::Reg(Register::RAX),
    });
    instructions.push(Instruction::CALL {
        target: Operand::Symbol("ExitProcess".to_string()),
    });

    instructions
}

fn generate_expression(expr: &Expression, instructions: &mut Vec<Instruction>) {
    match expr {
        Expression::NumberLiteral(n) => {
            instructions.push(Instruction::MOV {
                dst: Register::RAX,
                src: Operand::Imm(*n),
            });

            instructions.push(Instruction::PUSH { src: Register::RAX });
        }
        Expression::BinaryOperator { left, op, right } => {
            generate_expression(left, instructions);

            generate_expression(right, instructions);

            instructions.push(Instruction::POP { dst: Register::RBX });
            instructions.push(Instruction::POP { dst: Register::RAX });

            match op {
                BinaryOperator::Plus => {
                    instructions.push(Instruction::ADD {
                        dst: Register::RAX,
                        src: Operand::Reg(Register::RBX),
                    });
                }
                BinaryOperator::Minus => {
                    instructions.push(Instruction::SUB {
                        dst: Register::RAX,
                        src: Operand::Reg(Register::RBX),
                    });
                }
            }

            instructions.push(Instruction::PUSH { src: Register::RAX });
        }
    }
}
