#![allow(unused)]

mod static_info;
mod dynamic_info;
mod ptrace;
mod program;
mod header_info;
mod process_info;
mod terminal;
mod registers;

extern crate termion;       // for colors, style
extern crate libc;
extern crate capstone;

use capstone::prelude::*;
use libc::{WEXITSTATUS, WIFEXITED, WIFSIGNALED, WTERMSIG, backtrace};
use program::Program;
use sysinfo::ProcessExt;
use termion::{color, style};
use std::{self, env, ffi::CString, io, io::Write, os::unix::prelude::OsStringExt, process,
          process::{Command, ExitStatus, Output, Stdio}, str::{Split,from_utf8}, thread, 
          fmt};


fn get_input() -> String {
    let mut user_input = String::new();
    io::stdin()
        .read_line(&mut user_input)
        .expect("[ERROR] failed to read input");

    return user_input.trim().to_string();
}

fn check_for_bash_command(input: &String) -> Option<String> {
    let mut is_found: Option<usize>; 
    let mut is_found_reverse: Option<usize>; 

    // Check for $(<command>) format
    is_found = input.find("$(");
    is_found_reverse = input.rfind(")");

    if is_found != None {
        match is_found {
            Some(p) => {
                match is_found_reverse {
                    Some(rp) => {
                        // let command = &input[p + 2..rp];
                        Some(input[p + 2..rp].to_string())
                    },
                    // If closing paranthesis not found
                    None => {
                        println!("Invalid command.");
                        None
                    },
                }
            },
            None => todo!()
        }
    }
    else {
        // Check for `<command>` format
        is_found = input.find("`");
        is_found_reverse = input.rfind("`");

        match is_found {
            Some(p) => {
                match is_found_reverse {
                    Some(rp) => {
                        // let command = &input[p + 2..rp];
                        Some(input[p + 1..rp].to_string())
                    },
                    None => {
                        println!("Invalid command.");
                        None
                    }
                }
            },
            None => None
        }
    }
}

fn get_bash_command_output(mut bash_command_vec: &Vec<&str>) -> Result<Vec<String>, String> { 
    // Get only args from original vector
    let mut args_vec: Vec<&str> = bash_command_vec[1..].to_vec();
    let mut output: Output;

    // Execute 
    let mut executed_command = Command::new(bash_command_vec[0])
                                        .args(args_vec.into_iter())
                                        .output();


    // Check if everything Ok
    let output = match executed_command {
        Ok(o) => o,
        Err(e) => return Err(format!("Could not execute the bash command.\n{}\n", e)),
    };

    // Convert u8 to String, return vector to append later on in main
    if output.status.success() {
        let to_utf8 = String::from_utf8_lossy(&output.stdout[..output.stdout.len() - 1]);
        let utf8_to_vec: Vec<&str> = to_utf8.split(' ').collect::<Vec<&str>>();
        let string_vec = utf8_to_vec.iter().map(|x| x.to_string()).collect();
        Ok(string_vec)
    }
    else {
        // Print out bash error
        let err = String::from_utf8_lossy(&output.stderr);
        return Err(String::from(format!("Bash command was not executed.\n{}", err)));
    }

}



fn run_config(program_exec: &String, program_args: Vec<String>) -> Program {
    let program_pid: libc::pid_t;
    let mut arg_values: Vec<i8> = Vec::new();
    let mut args_ptr: Vec<*const i8> = Vec::new();
    let mut args_exist: bool = false;

    // Turn args into i8 array separated by nullbytes
    for i in 0..program_args.len() {
        let cs = CString::new(program_args[i].clone()).unwrap();
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
        // return;
    }
    else {
        println!("debugger attaching to pid {}", program_pid);
        let status: i32 = program.wait();

        // Check the exit code
        if WIFEXITED(status) {
            let x: i32 = WEXITSTATUS(status);
            println!("Program exited with code: {}\n", x);
        }
        // Or if abnormal exit e.g. segfault
        if WIFSIGNALED(status) {
            println!("Program ended with signal: {}\n", WTERMSIG(status));
        }
    }

    return program;
}

fn print_registers(mut program :&mut Program){
    let prog = program.get_user_struct().regs.rip;
    let sp = "     ";
    let sp_s = "    ";
    let name_color = color::Fg(color::Yellow);
    let number_color = color::Fg(color::Cyan);
    println!("
    {}rax: {}{}0x{:016x}  {}{}
    {}rbx: {}{}0x{:016x}  {}{}
    {}rcx: {}{}0x{:016x}  {}{}
    {}rdx: {}{}0x{:016x}  {}{}
    {}rsi: {}{}0x{:016x}  {}{}
    {}rdi: {}{}0x{:016x}  {}{}
    {}rbp: {}{}0x{:016x}  {}{}
    {}rsp: {}{}0x{:016x}  {}{}
    {}r8: {}{}0x{:016x}  {}{}
    {}r9: {}{}0x{:016x}  {}{}
    {}r10: {}{}0x{:016x}  {}{}
    {}r11: {}{}0x{:016x}  {}{}
    {}r12: {}{}0x{:016x}  {}{}
    {}r13: {}{}0x{:016x}  {}{}
    {}r14: {}{}0x{:016x}  {}{}
    {}r15: {}{}0x{:016x}  {}{}
    {}rip: {}{}0x{:016x}  {}{}
    {}eflags: {}{}0x{:016x}  {}{}
    {}cs: {}{}0x{:016x}  {}{}
    {}ss: {}{}0x{:016x}  {}{}
    {}ds: {}{}0x{:016x}  {}{}
    {}es: {}{}0x{:016x}  {}{}
    {}fs: {}{}0x{:016x}  {}{}
    {}gs: {}{}0x{:016x}  {}{}",
    name_color,sp_s,number_color, program.get_user_struct().regs.rax,number_color,program.get_user_struct().regs.rax,
    name_color,sp_s,number_color,program.get_user_struct().regs.rbx,number_color,program.get_user_struct().regs.rbx,
    name_color,sp_s,number_color,program.get_user_struct().regs.rcx,number_color,program.get_user_struct().regs.rcx,
    name_color,sp_s,number_color,program.get_user_struct().regs.rdx,number_color,program.get_user_struct().regs.rdx,
    name_color,sp_s,number_color,program.get_user_struct().regs.rsi,number_color,program.get_user_struct().regs.rsi,
    name_color,sp_s,number_color,program.get_user_struct().regs.rdi,number_color,program.get_user_struct().regs.rdi,
    name_color,sp_s,number_color,program.get_user_struct().regs.rbp,number_color,program.get_user_struct().regs.rbp,
    name_color,sp_s,number_color,program.get_user_struct().regs.rsp,number_color,program.get_user_struct().regs.rsp,
    name_color,sp,number_color,program.get_user_struct().regs.r8,number_color,program.get_user_struct().regs.r8,
    name_color,sp,number_color,program.get_user_struct().regs.r9,number_color,program.get_user_struct().regs.r9,
    name_color,sp_s,number_color,program.get_user_struct().regs.r10,number_color,program.get_user_struct().regs.r10,
    name_color,sp_s,number_color,program.get_user_struct().regs.r11,number_color,program.get_user_struct().regs.r11,
    name_color,sp_s,number_color,program.get_user_struct().regs.r12,number_color,program.get_user_struct().regs.r12,
    name_color,sp_s,number_color,program.get_user_struct().regs.r13,number_color,program.get_user_struct().regs.r13,
    name_color,sp_s,number_color,program.get_user_struct().regs.r14,number_color,program.get_user_struct().regs.r14,
    name_color,sp_s,number_color,program.get_user_struct().regs.r15,number_color,program.get_user_struct().regs.r15,
    name_color,sp_s,number_color,program.get_user_struct().regs.rip,number_color,program.get_user_struct().regs.rip,
    name_color," ",number_color,program.get_user_struct().regs.eflags,number_color,program.get_user_struct().regs.eflags,
    name_color,sp,number_color,program.get_user_struct().regs.cs,number_color,program.get_user_struct().regs.cs,
    name_color,sp,number_color,program.get_user_struct().regs.ss,number_color,program.get_user_struct().regs.ss,
    name_color,sp,number_color,program.get_user_struct().regs.ds,number_color,program.get_user_struct().regs.ds,
    name_color,sp,number_color,program.get_user_struct().regs.es,number_color,program.get_user_struct().regs.es,
    name_color,sp,number_color,program.get_user_struct().regs.fs,number_color,program.get_user_struct().regs.fs,
    name_color,sp,number_color,program.get_user_struct().regs.gs,number_color,program.get_user_struct().regs.gs);
}
fn print_spec_register(mut program:&mut Program,ime: String){
    let name_color = color::Fg(color::Yellow);
    let number_color = color::Fg(color::Cyan);
    match &ime as &str{
        "rax" =>{
            println!("{}rax: {}0x{:016x}{}  {}",name_color,number_color,program.get_user_struct().regs.rax,number_color,program.get_user_struct().regs.rax);
            //println!("{}rax:{}   {}",pr{}ogram.get_user_struct().regs.rax);
        },
        "rbx" => {
            println!("{}rbx:{} 0x{:016x}{}  {}",name_color,number_color,program.get_user_struct().regs.rbx,number_color,program.get_user_struct().regs.rbx);
        },
        "rcx" => {
            println!("{}rcx:{} 0x{:016x}{}  {}",name_color,number_color,program.get_user_struct().regs.rcx,number_color,program.get_user_struct().regs.rcx);
        },
        "rdx" => {
            println!("{}rdx:{} 0x{:016x}{}  {}",name_color,number_color, program.get_user_struct().regs.rdx,number_color,program.get_user_struct().regs.rdx);
        },
        "rsi" => {
            println!("{}rsi:{} 0x{:016x}{}  {}",name_color,number_color,program.get_user_struct().regs.rsi,number_color,program.get_user_struct().regs.rsi);
        },
        "rdi" => {
            println!("{}rdi:{} 0x{:016x}{}  {}",name_color,number_color,program.get_user_struct().regs.rdi,number_color,program.get_user_struct().regs.rdi);
        },
        "rbp" => {
            println!("{}rbp:{} 0x{:016x}{}  {}",name_color,number_color,program.get_user_struct().regs.rbp,number_color,program.get_user_struct().regs.rbp);
        },
        "rsp" => {
            println!("{}rsp:{} 0x{:016x}{}  {}",name_color,number_color,program.get_user_struct().regs.rsp,number_color,program.get_user_struct().regs.rsp);
        },
        "r8" => {
            println!("{}r8: {}0x{:016x}{}  {}",name_color,number_color,program.get_user_struct().regs.r8,number_color,program.get_user_struct().regs.r8);
        },
        "r9" => {
            println!("{}r9: {}0x{:016x}{}  {}",name_color,number_color,program.get_user_struct().regs.r9,number_color,program.get_user_struct().regs.r9);
        },
        "r10" => {
            println!("{}r10:{} 0x{:016x}{}  {}",name_color,number_color,program.get_user_struct().regs.r10,number_color,program.get_user_struct().regs.r10);
        },
        "r11" => {
            println!("{}r11:{} 0x{:016x}{}  {}",name_color,number_color,program.get_user_struct().regs.r11,number_color,program.get_user_struct().regs.r11);
        },
        "r12" => {
            println!("{}r12:{} 0x{:016x}{}  {}",name_color,number_color,program.get_user_struct().regs.r12,number_color,program.get_user_struct().regs.r12);
        },
        "r13" => {
            println!("{}r13:{} 0x{:016x}{}  {}",name_color,number_color,program.get_user_struct().regs.r13,number_color,program.get_user_struct().regs.r13);
        },
        "r14" => {
            println!("{}r14:{} 0x{:016x}{}  {}",name_color,number_color,program.get_user_struct().regs.r14,number_color,program.get_user_struct().regs.r14);
        },
        "r15" => {
            println!("{}r15:{} 0x{:016x}{}  {}",name_color,number_color,program.get_user_struct().regs.r15,number_color,program.get_user_struct().regs.r15);
        },
        "rip" => {
            println!("{}rip:{} 0x{:016x}{}  {}",name_color,number_color,program.get_user_struct().regs.rip,number_color,program.get_user_struct().regs.rip);
        },
        "eflags" => {
            println!("{}eflags: {}0x{:016x}{}  {}",name_color,number_color,program.get_user_struct().regs.eflags,number_color,program.get_user_struct().regs.eflags);
        },
        "cs" => {
            println!("{}cs: {}0x{:016x}{}  {}",name_color,number_color,program.get_user_struct().regs.cs,number_color,program.get_user_struct().regs.cs);
        },
        "ss" => {
            println!("{}ss: {}0x{:016x}{}  {}",name_color,number_color,program.get_user_struct().regs.ss,number_color,program.get_user_struct().regs.ss);
        },
        "ds" => {
            println!("{}ds: {}0x{:016x}{}  {}",name_color,number_color,program.get_user_struct().regs.ds,number_color,program.get_user_struct().regs.ds);
        },
        "es" => {
            println!("{}es: {}0x{:016x}{}  {}",name_color,number_color,program.get_user_struct().regs.es,number_color,program.get_user_struct().regs.es);
        },
        "fs" => {
            println!("{}fs: {}0x{:016x}{}  {}",name_color,number_color,program.get_user_struct().regs.fs,number_color,program.get_user_struct().regs.fs);
        },
        "gs" => {
            println!("{}gs: {}0x{:016x}{}  {}",name_color,number_color,program.get_user_struct().regs.gs,number_color,program.get_user_struct().regs.gs);
        },
        _ => {
            println!("no register under that name lives here");
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

    // Program struct needed for breakpoints and process information
    // Since the variable needs to be initialized we feed it random data
    // Else, the whole main loop should be rewritten
    let mut program: Program = Program::new(1234, &"".to_string());
    // let mut program: Program = Program::new();   // THIS WOULD BE OPTIMAL

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
    println!("Welcome to Feri Debugger. For commands and functions type 'help'.\n");

    // Main loop
    while running {
        terminal::print_prompt();
        let input = terminal::key_commands(&mut prev_comms);
        println!();
 
        let mut spliterator: Split<char> = input.as_str().split(' '); // Iterator through arguments

        // Filter out bash commands if they exist
        let mut bash_command: String = String::new();
        let mut bash_command_vec: Vec<&str> = Vec::new();

        let contains_bash_command = match check_for_bash_command(&input) {
            Some(c) => {
                bash_command = c;
                bash_command_vec = bash_command.split(' ').collect();
                true
            },
            None => {
                false
            }
        };

        let mut spliterator: Split<char> = input.as_str().split(' '); 
        match spliterator.next() {
            Some(arg) => match arg {
                "help" | "h" => print_help(),
                "run" | "r" => {
                        let mut vec_program_args: Vec<String> = Vec::new();

                        // Add filename to argv[0]
                        let fname = &filename.to_string();
                        vec_program_args.push(fname.clone());

                        // Check if bash command is used for args input
                        if contains_bash_command {
                            let mut tmp_vec = get_bash_command_output(&mut bash_command_vec);
                            match tmp_vec {
                                Ok(mut o) => {
                                    vec_program_args.append(&mut o);
                                },
                                Err(e) => {
                                    println!("{}[ERROR]{} {}", color::Fg(color::Red), style::Reset, e);
                                    continue;
                                }
                            }
                        }
                        else {
                            while let Some(program_args) = spliterator.next() {
                                vec_program_args.push(program_args.to_string());
                            }
                        }
                        program = run_config(&filename, vec_program_args);
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
                        program.list_breakpoints();
                    }
                    else if arg == "lf" {
                        static_info::list_func(&file_object);
                    }
                    else if let Some(second) = spliterator.next() {
                        match second {
                            "break" => {
                                program.list_breakpoints();
                            },
                            "func" => {
                                static_info::list_func(&file_object);
                            },
                            _ => println!("Please choose between <break/func>")
                        }
                    }
                    else { println!("Specify what to list: list <break/func>"); }
                },
                "continue" | "c" => program.resume(),
                "step" | "s" => program.singlestep(),
                "disas" | "d" => {
                    if let Some(func) = spliterator.next(){
                        //println!("dissasemble {} ", func.to_string());
                        static_info::disassemble(func, &file_object, &buffer, &capstone_obj);
                    }
                    else{
                        println!("not enough arguments type 'help' for help");
                    }
                },
                "break" | "b" => {
                    if let Some(address) = spliterator.next(){
                        // TODO: add so the address argument can have '0x' prefix
                        let addr: u64 = u64::from_str_radix(&address, 16).unwrap();
                        program.set_breakpoint(addr);
                        println!("Breakpoint set at 0x{:016x}!", addr);
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
                        print_spec_register(&mut program, name.to_string());
                    }
                    else{
                        print_registers(&mut program);
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
                "bt" => println!("List frames"), // use libc backtrace
                "info" => {
                    if let Some(topic) = spliterator.next() {
                        match topic {
                            "header" => header_info::header_info(&buffer),
                            "process" => process_info::process_info(program.pid),
                            _ => println!("not enouhg argumets type 'help' "),
                        }
                    }
                    else {
                        println!("not enough argumets type 'help' ");
                    }
                },
                "quit" | "q" => {
                    running = false;
                    println!("")},
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
";

    println!("{}\n", help_str);
}
