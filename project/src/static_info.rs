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
use std::fmt;
use capstone::prelude::*;

// https://gitlab.redox-os.org/redox-os/termion/-/issues/123
// Implementing traits to enable putting color in one variable
trait FgColor {
    fn write_to(&self, w: &mut dyn fmt::Write) -> fmt::Result;
}

impl<C> FgColor for color::Fg<C>
where
    C: color::Color,
{
    fn write_to(&self, w: &mut dyn fmt::Write) -> fmt::Result {
        write!(w, "{}", self)
    }
}

impl fmt::Display for dyn FgColor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::result::Result<(), fmt::Error> {
        self.write_to(f)
    }
}
// End of helper traits


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
            
            //TODO set base_addr dynamically when ASLR on
            // Set base_addr according to bitness
            let mut base_addr: u64;
            if is64 {
                base_addr = 0x555555554000;
            } else {
                base_addr = 0x56555000;
            }
            let func_base_addr: u64 = base_addr + *addr;

            // Interpret bytes with Capstone
            let insns = cap_obj.disasm_all(asm_bytes, func_base_addr)
                    .expect("Failed to disassemble");
                println!("Showing {} instructions from {}\n", insns.len(), func_name);

            let mut address: u64 = 0;
            let mut opcode: &str = "???";
            let mut operands: &str = "";

            // Init colors
            let mut color1: Box<dyn FgColor> = Box::new(color::Fg(color::White)); ;
            let mut color2: Box<dyn FgColor> = Box::new(color::Fg(color::Yellow));;

            for i in insns.as_ref() {

                address = i.address();
                opcode = i.mnemonic().unwrap();
                operands = i.op_str().unwrap();

                // Format output based on opcode
                match opcode {
                    // Resolve addresses by call instruction
                    "call" => {
                        let oper: u64 = i64::from_str_radix(operands.trim_start_matches("0x"), 16).unwrap() as u64;
                        for (name, (addr, size)) in func_table.iter() {
                            if *addr + base_addr == oper {
                                operands = name.as_str();
                                color1 = Box::new(color::Fg(color::Magenta));
                                color2 = Box::new(color::Fg(color::Red));
                                break;
                            }
                        }
                    },

                    // Make all jumps green
                    "jmp" | "je" | "jz" | "jne" | "jnz" | "jg" | "jnle" | "jge" | "jnl" | "jl" 
                        | "jnge" | "jle" | "jng" | "je" | "jz" | "jne" | "jnz" | "ja" | "jnbe" 
                        | "jae" | "jnb" | "jb" | "jnae" | "jbe" | "jna"
                        => { color1 = Box::new(color::Fg(color::Green)); },

                    _ => {
                        color1 = Box::new(color::Fg(color::White));
                        color2 = Box::new(color::Fg(color::Yellow));
                    }
                }
                // 64bit address padding
                if is64 {
                    println!("{}{:#018x}\t{}{}\t{}{}", color::Fg(color::Blue), address,
                        color1, opcode,
                        color2, operands);
                // 32bit address padding
                } else {
                    println!("{}{:#010x}\t{}{}\t{}{}", color::Fg(color::Blue), address,
                        color1, opcode,
                        color2, operands);
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
