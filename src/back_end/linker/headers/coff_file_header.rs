use crate::linker::headers::{ToBytes, image_optional::ImageOptionalHeader64};

#[repr(C)]
pub struct COFFFileHeader {
    pub machine: u16,
    pub number_of_sections: u16,
    pub time_date_stamp: u32,
    pub pointer_to_symbol_table: u32,
    pub number_of_symbols: u32,
    pub size_of_optional_header: u16,
    pub characteristics: u16,
}

impl ToBytes for COFFFileHeader {
    fn to_bytes(&self) -> &[u8] {
        unsafe { std::slice::from_raw_parts((self as *const _) as *const u8, size_of::<Self>()) }
    }
}

impl Default for COFFFileHeader {
    fn default() -> Self {
        const AMD64: u16 = 0x8664;
        const CHARACTERISTICS: u16 = 0x0022;

        Self {
            machine: AMD64,
            number_of_sections: 0,
            time_date_stamp: 0,
            pointer_to_symbol_table: 0,
            number_of_symbols: 0,
            size_of_optional_header: size_of::<ImageOptionalHeader64>() as u16,
            characteristics: CHARACTERISTICS,
        }
    }
}
