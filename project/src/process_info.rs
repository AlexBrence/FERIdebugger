use sysinfo::{ProcessExt, System, SystemExt};
use termion::{color, style};

// Print information for a given pid
pub fn process_info(pid: i32) {
    let mut system = sysinfo::System::new_all();
    system.refresh_all();
    if let Some(process) = system.process(pid) {
        println!(
            "{c1}Name: {c2}{}\n{c1}PID: {c2}{}\n{c1}PPID: {c2}{}\n{c1}UID: {c2}{}\n{c1}GID: {c2}{}\n{c1}Status: {c2}{}\n{c1}Cmd: {c2}{:?}\n{c1}Cwd: {c2}{:?}\n{c1}Root: {c2}{:?}",
            process.name(), process.pid(), process.parent().unwrap(), process.uid, process.gid, process.status(), process.cmd(), process.cwd(), process.root(),
            c1=color::Fg(color::LightMagenta), c2=color::Fg(color::Red)
        );
    }
}
