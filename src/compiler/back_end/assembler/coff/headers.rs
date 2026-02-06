use super::super::ToBytes;

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
