use std::env::args;
use std::fs;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::{color, style};
use std::io::{stdin,stdout,Write};
use std::io;

pub fn print_prompt() {
    /*
     * Creates nice looking hot and sexy prompt
     */

    print!("{}{}(", color::Fg(color::Green), style::Bold);
    print!("{}fdb", style::Reset);
    print!("{}{}) {}", color::Fg(color::Green), style::Bold, style::Reset);

    io::Write::flush(&mut io::stdout())
        .expect("[ERROR] flush failed");
}

pub fn key_commands (){
    let mut stdout = stdout().into_raw_mode().unwrap();
    let stdin = stdin();
    for c in stdin.keys(){
        match c.unwrap(){
            Key::Ctrl('q') =>{
                println!("quit");
                break;
            },
            Key::Up =>{
                println!("up");
                break;
            }
            _=> {}
        }
        stdout.flush().unwrap();
    }
}

