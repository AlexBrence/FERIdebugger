use std::env::args;
use std::fs;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::{color, style};
use std::io::{stdin,stdout,Write,self, Read};

pub fn print_prompt() {

    print!("{}╭─({}fdb", color::Fg(color::Green), style::Reset);
    io::Write::flush(&mut io::stdout())
        .expect("[ERROR] flush failed");

    println!("{})", color::Fg(color::Green));

    print!("╰─ > {}", style::Reset);
    io::Write::flush(&mut io::stdout())
        .expect("[ERROR] flush failed");
}

pub fn key_commands(prev_comms: &mut Vec<String>, comm_counter:&mut usize) -> String {
    let mut stdout = stdout().into_raw_mode().unwrap();
    let mut out_str = Vec::new();
    let mut s = String::new();
    let mut is_comm = false;
    for c in io::stdin().keys(){
        is_comm = false;
        match c.unwrap(){
            Key::Ctrl('q') =>{
                s="quit".to_string();
                break;
            },
            Key::Up =>{
                if comm_counter < &mut prev_comms.len()&& comm_counter >= &mut 0{
                    print!("{}{}",termion::clear::CurrentLine,prev_comms[*comm_counter]);
                    let mut s1=&prev_comms[*comm_counter];
                    s=s1.to_string();
                    *comm_counter+=1;
                    stdout.flush();
                }
            },
            Key::Down =>{
                if *comm_counter > 0 && comm_counter < &mut prev_comms.len(){
                    print!("{}{}",termion::clear::CurrentLine,prev_comms[*comm_counter]);
                    let mut s1=&prev_comms[*comm_counter];
                    s=s1.to_string();
                    *comm_counter-=1;
                    stdout.flush();
                }
            },
            Key::Ctrl('l') => {
                println!("{}",termion::clear::All);
                print!("{}",termion::cursor::Goto(1,1));
                break;
            },
            Key::Char(c) =>{
                is_comm = true;
                if c == '\n'{
                    break;
                }
                else{
                    out_str.push(c);
                    print!("{}",c);
                }
                stdout.flush();
            },
            Key::Char('\n') => {
                break;
            }
            _=> {
            }
        }
        stdout.flush().unwrap();
    }
    if is_comm {
        s = out_str.into_iter().collect();
        prev_comms.push(s.clone());
    }
    return s;
}

