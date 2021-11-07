/*
 * This file handles getting static information about the binary such as:
 * - listing functions
 * - disassembling functions
 *
 */
use goblin::{error, Object, elf::sym};
use std::path::Path;
use std::fs;
use termion::{color, style};


pub fn list_func(obj: &Object) {
    // Match executable type and list functions accordingly to the format
    match obj {
        // Linux
        // Print names and addresses from .symtab section
        Object::Elf(elf) => { 
            // List all symbols that are functions
            for section in &elf.syms {
                if section.is_function() {
                    // Print the function address and name
                    // section.st_value corresponds to string offset from elf.strtab

                    // Pad zeroes to match 32bit or 64bit address length
                    if elf.is_64 {
                        println!("{}{:#018x}  {}{}", color::Fg(color::Blue), section.st_value,
                            color::Fg(color::Red), elf.strtab.get_at(section.st_name).unwrap());
                    } else {
                        println!("{}{:#010x}  {}{}", color::Fg(color::Blue), section.st_value,
                            color::Fg(color::Red), elf.strtab.get_at(section.st_name).unwrap());
                    }
                }
            }
        }
        // TODO Windows
        Object::PE(pe)              => { println!("pe"); }
        Object::Mach(mach)          => { println!("mach: {:#?}", &mach); }
        Object::Archive(archive)    => { println!("archive: {:#?}", &archive); }
        Object::Unknown(magic)      => { println!("unknown magic: {:#x}", magic); }
    }
}

//TODO these two need to be in a separate file for handling running the program
pub fn load_file(filename: String) -> Vec<u8> {

    let path = Path::new(&filename);
    fs::read(path).unwrap()
}

pub fn parse_file(buffer: &Vec<u8>) -> Object {

    Object::parse(buffer).unwrap()
}
