fn main() {
    print_help();
}

fn print_help() {
    let help_str: &str = "FERI debugger
        
usage: fdb <input file>
    
optional arguments:
    -h                      display help

debugger commands:

    help                    print help for all commands
    run / r                 run the program
    continue / c            continue execution
    step / s                step one instruction

    b / break [address]     set breakpoint at given address
    list break / lb         list all breakpoints
    del break [n]           delete breakpoint number [n]

    reg                     print values in all registers
    reg [name]              print value in [name] register
    set reg [name] [value]  set register [name] to value [value]

    mem [address] [n]       dump memory, [n] bytes starting from [address]
    stack                   dump memory from current stack frame
";

    println!("{}\n", help_str);
}
