extern crate libc;
use std;
use std::ffi::{CString};
use libc::{WEXITED, c_int};
use super::ptrace;
use termion::{color, style};


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
}
