#![allow(unused)]
use crate::program;
use termion::color;

pub fn print_registers(mut program :&mut program::Program){
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
pub fn print_spec_register(mut program:&mut program::Program,ime: String){
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