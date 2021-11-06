/*
 * This file handles getting static information about the binary such as:
 * - listing functions
 * - disassembling functions
 *
 */
use goblin::{error, Object, elf::sym};
use std::path::Path;
use std::fs;


pub fn list_func(obj: &Object) {
    // Match executable type and list functions accordingly to the format
    match obj {
        // Linux
        // TODO add memory address for each function
        Object::Elf(elf) => { 
            for label in elf.strtab.to_vec() {
                for l in label {
                    if !l.is_empty() && !l.starts_with("_") && !l.ends_with(".c") 
                        && l != "deregister_tm_clones" && l != "completed.8060" {
                        println!("{}", l);
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
