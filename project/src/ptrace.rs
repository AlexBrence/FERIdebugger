use std;
extern crate libc;

pub fn trace_me() {
    unsafe {
        libc::ptrace(libc::PTRACE_TRACEME, 0, 0, 0);
    }
}
