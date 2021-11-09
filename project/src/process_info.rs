use sysinfo::{ProcessExt, System, SystemExt};

// Print information for a given pid
pub fn process_info(pid: i32) {
    let mut system = sysinfo::System::new_all();
    system.refresh_all();
    if let Some(process) = system.process(pid) {
        println!(
            "Name: {}\nPID: {}\nPPID: {}\nUID: {}\nGID: {}\nStatus: {}\nCmd: {:?}\nCwd: {:?}\nRoot: {:?}",
            process.name(), process.pid(), process.parent().unwrap(), process.uid, process.gid, process.status(), process.cmd(), process.cwd(), process.root()
        );
    }
}
