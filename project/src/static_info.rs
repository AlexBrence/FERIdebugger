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
            let mut op: &str = "";
            let mut operands: String = "".to_owned();   // modified, printable operands

            // Init colors
            let mut color1: Box<dyn FgColor> = Box::new(color::Fg(color::Yellow)); ;
            let mut color2: Box<dyn FgColor> = Box::new(color::Fg(color::White));;

            for i in insns.as_ref() {

                address = i.address();
                opcode = i.mnemonic().unwrap();
                op = i.op_str().unwrap();
                operands = op.to_owned();

                // Format output based on opcode
                match opcode {
                    "call" => {
                        color1 = Box::new(color::Fg(color::Magenta));

                        // Resolve address
                        let oper: u64 = i64::from_str_radix(op.trim_start_matches("0x"), 16).unwrap() as u64;
                        color2 = Box::new(color::Fg(color::Blue));
                        for (name, (addr, size)) in func_table.iter() {
                            if *addr + base_addr == oper {
                                operands = format!("{}<{}> {}0x{:x}", color::Fg(color::Red), name.as_str(),
                                    color::Fg(color::Blue), *addr + base_addr);
                                break;
                            }
                        }
                    },

                    "nop" | "leave" | "ret" => { 
                        color1 = Box::new(color::Fg(color::Red)); 
                        color2 = Box::new(color::Fg(color::Red));
                    },

                    "mov" | "movzx" | "lea" => {
                        color1 = Box::new(color::Fg(color::Cyan));
                        color2 = Box::new(color::Fg(color::White));
                    },

                    // Make all jumps green
                    "jmp" | "je" | "jz" | "jne" | "jnz" | "jg" | "jnle" | "jge" | "jnl" | "jl" 
                        | "jnge" | "jle" | "jng" | "je" | "jz" | "jne" | "jnz" | "ja" | "jnbe" 
                        | "jae" | "jnb" | "jb" | "jnae" | "jbe" | "jna"
                        => { 
                            color1 = Box::new(color::Fg(color::Green));
                            color2 = Box::new(color::Fg(color::Green));
                        },

                    _ => {
                        color1 = Box::new(color::Fg(color::Yellow));
                        color2 = Box::new(color::Fg(color::Yellow));
                    }
                }

                /*
                // TODO Color operands?
                match operands {
                    "rax" | "rcx" | "rdx" | "rbx" | "rsp" | "rbp" | "rsi" | "rdi"
                        | "rax" | "ecx" | "edx" | "ebx" | "esp" | "ebp" | "esi" | "edi"
                        | "ax" | "cx" | "dx" | "bx" | "sp" | "bp" | "si" | "di"
                        | "ah" | "al" | "ch" | "cl" | "dh" | "dl" | "bh" | "bl" | "spl" | "bpl" | "sil" | "dil"
                        => {}
                }
                */
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
                    let mut addr = section.st_value;
                    let size = section.st_size;

                    // TODO test while running
                    // Return string ??? if file is stripped
                    let func_name: String = match elf.strtab.get_at(section.st_name) {
                        Some(name)  => name.to_string(),
                        None        => format!("???"),
                    };
                    // Read offsets from /usr/lib/x86_64-linux-gnu/libc.so.6
                    // if address is 0x0 its libc function

                    // .got -> holds actual offsets
                    // .plt -> stub that look up addresses in .got.plt
                    // .got.plt -> target addresses (after they have been looked up)
                    //
                    // call   0x10a0 <printf@plt>
                    //
                    // printf@plt:
                    //      endbr64 
                    //      bnd jmp QWORD PTR [rip+0x2f1d]   # 0x3fc8 <printf@got.plt>
                    //                                          |
                    //                                        .rela.plt -> offset
                    // $ readelf -r test/elf64/test_loop
                    // PLAN:
                    // if addr == 0 {
                    //      disassemble function@plt
                    //      take operands from first jmp
                    //      return name that corresponds to address of operand in .rela.plt
                    // }
                    // .rela.plt -> .got.plt
                    // elf.pltrelocs
                    //
                    // elf.dynrelas
                    // elf.dynstrtab
                    // elf.libraries -> "libc.so.6"

                    // resolve functions in GOT
                    if section.st_value == 0 {
                        let strtab: Vec<&str> = elf.dynstrtab.to_vec().unwrap();

                        for reloc in elf.pltrelocs.into_iter() {
                            if func_name.starts_with(strtab[reloc.r_sym + 1]) {
                                //println!("{:x} {}", reloc.r_offset, strtab[reloc.r_sym+1]);
                                //println!("{:?}", elf.pltrelocs);
                                addr = reloc.r_offset;
                                break;
                            }
                        }
                    }

                    // section.st_value corresponds to string offset from elf.strtab
                    func_table.insert(func_name, (addr, size));

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
