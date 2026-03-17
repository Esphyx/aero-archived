use super::register::Register;


pub trait AssemblerBackend {
    fn isa(&self) -> &ISABuilder;
}

pub struct ISABuilder {
    registers: Vec<Register>,
}

impl ISABuilder {
    pub fn new() -> Self {
        Self {
            registers: Vec::new(),
        }
    }

    pub fn register(mut self, reg: Register) -> Self {
        self.registers.push(reg);
        self
    }

    pub fn build(self) {
        todo!()
    }
}
