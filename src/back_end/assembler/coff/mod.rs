use crate::back_end::assembler::{Relocation, Symbol, ToBytes, coff::headers::{CoffFileHeader, SectionHeader}};

pub mod headers;

pub struct CoffRelocation {
    virtual_address: u32,
    symbol_table_index: u32,
    type_: u16,
}

impl ToBytes for CoffRelocation {
    fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(size_of::<Self>());

        buf.extend(self.virtual_address.to_le_bytes());
        buf.extend(self.symbol_table_index.to_le_bytes());
        buf.extend(self.type_.to_le_bytes());

        buf
    }
}

#[derive(Debug)]
pub struct CommonObject {
    pub text: Vec<u8>,
    pub relocations: Vec<Relocation>,
    pub symbols: Vec<Symbol>,
}

impl ToBytes for CommonObject {
    fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Vec::new();

        const TRUE_SIZE_OF_COFF_FILE_HEADER: u32 = 20;
        const TRUE_SIZE_OF_SECTION_HEADER: u32 = 40;
        const TRUE_SIZE_OF_COFF_RELOCATION: u32 = 10;

        let mut coff_header = CoffFileHeader::default();
        coff_header.number_of_sections = 1;
        coff_header.pointer_to_symbol_table = TRUE_SIZE_OF_COFF_FILE_HEADER
            + TRUE_SIZE_OF_SECTION_HEADER
            + self.text.len() as u32
            + self.relocations.len() as u32 * TRUE_SIZE_OF_COFF_RELOCATION;
        coff_header.number_of_symbols = self.symbols.len() as u32;

        buf.extend(coff_header.to_bytes());

        // .text SECTION HEADER

        let mut section_header = SectionHeader::default();
        section_header.name = *b".text\0\0\0";
        section_header.size_of_raw_data = self.text.len() as u32;
        section_header.pointer_to_raw_data =
            TRUE_SIZE_OF_COFF_FILE_HEADER + TRUE_SIZE_OF_SECTION_HEADER;
        section_header.pointer_to_relocations =
            self.text.len() as u32 + TRUE_SIZE_OF_COFF_FILE_HEADER + TRUE_SIZE_OF_SECTION_HEADER;
        section_header.number_of_relocations = self.relocations.len() as u16;
        section_header.characteristics = 0x60000020;

        buf.extend(section_header.to_bytes());

        buf.extend(&self.text);

        for reloc in &self.relocations {
            let sym_index = self
                .symbols
                .iter()
                .position(|s| s.name == reloc.symbol)
                .unwrap() as u32;

            let coff_rel = CoffRelocation {
                virtual_address: reloc.offset as u32,
                symbol_table_index: sym_index,
                type_: 0x0004,
            };

            buf.extend(coff_rel.to_bytes());
        }

        let mut string_table = Vec::new();
        string_table.extend(&4u32.to_le_bytes()); // PLACEHOLDER

        for sym in &self.symbols {
            if sym.name.len() <= 8 {
                let mut name_bytes = [0u8; 8];
                name_bytes[..sym.name.len()].copy_from_slice(sym.name.as_bytes());
                buf.extend(name_bytes);
            } else {
                let offset = string_table.len() as u32;
                buf.extend(&0u32.to_le_bytes());
                buf.extend(&offset.to_le_bytes());
                string_table.extend(sym.name.as_bytes());
                string_table.push(0);
            }

            buf.extend(&sym.value.to_le_bytes());
            buf.extend(&sym.section.to_le_bytes());
            buf.extend(&sym.sym_type.to_le_bytes());
            buf.push(sym.storage_class);
            buf.push(0);
        }

        let string_table_len = string_table.len() as u32;
        string_table[..4].copy_from_slice(&string_table_len.to_le_bytes());
        buf.extend(&string_table);

        buf
    }
}
