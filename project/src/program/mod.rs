extern crate libc;
use std;
use std::ffi::{CString};
use libc::c_int;
use super::ptrace;


pub struct Program {
    pub pid: i32,
    pub executable: String,
    pub args: Vec<*const i8>,
}


impl Program {

    // Constructor 
    pub fn new(program_pid: i32, program_exec: &String) -> Program {
        Program {
            pid: program_pid,
            executable: (*program_exec).clone(),
            args: Vec::new()
        }
    }

    // Methods
    pub fn add_args(&mut self, program_args: Vec<*const i8>) {
        // Prepare the argv
        //self.args.push(self.executable);
        //self.args.push(std::ptr::null());

        self.args.clear();
        self.args = program_args;
    }

    pub fn run(&mut self) {
        unsafe {
            // ptrace::trace_me();

            // Create a CString version of the program
            let cprogram_str = CString::new((self.executable).clone()).unwrap();
            let cprogram = cprogram_str.as_ptr();

            // Prepare the environment
            // let mut vec_environment: Vec<*const i8> = Vec::new();
            // vec_environment.push(CString::new("HOME=~/Desktop").expect("vec_env_push").as_ptr());
            // vec_environment.push(std::ptr::null());

            //for x in &self.args {
            //    println!("{:?}", **x);
            //}
            // Start the executable
            let ret = libc::execv(cprogram, self.args.as_ptr()/* , vec_environment.as_ptr() */);
            println!("Return was ............. {:?}", ret);

            // Failed to run
            println!("[ERROR] Failed to run, exited with err {:?} and errno {}", ret, *libc::__errno_location());
        }
    }


    pub fn wait(&mut self) -> u32 {
        let mut status: i32 = 0;

        unsafe {
            // wait for the next ptrace induced block
            libc::waitpid(-1, &mut status, 0);
        }

        return status as u32;
    }
}
