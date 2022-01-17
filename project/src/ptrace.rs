use std;
extern crate libc;

pub fn trace_me() {
    unsafe {
        libc::ptrace(libc::PTRACE_TRACEME, 0, 0, 0);
        libc::personality(libc::ADDR_NO_RANDOMIZE as u64);
    }
}

pub fn peek_text(pid: i32, addr: u64) -> Result<u64, i32> {
    unsafe {
        *libc::__errno_location() = 0;
        let word: i64 = libc::ptrace(libc::PTRACE_PEEKTEXT, pid, addr, 0);
        if word == -1 && *libc::__errno_location() != 0 {
            return Err(*libc::__errno_location());
        } else {
            return Ok(word as u64);
        }
    }
}

pub fn poke_text(pid: i32, addr: u64, data: u64) {
    unsafe {
        libc::ptrace(libc::PTRACE_POKETEXT, pid, addr, data);
    }
}

pub fn get_user(pid: i32, user_struct: *mut libc::user) {
    unsafe {
        libc::ptrace(libc::PTRACE_GETREGS, pid, 0, user_struct);
    }
}

pub fn write_user(pid: i32, user_struct: *const libc::user) {
    unsafe {
        libc::ptrace(libc::PTRACE_SETREGS, pid, 0, user_struct);
    }
}

pub fn singlestep(target_pid: i32) {
    unsafe {
        libc::ptrace(libc::PTRACE_SINGLESTEP, target_pid, 0, 0);
    }
}

// 'continue' is a keyword in rust and can't be used here
pub fn resume(target_pid: i32) {
    unsafe {
        libc::ptrace(libc::PTRACE_CONT, target_pid, 0, 0);
    }
}

pub fn stop(pid: i32) {
    unsafe {
        libc::ptrace(libc::PTRACE_KILL, pid, 0, 0);
    }
}

pub fn backtrace(pid: i32, user_struct: *const libc::user) {
    unsafe {
        let prc = libc::ptrace(libc::PTRACE_GETFPXREGS, pid, 0, user_struct);
        println!("{}", prc);
    }
}
