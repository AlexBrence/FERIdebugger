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
use crate::program;

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

    let mut base_addr: u64 = match is64 {
        true => 0x555555554000,
        false => 0x56555000,
    };

    println!();

    // 64bit address padding
    if is64 {
        for (name, (addr, size)) in sorted_func_table.iter() {
            println!("{}{:#018x}  {}{}", color::Fg(color::Blue), addr + base_addr,
                color::Fg(color::Red), name);
        }
    // 32bit address padding
    } else {
        for (name, (addr, size)) in sorted_func_table.iter() {
            println!("{}{:#010x}  {}{}", color::Fg(color::Blue), addr + base_addr,
                color::Fg(color::Red), name);
        }
    }
    println!();
}

pub fn disassemble(func_id: &str, obj: &Object, buff: &Vec<u8>, cap_obj: &Capstone, program: &mut program::Program) {
    let mut is64: bool = true;
    let func_table = get_func_table(obj, &mut is64);

    // Get instruction pointer if program is running
    let ip_value = program.get_user_struct().regs.rip;

    //TODO set base_addr dynamically when ASLR on
    // Set base_addr according to bitness
    let mut base_addr: u64 = match is64 {
        true => 0x555555554000,
        false => 0x56555000,
    };
    let mut func_base_addr: u64 = 0;
    let mut start: usize = 0;
    let mut end: usize = 0;
    let mut func_name: String = String::new();

    match func_table.get(func_id) {
        Some((addr, size)) => {
            start = *addr as usize;
            end = (*addr + *size) as usize;
            func_base_addr = base_addr + *addr;
        },
        None => {
            // Disassemble address
            // Check if valid address
            match u64::from_str_radix(&func_id.trim_start_matches("0x"), 16) {
                // Find start of function and disassemble it
                Ok(a) => {
                    for (key, (addr, size)) in &func_table {
                        if (*addr + base_addr) <= a && (addr + base_addr + size) >= a {
                            start = *addr as usize;
                            end = (*addr + *size) as usize;
                            func_base_addr = base_addr + *addr;
                            func_base_addr = base_addr + *addr;
                            func_name = key.to_string();
                            break
                        }
                    }
                },
                Err(f) => {
                    eprintln!("Invalid address");
                    return;
                }
            };
        },
    };

    let asm_bytes = &buff[start..end];
            
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
                match i64::from_str_radix(op.trim_start_matches("0x"), 16) {
                    Ok(val) => {
                        let mut resolved = false;
                        let mut ins_addr = val as u64;
                        color2 = Box::new(color::Fg(color::Blue));
                        for (name, (addr, size)) in func_table.iter() {
                            if *addr + base_addr == ins_addr {
                                operands = format!("{}0x{:x} {}-> {}{}", color::Fg(color::Blue), *addr + base_addr,
                                    color::Fg(color::White), color::Fg(color::Red), name.as_str());
                                resolved = true;
                                break;
                            }
                        }
                        // If address is not resolved check if its from PLT
                        // Get first 3 instructions
                        // Check if one of them is call
                        // Check what call resolves to (add next address with the offset in call)
                        // Interpret bytes with Capstone
                        if !resolved {
                            let address_operand = val as u64;
                            let start_c: usize = (address_operand - base_addr) as usize;
                            let end_c: usize = start_c + 12;
                            let asm_bytes_call_plt = &buff[start_c..end];
                            match cap_obj.disasm_count(asm_bytes_call_plt, start_c as u64, 3) {
                                Ok(isns_c) => {
                                    let mut offset_got: u64 = 0;
                                    let mut pc_value: u64 = 0;
                                    let mut found_jmp = false;
                                    let mut got_all_offsets = false;

                                    for i_c in isns_c.as_ref() {
                                        if !got_all_offsets {
                                            match i_c.op_str() {
                                                Some(opc) => {
                                                    if !found_jmp {
                                                        if i_c.mnemonic().unwrap().contains("jmp") {
                                                            //println!("FOUND JMP FROM PLT 0x{:x} {}", i_c.address(), opc);
                                                            offset_got = u64::from_str_radix(opc.split("+").last().unwrap().replace("]", "").trim().trim_start_matches("0x"), 16).unwrap();
                                                            found_jmp = true
                                                        }
                                                    } else {
                                                        pc_value = i_c.address();
                                                        //println!("PC {:x}", pc_value);
                                                        got_all_offsets = true;
                                                    }
                                                },
                                                None => {},
                                            };
                                        } else {
                                            break;
                                        }
                                    }
                                    // Calculate GOT offset
                                    ins_addr = pc_value + offset_got + base_addr;
                                    //println!("{:x}", ins_addr);

                                    for (name, (addr, size)) in func_table.iter() {
                                        if *addr + base_addr == ins_addr {
                                            operands = format!("{}0x{:x} {}-> {}{}", color::Fg(color::Blue), *addr + base_addr,
                                                color::Fg(color::White), color::Fg(color::Red), name.as_str());
                                            resolved = true;
                                            break;
                                        }
                                    }
                                },
                                Err(_e) => {},
                            };
                        }
                    },
                    Err(_e) => {
                        color2 = Box::new(color::Fg(color::White));
                    },
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

        // Highlight if instruction pointer points to this address
        if address == ip_value {
            print!("{}", color::Bg(color::Yellow));
            color1 = Box::new(color::Fg(color::Black));
            color2 = Box::new(color::Fg(color::Black));
        }
        // 64bit address padding
        if is64 {
            print!("{}{:#018x}    {}{} {}{}", color::Fg(color::Blue), address,
                color1, opcode,
                color2, operands);
        // 32bit address padding
        } else {
            print!("{}{:#010x}    {}{} {}{}", color::Fg(color::Blue), address,
                color1, opcode,
                color2, operands);
        }
        println!("{}", color::Bg(color::Reset));
    }
    println!();
}

pub fn print_nearby_instructions(ip_val: usize, buff: &Vec<u8>, cap_obj: &Capstone) {

    /*
    let mut base_addr: u64 = match is64 {
        true => 0x555555554000,
        false => 0x56555000,
    };
    */
    // TODO add 32bit support
    // TODO read from memory instead of binary
    let base_addr = 0x555555554000;

    let mut start: usize = ip_val - base_addr;
    let mut end: usize = (ip_val - base_addr) + 16;

    let asm_bytes = &buff[start..end];

    // Interpret bytes with Capstone
    let insns = cap_obj.disasm_count(asm_bytes, ip_val as u64, 3)
            .expect("Failed to disassemble");

    let mut address: u64 = 0;
    let mut opcode: &str = "???";
    let mut op: &str = "";
    let mut operands: String = "".to_owned();   // modified, printable operands

    for i in insns.as_ref() {

        address = i.address();
        opcode = i.mnemonic().unwrap();
        op = i.op_str().unwrap();
        operands = op.to_owned();

        // Highlight if instruction pointer points to this address
        if address == ip_val as u64 {
            print!("{}", color::Bg(color::Yellow));
        }
        print!("{}{} {}", color::Fg(color::Black), opcode, operands);
        println!("{}", color::Bg(color::Reset));
    }
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
                    //
                    // DT_JMPREL -> Address of relocation entries associated solely with the PLT
                    // -> Dyn { d_tag: "DT_JMPREL" d_val: 0x690 }
                    //
                    // .plt is of type SHT_PROGBITS
                    //
                    // DT_PLTRELSZ -> Size in bytes of PLT relocation entries
                    // DT_PLTGOT -> Address of PLT and/or GOT
                    //
                    //
                    // In [2]: hex(0x0000000000003fb8 - 0x1080)
                    // Out[2]: '0x2f38'
                    // 
                    // In [3]: hex(0x0000000000003fc8 - 0x10a0)
                    // Out[3]: '0x2f28'
                    // 
                    // In [4]: hex(0x0000000000003fc0 - 0x1090)
                    // Out[4]: '0x2f30'
                    //
                    // hexdump only finds hardcoded address 0x3fa0 (.got.plt offset)

                    /*
                    match &elf.dynamic {
                        Some(val) => { println!("{:?}", val); },
                        None => { }
                    };
                    */

                    // resolve functions in .got.plt
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
