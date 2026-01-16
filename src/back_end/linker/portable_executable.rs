use crate::linker::headers::{
    ToBytes, coff_file_header::COFFFileHeader, image_optional::ImageOptionalHeader64,
    section::SectionHeader,
};

pub enum PESection {
    Code(Vec<u8>),
    Imports(Vec<Import>),
}

pub struct Import {
    pub dll_name: String,
    pub functions: Vec<String>,
}

pub struct PortableExecutable {
    imports: Vec<Import>,
    code: Vec<u8>,
}

impl PortableExecutable {
    pub fn new() -> Self {
        Self {
            imports: Vec::new(),
            code: Vec::new(),
        }
    }

    pub fn add_import(mut self, import: Import) -> Self {
        self.imports.push(import);
        self
    }

    pub fn set_code(mut self, code: Vec<u8>) -> Self {
        self.code = code;
        self
    }

    fn dos_stub() -> Vec<u8> {
        let mut dos_stub = vec![0u8; 128];
        dos_stub[0..2].copy_from_slice(b"MZ");
        dos_stub[0x3C..0x40].copy_from_slice(&(0x80 as u32).to_le_bytes());
        dos_stub
    }

    pub fn generate(mut self, code: &[u8]) -> Vec<u8> {
        let mut buffer = Vec::new();

        let dos_stub = Self::dos_stub();

        let signature = b"PE\0\0";

        let coff_file_header = COFFFileHeader::default();

        buffer
    }
}
