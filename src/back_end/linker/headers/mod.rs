pub mod coff_file_header;
pub mod image_optional;
pub mod section;

pub trait ToBytes {
    fn to_bytes(&self) -> &[u8];
}
