#![allow(unused)]

mod static_info;
mod dynamic_info;
mod ptrace;
mod program;
mod header_info;
mod process_info;
mod terminal;

extern crate termion;       // for colors, style
extern crate libc;
extern crate capstone;

use capstone::prelude::*;
use libc::{WEXITSTATUS, WIFEXITED, WIFSIGNALED, WTERMSIG};
use program::Program;
use termion::{color, style};
use termion::clear::All;
use termion::cursor::Goto;
use std;
use std::{env, io, process, str::Split, ffi::CString};



fn get_input() -> String {
    /*
     * Gets user input
     */

    let mut user_input = String::new();
    io::stdin()
        .read_line(&mut user_input)
        .expect("[ERROR] failed to read input");

    return user_input.trim().to_string();
}

fn run_config(program_exec: &String, program_args: Vec<&str>) {
    let program_pid: libc::pid_t;
    let mut arg_values: Vec<i8> = Vec::new();
    let mut args_ptr: Vec<*const i8> = Vec::new();
    let mut args_exist: bool = false;

    // Turn args into i8 array separated by nullbytes
    for i in 0..program_args.len() {
        let cs = CString::new(program_args[i]).unwrap();
        let cv: Vec<u8> = cs.into_bytes_with_nul();
        let mut tmp: Vec<i8> = cv.into_iter().map(|c| c as i8).collect::<_>();
        arg_values.append(&mut tmp);
    }

    // Put pointers for arguments in arts_ptr
    let mut arg_first_char: bool = true;
    for a in &arg_values {
        if arg_first_char {
            args_ptr.push(a);
            arg_first_char = false;
        }
        if *a == 0 {
            arg_first_char = true;
        }
    }
    // Terminate reading more arguments (argv stops here)
    args_ptr.push(std::ptr::null());

    unsafe {
        program_pid = libc::fork();
    }

    // Create new instance and and arguments if exist
    let mut program: Program = Program::new(program_pid,
                                            program_exec);
    program.add_args(args_ptr);

    if program_pid < 0 {
        panic!("Fork failed");
    }
    else if program_pid == 0 {  // if child
        println!("Running {}", program_exec);
        program.run();
        return;
    }
    else {
        println!("debugger attaching to pid {}", program_pid);
        let status: i32 = program.wait();

        // Check the exit code
        if WIFEXITED(status) {
            let x: i32 = WEXITSTATUS(status);
            println!("Program exited with code: {}", x);
        }
        // Or if abnormal exit e.g. segfault
        if WIFSIGNALED(status) {
            println!("Program ended with kill: {}", WTERMSIG(status));
        }
    }
}

fn main() {

    // TODO make this prettier
    // Read args and set filename
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("Please specify a filename");
        process::exit(1);
    }
    // Reading file into buffer
    let filename = args[1].clone();
    let buffer = static_info::load_file(filename.clone());

    // Parsing file as an object
    // Reference to the file_object is further passed to functions
    let file_object = static_info::parse_file(&buffer);

    // Create Capstone object
    let capstone_obj = Capstone::new()
        .x86()
        .mode(arch::x86::ArchMode::Mode64)
        .syntax(arch::x86::ArchSyntax::Intel)
        .detail(true)
        .build()
        .expect("Failed to create Capstone object");

    let mut running: bool = true;
    let mut prev_comms: Vec<String> = Vec::new();

    let mut comm_counter: usize = 0;
    println!("Welcome to Feri Debugger. For commands and functions type 'help'.\n");

    // Main loop
    while running {
        terminal::print_prompt();
        let input = terminal::key_commands(&mut prev_comms,&mut comm_counter);
        let mut spliterator: Split<char> = input.as_str().split(' '); // Iterator through arguments

        match spliterator.next() {
            Some(arg) => match arg {
                "help" | "h" => print_help(),
                "run" | "r" => {
                        let mut vec_program_args: Vec<&str> = Vec::new();
                        // Add filename to argv[0]
                        let fname = &filename.to_string();
                        vec_program_args.push(fname);
                        while let Some(program_args) = spliterator.next() {
                            vec_program_args.push(program_args);
                        }
                        run_config(&filename, vec_program_args);
                },
                "del" => {
                    if let Some("break") = spliterator.next() {
                        if let Some(num) = spliterator.next() {
                            println!("del break {}", num); // del_break_single(num);
                        }
                        else {
                            /* delete_break_all(); */    // Tu me popravite če ni tak mišljeno
                        }
                    }
                    else { println!("Specify what to delete: del <break> [n]"); }
                },
                "list" | "lb" | "lf" => {
                    if arg == "lb" {
                        println!("list break"); /* list_break(); */
                    }
                    else if arg == "lf" {
                        static_info::list_func(&file_object);
                    }
                    else if let Some(second) = spliterator.next() {
                        match second {
                            "break" => {
                                println!("list break");
                                /* list_break(); */
                            },
                            "func" => {
                                static_info::list_func(&file_object);
                            },
                            _ => println!("Please choose between <break/func>")
                        }
                    }
                    else { println!("Specify what to list: list <break/func>"); }
                },
                "continue" | "c" => println!("continue"),
                "step" | "s" => println!("step"),
                "disas" | "d" => {
                    if let Some(func) = spliterator.next() {
                        static_info::disassemble(func, &file_object, &buffer, &capstone_obj);
                        //println!("dissasemble {} ", func.to_string());
                    }
                    else{
                        println!("not enough arguments type 'help' for help");
                    }
                },
                "break" | "b" => {
                    if let Some(address) = spliterator.next(){
                        println!("break at adress {} ", address);
                    }
                    else{
                        println!("not enough arguments type 'help' for help");
                    }
                },
                "on" => {
                    if let Some(num) = spliterator.next(){
                        println!("enable breakpoint on: {}", num);
                    }
                    else{
                        println!("not enough arguments type 'help' for help");
                    }
                },
                "off" => {
                    if let Some(num) =spliterator.next(){
                        println!("disable breakpoint on: {}", num);
                    }
                    else{
                        println!("not enough arguments type 'help' for help");
                    }
                },
                "reg" => {
                    if let Some(name) = spliterator.next(){
                        println!("values in all registers");
                    }
                    else{
                        println!("not enough arguments type 'help' for help");
                    }
                },
                "set" => {
                    if let Some("reg")= spliterator.next(){
                        if let Some(name) = spliterator.next(){
                            if let Some(num) = spliterator.next(){
                                println!("set register {} to {}", name, num);
                            }
                            else{
                                println!("not enough arguments type 'help' for help");
                            }
                        }
                    }
                },
                "mem" => {
                    if let Some(name) = spliterator.next(){
                        if let Some(byte_num) = spliterator.next(){
                            println!("dump {} bytes starting with {} ", byte_num, name);
                        }
                        else{
                            println!("not enough arguments type 'help' ")
                        }
                    }
                },
                "stack" => println!("dump memory from current stack"),
                "info" => {
                    if let Some(topic) = spliterator.next() {
                        match topic {
                            "header" => header_info::header_info(&buffer),
                            // run_config needs to return the whole program object or the pid_t
                            // "process" => process_info::process_info(pid),
                            _ => println!("not enouhg argumets type 'help' "),
                        }
                    }
                    else {
                        println!("not enouhg argumets type 'help' ");
                    }
                },
                "quit" | "q" => running = false,
                "clear" =>{
                    println!("{}",termion::clear::All);
                    print!("{}",termion::cursor::Goto(1,1));
                    },
                _ => println!("This command does not exist. Type 'help' for commands and functions."),
            },
            None => todo!(),
        }
    }
}


fn print_help() {
    let help_str: &str = "FERI debugger

usage: fdb <input file>

optional arguments:
    -h                      display help

debugger commands:

    help                    print help for all commands
    run / r [arg1, arg2...] run the program with arguments
    continue / c            continue execution
    step / s                step one instruction

    d / disas [label]       disassemble function
    lf / list func          list all functions

    b / break [address]     set breakpoint at given address
    list break / lb         list all breakpoints
    del break [n]           delete breakpoint number [n]
    [n] on/off              enable/disable breakpoint number [n]

    reg                     print values in all registers
    reg [name]              print value in [name] register
    set reg [name] [value]  set register [name] to value [value]

    mem [address] [n]       dump memory, [n] bytes starting from [address]
    stack                   dump memory from current stack frame

    info <header, process>  print information
    clear                   clears terminal
";

    println!("{}\n", help_str);
}
