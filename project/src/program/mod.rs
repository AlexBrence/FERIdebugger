extern crate libc;
extern crate backtrace;

use std;
use gimli::{UninitializedUnwindContext, UnwindTable};
use std::env::current_exe;
use std::ffi::CString;
use libc::{WEXITED, c_int, backtrace, c_void, c_char};
use super::ptrace;
use termion::{color, style};
use capstone::prelude::*;
use goblin::Object;
use super::static_info;



#[derive(Clone)]
pub struct Breakpoint {
    addr: u64,
    orig_byte: u8,
    enabled: bool,
}

pub struct Program {
    pub pid: i32,
    pub executable: String,
    pub args: Vec<*const i8>,
    pub breakpoints: Vec<Breakpoint>,
}


impl Program {

    // Constructor
    pub fn new(program_pid: i32, program_exec: &String) -> Program {
        Program {
            pid: program_pid,
            executable: (*program_exec).clone(),
            args: Vec::new(),
            breakpoints: Vec::new(),
        }
    }

    // Methods
    pub fn add_args(&mut self, program_args: Vec<*const i8>) {
        // Prepare the argv

        self.args.clear();
        self.args = program_args;
    }

    pub fn run(&mut self) {
        unsafe {
            ptrace::trace_me();

            // Create a CString version of the program
            let cprogram_str = CString::new((self.executable).clone()).unwrap();
            let cprogram = cprogram_str.as_ptr();

            // Start the executable
            let ret = libc::execv(cprogram, self.args.as_ptr());
            println!("Return was ............. {:?}", ret);

            // If failed to run
            println!("[ERROR] Failed to run, exited with err {:?} and errno {}", ret, *libc::__errno_location());
        }
    }


    pub fn wait(&mut self) -> i32 {
        let mut status: i32 = 0;

        unsafe {
            // wait for the next ptrace induced block
            libc::waitpid(-1, &mut status, 0);
        }

        return status;
    }


    pub fn peek_byte_at(&mut self, location: u64) -> u8 {
        let loc = (location / 8) * 8;
        let offset = location % 8;
        let word: Result<u64, i32> = ptrace::peek_text(self.pid, loc);
        match word {
            Ok(w) => return ((w & (0xff << (8 * offset))) >> (8 * offset)) as u8,
            Err(e) =>
                panic!("Error: failed to read byte at {:016x} errno: {}", loc, e),
        }
    }

    pub fn poke_byte_at(&mut self, location: u64, data: u8) {
        let loc = (location / 8) * 8;
        let offset = location % 8;
        let mut word: u64 = ptrace::peek_text(self.pid, loc)
            .ok()
            .expect("OOPS");
        word = (word & !(0xff << (8 * offset))) | ((data as u64) << (8 * offset));
        ptrace::poke_text(self.pid, loc, word);
    }

    pub fn get_user_struct(&mut self) -> libc::user {
        unsafe {
            let mut user_struct: libc::user = std::mem::uninitialized();
            ptrace::get_user(self.pid, &mut user_struct);
            return user_struct;
        }
    }

    pub fn write_user_struct(&mut self, usr: libc::user) {
        ptrace::write_user(self.pid, &usr);
    }

    pub fn list_breakpoints(&mut self) {
        if self.breakpoints.len() == 0 {
            println!("{c1}No breakpoints set yet", c1=color::Fg(color::Red));
            return;
        }
        for i in 0..self.breakpoints.len() {
            let bp: Breakpoint = self.breakpoints[i].clone();

            println!("{c1}Breakpoint {} at {c2}0x{:016x} : {}", i, bp.addr, bp.enabled, c1=color::Fg(color::LightMagenta), c2=color::Fg(color::Yellow));
        }
    }

    pub fn set_breakpoint(&mut self, loc: u64) {
        // Check if breakpoint already exists
        let mut index = self.breakpoints.iter().position(|i| i.addr == loc);
        if index.is_some() {
            println!("Breakpoint is already set.");
            return;
        }

        let orig_byte: u8 = self.peek_byte_at(loc);

        self.poke_byte_at(loc, 0xCC);

        self.breakpoints.push(Breakpoint {
                addr: loc,
                orig_byte: orig_byte,
                enabled: true,
        });

        println!("Breakpoint set at 0x{:016x}!", loc);
    }

    pub fn delete_breakpoint(&mut self, no: u64) {
        let index = no as usize;
        if self.breakpoints.len() < index {
            println!("Breakpoint with that address doesn't exist.");
        }
        else {
            let mut orig_byte: u8 = self.breakpoints[index].orig_byte;
            let mut addr = self.breakpoints[index].addr;
            self.poke_byte_at(addr, orig_byte);
            self.breakpoints.remove(index);
        }

        // match index {
        //     Some(i) => {
        //         let mut orig_byte: u8 = self.breakpoints[i].orig_byte;
        //         let mut addr = self.breakpoints[i].loc;
        //         self.poke_byte_at(addr, orig_byte);
        //         self.breakpoints.remove(i);
        //     },
        //     None => println!("Breakpoint with that address doesn't exist."),
        // }
    }

    pub fn enable_breakpoint(&mut self, no: u64){
        let index = no as usize;

        if self.breakpoints.len() < index {
            println!("Breakpoint with that address DOESN'T EXIST.");
        }
        else {
            if !self.breakpoints[index].enabled {
                let mut addr = self.breakpoints[index].addr;
                self.poke_byte_at(addr, 0xCC);
                self.breakpoints[index].enabled = true;
                println!("Breakpoint is enabled.");
            }
            else {
                println!("Breakpoint with that address is ALREADY ENABLED.");
            }
        }
    }

    pub fn disable_breakpoint(&mut self, no: u64){
        let index = no as usize;

        if self.breakpoints.len() < index {
            println!("Breakpoint with that address DOESN'T EXIST.");
        }
        else {
            if self.breakpoints[index].enabled {
                let mut orig_byte: u8 = self.breakpoints[index].orig_byte;
                let mut addr = self.breakpoints[index].addr;
                self.poke_byte_at(addr, orig_byte);
                self.breakpoints[index].enabled = false;
                println!("Breakpoint is disabled.");
            }
            else {
                println!("Breakpoint with that address is ALREADY DISABLED.");
            }
        }
    }

    pub fn handle_breakpoint(&mut self) {
        let mut user: libc::user = self.get_user_struct();
        let rip: u64 = user.regs.rip - 1;

        for i in 0..self.breakpoints.len() {
            let bp = self.breakpoints[i].clone();

            if bp.addr == rip {
                self.poke_byte_at(bp.addr, bp.orig_byte);

                user.regs.rip = rip;
                self.write_user_struct(user);
                return;
            }
        }

        panic!("oops");
    }

    pub fn singlestep(&mut self) {
        ptrace::singlestep(self.pid);
    }

    // 'continue' is a keyword in rust and can't be used here
    pub fn resume(&mut self) {
        ptrace::resume(self.pid);
    }

    pub fn remake_breakpoints(&mut self) {
        for i in 0..self.breakpoints.len() {
            if self.breakpoints[i].enabled {
                let addr = self.breakpoints[i].addr;
                self.poke_byte_at(addr, 0xCC);
            }
        }
    }

    // set registers
    pub fn set_reg(&mut self, register: &str, value: u64) {
        let mut user: libc::user = self.get_user_struct();
        let mut regs = user.regs;

        match register {
            "rax" => regs.rax = value,
            "rbx" => regs.rbx = value,
            "rcx" => regs.rcx = value,
            "rdx" => regs.rdx = value,
            "r15" => regs.r15 = value,
            "r14" => regs.r14 = value,
            "r13" => regs.r13 = value,
            "r12" => regs.r12 = value,
            "r11" => regs.r11 = value,
            "r10" => regs.r10 = value,
            "r9" => regs.r9 = value,
            "r8" => regs.r8 = value,
            "rsp" => regs.rsp = value,
            "rbp" => regs.rbp = value,
            "rsi" => regs.rsi = value,
            "rdi" => regs.rdi = value,
            "rip" => regs.rip = value,
            "eflags" => regs.eflags = value,
            "cs" => regs.cs = value,
            "ss" => regs.ss = value,
            "ds" => regs.ds = value,
            "es" => regs.es = value,
            // ADD ERROR HANDLING
            _ => {},
        }

        // save changes to registers
        user.regs = regs;
        self.write_user_struct(user);
    }

    pub fn stop(&mut self) {
        ptrace::stop(self.pid);
    }


    pub fn read_words(&self, from: usize, size: usize) -> Option<Vec<u64>> {
        let mut words = Vec::with_capacity(size);
        let wordlen = std::mem::size_of::<usize>();
        for i in 0..size {
            words.push(ptrace::peek_text(self.pid, (from + wordlen * i) as u64).unwrap());
        }
        Some(words)
    }

    pub fn fetch_state(&mut self, obj: &Object, buff: &Vec<u8>, cap_obj: &Capstone) -> Result<(), ()> {
        let mut is64: bool = true;
        let func_table = static_info::get_func_table(obj, &mut is64);

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

        let registers = self.get_user_struct().regs;
        let mut start: usize = 0;
        let mut end: usize = 0;
        let asm_bytes = &buff[start..end];
        let insns = cap_obj.disasm_all(asm_bytes, registers.rip)
            .expect("Failed to disassemble");
        println!("instruction: {}", insns);

        let mut opcode: &str = "???";
        let mut op: &str = "";
        let mut operands: String = "".to_owned();

        for i in insns.as_ref() {
            opcode = i.mnemonic().unwrap();
            op = i.op_str().unwrap();
            operands = op.to_owned();
            println!("**** OPCODE: {}\nOperands: {}", opcode, operands);

            match opcode {
                "endbr64" => {
                    println!("No stack.");
                    return Ok(())
                },
                "push" => {
                    if operands.as_str() == "rbp" || operands.as_str() == "rbx" {
                        println!("No stack.");
                        return Ok(())
                    };
                },
                "mov" => {
                    if operands.as_str() == "rbp, rsp" {
                        println!("No stack.");
                        return Ok(())
                    }
                },
                "sub" => {
                    if operands.contains("rsp") {
                        println!("No stack.");
                        return Ok(())
                    }
                },
                _ => {}
            }
        }


        let size: u64 = registers.rbp - registers.rsp;
        println!("rsp: {}\nrbp: {}", registers.rbp, registers.rsp);
        let stack = self.read_words(registers.rsp as usize, size as usize).unwrap();
        for s in &stack {
            println!("0x{:016x}", s);
        }
        // self.handle_breakpoint();

        Ok(())
    }

}
