use super::register::Register;

#[derive(Debug)]
pub enum Operand {
    Imm(u32),
    Reg(Register),
    Symbol(String),
}
