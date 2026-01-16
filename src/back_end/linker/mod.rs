mod import_descriptor;
mod portable_executable;

// pub fn link(binary: Vec<u8>) -> Result<File, Box<dyn std::error::Error>> {
//     generate_pe_file(&binary, "main.exe")
// }

// fn generate_pe_file(binary: &[u8], filename: &str) -> Result<File, Box<dyn std::error::Error>> {
//     let mut file = File::create(filename)?;

//     let mut dos_stub = vec![0u8; 128];
//     dos_stub[0..2].copy_from_slice(b"MZ");
//     dos_stub[0x3C..0x40].copy_from_slice(&0x80u32.to_le_bytes());
//     file.write_all(&dos_stub)?;

//     let file_header = COFFFileHeader {
//         machine: 0x8664,
//         number_of_sections: 2, // .text + .idata
//         time_date_stamp: 0,
//         pointer_to_symbol_table: 0,
//         number_of_symbols: 0,
//         size_of_optional_header: size_of::<ImageOptionalHeader64>() as u16,
//         characteristics: 0x0022,
//     };

//     let optional_header = ImageOptionalHeader64 {
//         magic: 0x20B,
//         major_linker_version: 0,
//         minor_linker_version: 0,
//         size_of_code: 0x200,
//         size_of_uninitialized_data: 0,
//         address_of_entry_point: 0x1000, // start of text section
//         base_of_code: 0x1000,
//         image_base: 0x400000,
//         section_alignment: 0x1000,
//         file_alignment: 0x200,
//         major_operating_system_version: 6,
//         minor_operating_system_version: 0,
//         major_image_version: 0,
//         minor_image_version: 0,
//         major_subsystem_version: 6,
//         minor_subsystem_version: 0,
//         win32_version_value: 0,
//         size_of_image: 0x2000,
//         size_of_headers: 0x200,
//         check_sum: 0,
//         subsystem: 3,
//         dii_characteristics: 0,
//         size_of_stack_reserve: 0x100000,
//         size_of_stack_commit: 0x1000,
//         size_of_heap_reserve: 0x100000,
//         size_of_heap_commit: 0x1000,
//         loader_flags: 0,
//         number_of_rva_and_sizes: 16,
//     };

//     file.seek(SeekFrom::Start(0x80))?;
//     file.write_all(b"PE\0\0")?;

//     let file_header_bytes = unsafe {
//         from_raw_parts(
//             &file_header as *const COFFFileHeader as *const u8,
//             size_of::<COFFFileHeader>(),
//         )
//     };
//     file.write_all(file_header_bytes)?;

//     let optional_header_bytes: &[u8] = unsafe {
//         std::slice::from_raw_parts(
//             &optional_header as *const ImageOptionalHeader64 as *const u8,
//             size_of::<ImageOptionalHeader64>(),
//         )
//     };
//     file.write_all(optional_header_bytes)?;

//     let text_section_header = SectionHeader {
//         name: 0x007874652E6D742E,
//         virtual_size: binary.len() as u32,
//         virtual_address: 0x1000,
//         size_of_raw_data: 0x200,
//         pointer_to_raw_data: 0x200,
//         pointer_to_relocations: 0,
//         pointer_to_line_numbers: 0,
//         number_of_relocations: 0,
//         number_of_line_numbers: 0,
//         characteristics: 0x60000020,
//     };

//     let text_section_header_bytes = unsafe {
//         std::slice::from_raw_parts(
//             &text_section_header as *const SectionHeader as *const u8,
//             size_of::<SectionHeader>(),
//         )
//     };
//     file.write_all(text_section_header_bytes)?;

//     let idata_rva = 0x2000;
//     let idata_offset = 0x400;
//     let idata_section = SectionHeader {
//         name: 0x00616469742E0000, // ".idata"
//         virtual_size: 200,
//         virtual_address: idata_rva,
//         size_of_raw_data: 0x200,
//         pointer_to_raw_data: idata_offset,
//         pointer_to_relocations: 0,
//         pointer_to_line_numbers: 0,
//         number_of_relocations: 0,
//         number_of_line_numbers: 0,
//         characteristics: 0x40000040,
//     };

//     let idata_section_bytes = unsafe {
//         from_raw_parts(
//             &idata_section as *const SectionHeader as *const u8,
//             std::mem::size_of::<SectionHeader>(),
//         )
//     };
//     file.write_all(idata_section_bytes)?;

//     let current_pos = file.seek(SeekFrom::Current(0))?;
//     if current_pos < 0x200 {
//         file.write_all(&vec![0u8; (0x200 - current_pos) as usize])?;
//     }

//     file.write_all(binary)?;
//     if binary.len() < 0x200 {
//         file.write_all(&vec![0u8; 0x200 - binary.len()])?;
//     }

//     file.seek(SeekFrom::Start(idata_offset as u64))?;

//     let mut hint_name = vec![];
//     hint_name.extend(&0u16.to_le_bytes());
//     hint_name.extend(b"ExitProcess\0");

//     let iat_rva = idata_rva + 0x20;
//     let mut iat = vec![];
//     iat.extend(&(idata_rva + 0x10).to_le_bytes());
//     iat.extend(&0u64.to_le_bytes());

//     let mut import_desc = vec![];
//     import_desc.extend(&(idata_rva + 0x10).to_le_bytes());
//     import_desc.extend(&0u32.to_le_bytes());
//     import_desc.extend(&0u32.to_le_bytes());
//     import_desc.extend(&(idata_rva + 0x30).to_le_bytes());
//     import_desc.extend(&(idata_rva + 0x10).to_le_bytes());
//     import_desc.extend(&[0u8; 20]);

//     let mut dll_name = b"KERNEL32.dll\0".to_vec();

//     file.write_all(&import_desc)?;
//     file.write_all(&hint_name)?;
//     file.write_all(&iat)?;
//     file.write_all(&dll_name)?;

//     Ok(file)
// }
