#![allow(unused)]
use crate::program;
use termion::color;

pub fn print_registers(mut program :&mut program::Program){
    let prog = program.get_user_struct().regs;
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
    name_color,sp_s,number_color, prog.rax,number_color,prog.rax,
    name_color,sp_s,number_color,prog.rbx,number_color,prog.rbx,
    name_color,sp_s,number_color,prog.rcx,number_color,prog.rcx,
    name_color,sp_s,number_color,prog.rdx,number_color,prog.rdx,
    name_color,sp_s,number_color,prog.rsi,number_color,prog.rsi,
    name_color,sp_s,number_color,prog.rdi,number_color,prog.rdi,
    name_color,sp_s,number_color,prog.rbp,number_color,prog.rbp,
    name_color,sp_s,number_color,prog.rsp,number_color,prog.rsp,
    name_color,sp,number_color,prog.r8,number_color,prog.r8,
    name_color,sp,number_color,prog.r9,number_color,prog.r9,
    name_color,sp_s,number_color,prog.r10,number_color,prog.r10,
    name_color,sp_s,number_color,prog.r11,number_color,prog.r11,
    name_color,sp_s,number_color,prog.r12,number_color,prog.r12,
    name_color,sp_s,number_color,prog.r13,number_color,prog.r13,
    name_color,sp_s,number_color,prog.r14,number_color,prog.r14,
    name_color,sp_s,number_color,prog.r15,number_color,prog.r15,
    name_color,sp_s,number_color,prog.rip,number_color,prog.rip,
    name_color," ",number_color,prog.eflags,number_color,prog.eflags,
    name_color,sp,number_color,prog.cs,number_color,prog.cs,
    name_color,sp,number_color,prog.ss,number_color,prog.ss,
    name_color,sp,number_color,prog.ds,number_color,prog.ds,
    name_color,sp,number_color,prog.es,number_color,prog.es,
    name_color,sp,number_color,prog.fs,number_color,prog.fs,
    name_color,sp,number_color,prog.gs,number_color,prog.gs);
}
pub fn print_spec_register(mut program:&mut program::Program,ime: String){
    let prog = program.get_user_struct().regs;
    let name_color = color::Fg(color::Yellow);
    let number_color = color::Fg(color::Cyan);
    match &ime as &str{
        "rax" =>{
            println!("{}rax: {}0x{:016x}{}  {}",name_color,number_color,prog.rax,number_color,prog.rax);
            //println!("{}rax:{}   {}",pr{}ogram.get_user_struct().regs.rax);
        },
        "rbx" => {
            println!("{}rbx:{} 0x{:016x}{}  {}",name_color,number_color,prog.rbx,number_color,prog.rbx);
        },
        "rcx" => {
            println!("{}rcx:{} 0x{:016x}{}  {}",name_color,number_color,prog.rcx,number_color,prog.rcx);
        },
        "rdx" => {
            println!("{}rdx:{} 0x{:016x}{}  {}",name_color,number_color, prog.rdx,number_color,prog.rdx);
        },
        "rsi" => {
            println!("{}rsi:{} 0x{:016x}{}  {}",name_color,number_color,prog.rsi,number_color,prog.rsi);
        },
        "rdi" => {
            println!("{}rdi:{} 0x{:016x}{}  {}",name_color,number_color,prog.rdi,number_color,prog.rdi);
        },
        "rbp" => {
            println!("{}rbp:{} 0x{:016x}{}  {}",name_color,number_color,prog.rbp,number_color,prog.rbp);
        },
        "rsp" => {
            println!("{}rsp:{} 0x{:016x}{}  {}",name_color,number_color,prog.rsp,number_color,prog.rsp);
        },
        "r8" => {
            println!("{}r8: {}0x{:016x}{}  {}",name_color,number_color,prog.r8,number_color,prog.r8);
        },
        "r9" => {
            println!("{}r9: {}0x{:016x}{}  {}",name_color,number_color,prog.r9,number_color,prog.r9);
        },
        "r10" => {
            println!("{}r10:{} 0x{:016x}{}  {}",name_color,number_color,prog.r10,number_color,prog.r10);
        },
        "r11" => {
            println!("{}r11:{} 0x{:016x}{}  {}",name_color,number_color,prog.r11,number_color,prog.r11);
        },
        "r12" => {
            println!("{}r12:{} 0x{:016x}{}  {}",name_color,number_color,prog.r12,number_color,prog.r12);
        },
        "r13" => {
            println!("{}r13:{} 0x{:016x}{}  {}",name_color,number_color,prog.r13,number_color,prog.r13);
        },
        "r14" => {
            println!("{}r14:{} 0x{:016x}{}  {}",name_color,number_color,prog.r14,number_color,prog.r14);
        },
        "r15" => {
            println!("{}r15:{} 0x{:016x}{}  {}",name_color,number_color,prog.r15,number_color,prog.r15);
        },
        "rip" => {
            println!("{}rip:{} 0x{:016x}{}  {}",name_color,number_color,prog.rip,number_color,prog.rip);
        },
        "eflags" => {
            println!("{}eflags: {}0x{:016x}{}  {}",name_color,number_color,prog.eflags,number_color,prog.eflags);
        },
        "cs" => {
            println!("{}cs: {}0x{:016x}{}  {}",name_color,number_color,prog.cs,number_color,prog.cs);
        },
        "ss" => {
            println!("{}ss: {}0x{:016x}{}  {}",name_color,number_color,prog.ss,number_color,prog.ss);
        },
        "ds" => {
            println!("{}ds: {}0x{:016x}{}  {}",name_color,number_color,prog.ds,number_color,prog.ds);
        },
        "es" => {
            println!("{}es: {}0x{:016x}{}  {}",name_color,number_color,prog.es,number_color,prog.es);
        },
        "fs" => {
            println!("{}fs: {}0x{:016x}{}  {}",name_color,number_color,prog.fs,number_color,prog.fs);
        },
        "gs" => {
            println!("{}gs: {}0x{:016x}{}  {}",name_color,number_color,prog.gs,number_color,prog.gs);
        },
        _ => {
            println!("no register under that name lives here");
        }
    }
}