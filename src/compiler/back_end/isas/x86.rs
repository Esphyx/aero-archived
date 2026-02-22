use crate::compiler::back_end::assembler::isa::{AssemblerBackend, ISABuilder};

struct X86 {}

impl AssemblerBackend for X86 {
    fn isa(&self) -> &ISABuilder {
        let mut isa = ISABuilder::new();

        todo!()
    }
}
