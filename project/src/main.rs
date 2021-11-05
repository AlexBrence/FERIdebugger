extern crate termion;       // for colors, style
use termion::{color, style};

use std::{env, io, str::Split};


fn print_prompt() {
    /*
     * Creates nice looking hot and sexy prompt 
     */

    print!("{}{}(", color::Fg(color::Green), style::Bold);
    print!("{}fdb", style::Reset);
    print!("{}{}) {}", color::Fg(color::Green), style::Bold, style::Reset);

    io::Write::flush(&mut io::stdout())
        .expect("[ERROR] flush failed");
}


fn get_input() -> String {
    /*
     * Get's user input
     */

    let mut user_input = String::new();
    io::stdin()
        .read_line(&mut user_input)
        .expect("[ERROR] failed to read input");

    return user_input.trim().to_string();
}

fn print_argument(mut arg: Vec<&str>){
    arg = arg[1..(arg.len())].to_vec();
    for element in arg{
        println!("{}", element);
    }
}

fn main() {
    let mut running: bool = true;

    println!("Welcome to Feri Debugger. For commands and functions type 'help'.\n");

    // Main loop
    while running {
        print_prompt();
        let input = get_input();
        let mut spliterator: Split<char> = input.as_str().split(' '); // Iterator through arguments

        match spliterator.next() {
            Some(arg) => match arg {
                "help" | "h" => print_help(),
                "run" | "r" => {
                        let mut vec_program_args: Vec<&str> = Vec::new();
                        while let Some(program_args) = spliterator.next() {
                            vec_program_args.push(program_args);
                        }
                        println!("arguments were: {:?}", vec_program_args);
                        /* run_program_with_arguments(vec_program_args); */
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
                        println!("list func"); /* list_func(); */ 
                    }
                    else if let Some(second) = spliterator.next() {
                        match second {
                            "break" => {
                                println!("list break");
                                /* list_break(); */
                            },
                            "func" => {
                                println!("list func");
                                /* list_func(); */
                            }
                            _ => println!("Please choose between <break/func>")
                        }
                    }
                    else { println!("Specify what to list: list <break/func>"); }
                },
                "continue" | "c" => println!("continue"),
                "step" | "s" => println!("step"),
                "disas" | "d" => {
                    println!("dissasemble")
                },//argument
                "break" | "b" => {
                    println!("break at adress")
                },//argument
                "del break" | "db" => {
                    println!("delete breakpoint")
                },//argument
                "on" => {
                    println!("enable breakpoint")
                },//argument
                "off" => {
                    println!("disable breakpoint")
                },//argument
                "reg" => {
                    println!("values in all registers")
                },//argument
                "set reg" => {
                    println!("set register name")
                },//argument
                "mem" => {
                    println!("dump memory")
                },//argument
                "stack" => println!("dump memory from current stack"),
                "quit" | "q" => running = false,
                _ => println!("This command does not exist. Type 'help' for commands and functions."),
            },
            None => todo!(),
        // let option = splitted[0]; // First one is option, others are arguments
        //
        // match option {
        //     "help" | "h" => print_help(),
        //     "list" | "lf" => if splitted.len() > 1 && splitted[1] == "func" {
        //                         println!("lf was called"); /* list_functions(); */ 
        //                     },
        //     "quit" | "q" => running = false,
        //     _ => println!("This command does not exist. Type 'help' for commands and functions.")
        // }
        //println!();
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
";

    println!("{}\n", help_str);
}
