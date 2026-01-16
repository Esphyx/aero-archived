use crate::{
    back_end::assembler::{
        headers::{CoffFileHeader, SectionHeader},
        instruction::{EncodedInstruction, Instruction},
        operand::Operand,
    },
    front_end::lexer,
};

// pub mod headers;
pub mod instruction;
pub mod operand;
pub mod register;

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
                sym_type: 0x20, // function
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

#[derive(Debug)]
pub struct Relocation {
    pub offset: usize,
    pub symbol: String,
    pub addend: i64,
}

pub trait ToBytes {
    fn to_bytes(&self) -> Vec<u8>;
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

#[derive(Debug)]
pub struct Symbol {
    pub name: String,
    pub value: u32,
    pub section: i16,
    pub sym_type: u16,
    pub storage_class: u8,
}

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

pub mod headers {
    use super::ToBytes;

    #[repr(C)]
    pub struct CoffFileHeader {
        pub machine: u16,
        pub number_of_sections: u16,
        pub time_date_stamp: u32,
        pub pointer_to_symbol_table: u32,
        pub number_of_symbols: u32,
        pub size_of_optional_header: u16,
        pub characteristics: u16,
    }

    impl ToBytes for CoffFileHeader {
        fn to_bytes(&self) -> Vec<u8> {
            let mut buf = Vec::with_capacity(size_of::<Self>());

            buf.extend(&self.machine.to_le_bytes());
            buf.extend(&self.number_of_sections.to_le_bytes());
            buf.extend(&self.time_date_stamp.to_le_bytes());
            buf.extend(&self.pointer_to_symbol_table.to_le_bytes());
            buf.extend(&self.number_of_symbols.to_le_bytes());
            buf.extend(&self.size_of_optional_header.to_le_bytes());
            buf.extend(&self.characteristics.to_le_bytes());

            buf
        }
    }

    impl Default for CoffFileHeader {
        fn default() -> Self {
            const AMD64: u16 = 0x8664;
            const CHARACTERISTICS: u16 = 0x0022;

            Self {
                machine: AMD64,
                number_of_sections: 0,
                time_date_stamp: 0,
                pointer_to_symbol_table: 0,
                number_of_symbols: 0,
                size_of_optional_header: 0,
                characteristics: CHARACTERISTICS,
            }
        }
    }

    #[derive(Default)]
    #[repr(C)]
    pub struct SectionHeader {
        pub name: [u8; 8],
        pub virtual_size: u32,
        pub virtual_address: u32,
        pub size_of_raw_data: u32,
        pub pointer_to_raw_data: u32,
        pub pointer_to_relocations: u32,
        pub pointer_to_line_numbers: u32,
        pub number_of_relocations: u16,
        pub number_of_line_numbers: u16,
        pub characteristics: u32,
    }

    impl ToBytes for SectionHeader {
        fn to_bytes(&self) -> Vec<u8> {
            let mut buf = Vec::with_capacity(size_of::<Self>());

            buf.extend(&self.name);
            buf.extend(&self.virtual_size.to_le_bytes());
            buf.extend(&self.virtual_address.to_le_bytes());
            buf.extend(&self.size_of_raw_data.to_le_bytes());
            buf.extend(&self.pointer_to_raw_data.to_le_bytes());
            buf.extend(&self.pointer_to_relocations.to_le_bytes());
            buf.extend(&self.pointer_to_line_numbers.to_le_bytes());
            buf.extend(&self.number_of_relocations.to_le_bytes());
            buf.extend(&self.number_of_line_numbers.to_le_bytes());
            buf.extend(&self.characteristics.to_le_bytes());

            buf
        }
    }

    #[repr(C)]
    pub struct ImageOptionalHeader64 {
        pub magic: u16,
        pub major_linker_version: u8,
        pub minor_linker_version: u8,
        pub size_of_code: u32,
        pub size_of_uninitialized_data: u32,
        pub address_of_entry_point: u32,
        pub base_of_code: u32,
        pub image_base: u64,
        pub section_alignment: u32,
        pub file_alignment: u32,
        pub major_operating_system_version: u16,
        pub minor_operating_system_version: u16,
        pub major_image_version: u16,
        pub minor_image_version: u16,
        pub major_subsystem_version: u16,
        pub minor_subsystem_version: u16,
        pub win32_version_value: u32,
        pub size_of_image: u32,
        pub size_of_headers: u32,
        pub check_sum: u32,
        pub subsystem: u16,
        pub dii_characteristics: u16,
        pub size_of_stack_reserve: u64,
        pub size_of_stack_commit: u64,
        pub size_of_heap_reserve: u64,
        pub size_of_heap_commit: u64,
        pub loader_flags: u32,
        pub number_of_rva_and_sizes: u32,
    }

    impl ToBytes for ImageOptionalHeader64 {
        fn to_bytes(&self) -> Vec<u8> {
            let mut buf = Vec::with_capacity(size_of::<Self>());

            buf.extend(&self.magic.to_le_bytes());
            buf.push(self.major_linker_version);
            buf.push(self.minor_linker_version);
            buf.extend(&self.size_of_code.to_le_bytes());
            buf.extend(&self.size_of_uninitialized_data.to_le_bytes());
            buf.extend(&self.address_of_entry_point.to_le_bytes());
            buf.extend(&self.base_of_code.to_le_bytes());
            buf.extend(&self.image_base.to_le_bytes());
            buf.extend(&self.section_alignment.to_le_bytes());
            buf.extend(&self.file_alignment.to_le_bytes());
            buf.extend(&self.major_operating_system_version.to_le_bytes());
            buf.extend(&self.minor_operating_system_version.to_le_bytes());
            buf.extend(&self.major_image_version.to_le_bytes());
            buf.extend(&self.minor_image_version.to_le_bytes());
            buf.extend(&self.major_subsystem_version.to_le_bytes());
            buf.extend(&self.minor_subsystem_version.to_le_bytes());
            buf.extend(&self.win32_version_value.to_le_bytes());
            buf.extend(&self.size_of_image.to_le_bytes());
            buf.extend(&self.size_of_headers.to_le_bytes());
            buf.extend(&self.check_sum.to_le_bytes());
            buf.extend(&self.subsystem.to_le_bytes());
            buf.extend(&self.dii_characteristics.to_le_bytes());
            buf.extend(&self.size_of_stack_reserve.to_le_bytes());
            buf.extend(&self.size_of_stack_commit.to_le_bytes());
            buf.extend(&self.size_of_heap_reserve.to_le_bytes());
            buf.extend(&self.size_of_heap_commit.to_le_bytes());
            buf.extend(&self.loader_flags.to_le_bytes());
            buf.extend(&self.number_of_rva_and_sizes.to_le_bytes());

            buf
        }
    }

    impl Default for ImageOptionalHeader64 {
        fn default() -> Self {
            const MAGIC_FOR_64BIT_HEADER: u16 = 0x20B;

            Self {
                magic: MAGIC_FOR_64BIT_HEADER,
                major_linker_version: 0,
                minor_linker_version: 0,
                size_of_code: 0,
                size_of_uninitialized_data: 0,
                address_of_entry_point: 0,
                base_of_code: 0,
                image_base: 0,
                section_alignment: 0,
                file_alignment: 0,
                major_operating_system_version: 0,
                minor_operating_system_version: 0,
                major_image_version: 0,
                minor_image_version: 0,
                major_subsystem_version: 0,
                minor_subsystem_version: 0,
                win32_version_value: 0,
                size_of_image: 0,
                size_of_headers: 0,
                check_sum: 0,
                subsystem: 0,
                dii_characteristics: 0,
                size_of_stack_reserve: 0,
                size_of_stack_commit: 0,
                size_of_heap_reserve: 0,
                size_of_heap_commit: 0,
                loader_flags: 0,
                number_of_rva_and_sizes: 0,
            }
        }
    }
}
