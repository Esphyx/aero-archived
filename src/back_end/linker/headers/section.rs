use crate::linker::headers::ToBytes;

#[repr(C)]
pub struct SectionHeader {
    pub name: u64,
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
    fn to_bytes(&self) -> &[u8] {
        unsafe { std::slice::from_raw_parts((self as *const _) as *const u8, size_of::<Self>()) }
    }
}
