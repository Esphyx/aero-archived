use crate::back_end::assembler::{
    coff::CommonObject,
    instruction::{EncodedInstruction, Instruction},
    operands::Operand,
};

pub mod coff;
pub mod instruction;
pub mod operands;
pub mod register;

pub trait ToBytes {
    fn to_bytes(&self) -> Vec<u8>;
}

#[derive(Debug)]
pub struct Relocation {
    pub offset: usize,
    pub symbol: String,
    pub addend: i64,
}

#[derive(Debug)]
pub struct Symbol {
    pub name: String,
    pub value: u32,
    pub section: i16,
    pub sym_type: u16,
    pub storage_class: u8,
}

pub fn assemble(instructions: Vec<Instruction>) -> CommonObject {
    let mut all_bytes = Vec::new();
    let mut all_relocations = Vec::new();

    let mut current_offset = 0;

    for instruction in &instructions {
        let EncodedInstruction { bytes, relocations } = instruction.encode(current_offset);

        current_offset += bytes.len();

        all_bytes.extend(bytes);
        all_relocations.extend(relocations);
    }

    let mut symbols = Vec::new();

    for reloc in &all_relocations {
        if !symbols.iter().any(|s: &Symbol| s.name == reloc.symbol) {
            symbols.push(Symbol {
                name: reloc.symbol.clone(),
                value: 0,
                section: 0,
                sym_type: 0x20,   // function
                storage_class: 2, // external
            });
        }
    }

    CommonObject {
        text: all_bytes,
        relocations: all_relocations,
        symbols,
    }
}
