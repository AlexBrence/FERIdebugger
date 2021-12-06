use std::env::args;
use std::fs;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::{color, style};
use std::io::{stdin,stdout,Write,self,Read};
use std::fmt::Display;
use std::convert::TryInto;
extern crate crossterm;
use self::crossterm::cursor;
use self::crossterm::Screen;



pub fn print_prompt() {

    print!("{}╭─({}fdb", color::Fg(color::Green), style::Reset);
    io::Write::flush(&mut io::stdout())
        .expect("[ERROR] flush failed");

    println!("{})", color::Fg(color::Green));

    print!("╰─ > {}", style::Reset);
    io::Write::flush(&mut io::stdout())
        .expect("[ERROR] flush failed");
}

pub fn key_commands(prev_comms: &mut Vec<String>) -> String {
    let mut stdout = stdout().into_raw_mode().unwrap();
    let mut out_str = Vec::new();
    let mut s = String::new();
    let mut is_comm = false;
    let mut is_prev = false;
    let screen = Screen::default();
    let mut cursor = cursor(&screen);
    let (cur_x,cur_y) = cursor.pos();
    let mut comm_counter=prev_comms.len();
    for c in io::stdin().keys(){
        is_comm = false;
        match c.unwrap(){
            Key::Ctrl('q') =>{
                s="quit".to_string();
                break;
            },
            Key::Up =>{
                is_prev=true;
                cursor.goto(cur_x,cur_y);
                if(comm_counter!=0){
                    comm_counter-=1;
                    print!("{}",termion::clear::AfterCursor);
                    print!("{}",prev_comms[comm_counter]);
                    let mut s1=prev_comms[comm_counter].clone();
                    s=s1.to_string();
                }
                stdout.flush();
            
            },
            Key::Down =>{
                is_prev=true;
                cursor.goto(cur_x,cur_y);
                if(comm_counter<prev_comms.len()){
                    comm_counter+=1;
                    print!("{}",termion::clear::AfterCursor);
                    if(comm_counter>=prev_comms.len()){
                        print!("");
                    }else{
                        print!("{}",prev_comms[comm_counter-1]);
                        let mut s1=prev_comms[comm_counter-1].clone();
                        s=s1.to_string();
                    }
                }
                stdout.flush();
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
                    write!(stdout,"{}",c);
                }
                stdout.flush();
            },
            Key::Char('\n') => {
                break;
            },
            Key::Left =>{
                cursor.move_left(1);
            },
            Key::Backspace => {
                cursor.move_left(1);
                print!("{}",termion::clear::AfterCursor);
                out_str.remove(out_str.len()-1);
                let mut s:String=out_str.clone().into_iter().collect();
                stdout.flush().unwrap();            
            },
            _=> {
            }
        }
        stdout.flush().unwrap();
    };
    if is_comm && !is_prev{
        s = out_str.into_iter().collect();
        prev_comms.push(s.clone());
    }
    return s;
}

