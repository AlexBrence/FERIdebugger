extern crate termion;       // for colors, style
use termion::{color, style};

use std::io;


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


fn main() {
    let mut running: bool = true;

    println!("Welcome to Feri Debugger. For commands and functions type 'help'.\n");

    // Main loop
    while running {
        print_prompt();
        let input = get_input();

        match input.as_str() {
            "help" | "h" => println!("prc mogo sem nekaj tu dat just for tmp") /* help() */,
            "quit" | "q" => running = false,
            _ => println!("This command does not exist. Type 'help' for commands and functions.")
        }
        println!();
    }
}
