use std::path::Path;
use std::fs;

// Print information about an ELF binary from it's header
// Header is parsed so the output is more verbose
pub fn header_info() -> error::Result<()> {
    /* TODO:
        - integrade into main function (add command: info/readelf)
        - change how the file is loaded (pass 'something' to the function?)
    */
    // let mut input_path = user_input();
    let path = Path::new(&input_path);
    let buffer = fs::read(path)?;
    let head_obj = goblin::elf::Elf::parse_header(&buffer).unwrap();

    let magic = head_obj.e_ident;

    let arch = match head_obj.e_ident[4] {
        1 => "ELF 32",
        2 => "ELF 64",
        _ => "error in format"
    };

    let endianness = match head_obj.e_ident[4] {
        1 => "little endian",
        2 => "big endian",
        _ => "error in format"
    };

    let version = match head_obj.e_ident[6] {
        1 => "1 (current)",
        _ => "error in format"
    };

    let ABI_list = ["System V", "HP-UX", "NetBSD", "Linux", "GNU Hurd", "Solaris", "AIX", "IRIX", "FreeBSD", "Tru64", "Novell Modesto", "OpenBSD", "OpenVMS", "NonStop Kernel", "AROS", "Fenix OS", "CloudABI", "Stratus Technologies OpenVOS"];

    let ABI = ABI_list[usize::from(head_obj.e_ident[7])];

    let ABI_version = head_obj.e_ident[8];

    let object_file_type = match head_obj.e_type {
        0 => "NONE (No file type)",
        1 => "REL (Relocations file)",
        2 => "EXEC (Executable file)",
        3 => "DYN (Shared object file)",
        4 => "CODE (Core file)",
        65024 => "LOOS",
        65279 => "HIOS",
        65280 => "LOPROC",
        65535 => "HIPROC",
        _ => "error in format"
    };

    println!("ELF Header:
    Magic:      {:x?}
    Class:                              {}
    Data:                               {}
    Version:                            {}
    OS/ABI:                             {}
    ABI Version:                        {}
    Type:                               {}
    Version:                            {:#x}
    Entry point address:                {:#x}
    Start of program headers:           {} (bytes into file)
    Start of section headers:           {} (bytes into file)
    Flags:                              {:#x}
    Size of this header:                {} (bytes)
    Size of program headers:            {} (bytes)
    Number of program headers:          {}
    Size of section headers:            {} (bytes)
    Number of section headers:          {}
    Section header string table index:  {}",
    magic, arch, endianness, version, ABI, ABI_version, object_file_type, head_obj.e_version, head_obj.e_entry, head_obj.e_phoff, head_obj.e_shoff, head_obj.e_flags, head_obj.e_ehsize, head_obj.e_phentsize, head_obj.e_phnum, head_obj.e_shentsize, head_obj.e_shnum, head_obj.e_shstrndx);
    Ok(())
}
