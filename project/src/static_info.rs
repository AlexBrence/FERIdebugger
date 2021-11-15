/*
 * This file handles getting static information about the binary such as:
 * - listing functions
 * - disassembling functions
 *
 */
use goblin::{error, Object, elf::sym, elf::Sym};
use std::path::Path;
use std::fs;
use std::collections::HashMap;
use termion::{color, style};
use capstone::prelude::*;

pub fn list_func(obj: &Object) {
    let mut is64: bool = true;
    let func_table = get_func_table(obj, &mut is64);

    // Sort entries in HashMap by address
    let mut sorted_func_table: Vec<_> = func_table.iter().collect();
    sorted_func_table.sort_by_key(|a| a.1);

    println!();

    // 64bit address padding
    if is64 {
        for (name, (addr, size)) in sorted_func_table.iter() {
            println!("{}{:#018x}  {}{}", color::Fg(color::Blue), addr,
                color::Fg(color::Red), name);
        }
    // 32bit address padding
    } else {
        for (name, (addr, size)) in sorted_func_table.iter() {
            println!("{}{:#010x}  {}{}", color::Fg(color::Blue), addr,
                color::Fg(color::Red), name);
        }
    }
    println!();
}

pub fn disassemble(func_name: &str, obj: &Object, buff: &Vec<u8>, cap_obj: &Capstone) {
    let mut is64: bool = true;
    let func_table = get_func_table(obj, &mut is64);

    match func_table.get(func_name) {
        Some((addr, size)) => {
            let start: usize = *addr as usize;
            let end: usize = (*addr + *size) as usize;
            let asm_bytes = &buff[start..end];
            
            //TODO set base_addr dynamically
            let base_addr: u64 = *addr;

            // Interpret bytes with Capstone
            let insns = cap_obj.disasm_all(asm_bytes, base_addr)
                    .expect("Failed to disassemble");
                println!("Showing {} instructions from {}\n", insns.len(), func_name);

                // 64bit address padding
                if is64 {
                    for i in insns.as_ref() {
                        println!("{}{:#018x}\t{}{}\t{}{}", color::Fg(color::Blue), i.address(),
                            color::Fg(color::White), i.mnemonic().unwrap(),
                            color::Fg(color::Yellow), i.op_str().unwrap());
                    }
                // 32bit address padding
                } else {
                    for i in insns.as_ref() {
                        println!("{}{:#010x}\t{}{}\t{}{}", color::Fg(color::Blue), i.address(),
                            color::Fg(color::White), i.mnemonic().unwrap(),
                            color::Fg(color::Yellow), i.op_str().unwrap());
                    }
                }
        },
        None => { println!("{}Error: Function not found.", color::Fg(color::Red)); }
    }
    println!();
}

fn get_func_table(obj: &Object, is64: &mut bool) -> HashMap<String, (u64, u64)> {
    let mut func_table: HashMap<String, (u64, u64)> = HashMap::new();
    // Match executable type and list functions accordingly to the format
    match obj {
        // Linux
        // List names and addresses from .symtab section
        Object::Elf(elf) => { 
            // List all symbols that are functions
            for section in &elf.syms {
                if section.is_function() {
                    // Add function address and name to HashMap

                    // TODO test while running
                    // Return string ??? if file is stripped
                    let func_name: String = match elf.strtab.get_at(section.st_name) {
                        Some(name)  => name.to_string(),
                        None        => format!("???"),
                    };

                    // section.st_value corresponds to string offset from elf.strtab
                    func_table.insert(func_name, (section.st_value, section.st_size));

                    // Pad zeroes to match 32bit or 64bit address length
                    *is64 = elf.is_64;
                    /*
                    if elf.is_64 {
                        //println!("{}{:#018x}  {}{}", color::Fg(color::Blue), section.st_value,
                        //    color::Fg(color::Red), elf.strtab.get_at(section.st_name).unwrap());
                    } else {
                        //println!("{}{:#010x}  {}{}", color::Fg(color::Blue), section.st_value,
                        //    color::Fg(color::Red), elf.strtab.get_at(section.st_name).unwrap());
                    }
                    */
                }
            }
        }
        // TODO Windows
        Object::PE(pe)              => { println!("pe"); }
        Object::Mach(mach)          => { println!("mach: {:#?}", &mach); }
        Object::Archive(archive)    => { println!("archive: {:#?}", &archive); }
        Object::Unknown(magic)      => { println!("unknown magic: {:#x}", magic); }
    }
    func_table
}

//TODO these two need to be in a separate file for handling running the program
pub fn load_file(filename: String) -> Vec<u8> {

    let path = Path::new(&filename);
    fs::read(path).unwrap()
}

pub fn parse_file(buffer: &Vec<u8>) -> Object {

    Object::parse(buffer).unwrap()
}
