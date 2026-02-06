use super::{
    super::front_end::grammar::ast::SourceAST, assembler::instruction::Instruction,
};

pub fn generate(_ast: &SourceAST) -> Vec<Instruction> {
    todo!()
    // let mut instructions = Vec::new();

    // match &ast.program {
    //     Expression::LetBinding { name, value, after } => {
    //         if name == "main" {
    //             generate_expression(&value, &mut instructions);
    //         } else {
    //             todo!()
    //         }
    //     }
    //     _ => todo!(),
    // }

    // instructions.push(Instruction::MOV {
    //     dst: Register::RCX,
    //     src: Operand::Reg(Register::RAX),
    // });
    // instructions.push(Instruction::CALL {
    //     target: Operand::Symbol("ExitProcess".to_string()),
    // });

    // instructions
}

// fn generate_expression(expr: &Expression, instructions: &mut Vec<Instruction>) {
//     match expr {
//         Expression::Atom(atom) => match atom {
//             Atom::U32 => todo!(),
//             Atom::NumberLiteral(n) => {
//                 instructions.push(Instruction::MOV {
//                     dst: Register::RAX,
//                     src: Operand::Imm(*n),
//                 });

//                 instructions.push(Instruction::PUSH { src: Register::RAX });
//             }
//             Atom::Identifier(_) => todo!(),
//         },
//         Expression::BinaryOperator { left, op, right } => {
//             generate_expression(left, instructions);

//             generate_expression(right, instructions);

//             instructions.push(Instruction::POP { dst: Register::RBX });
//             instructions.push(Instruction::POP { dst: Register::RAX });

//             match op {
//                 BinaryOperator::Plus => {
//                     instructions.push(Instruction::ADD {
//                         dst: Register::RAX,
//                         src: Operand::Reg(Register::RBX),
//                     });
//                 }
//                 BinaryOperator::Minus => {
//                     instructions.push(Instruction::SUB {
//                         dst: Register::RAX,
//                         src: Operand::Reg(Register::RBX),
//                     });
//                 }
//             }

//             instructions.push(Instruction::PUSH { src: Register::RAX });
//         }
//         _ => todo!(),
//     }
// }
