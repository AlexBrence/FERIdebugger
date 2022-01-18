# FERI debugger

Debugger written in rust at Faculty of Electrical Engineering and Computer Science FERI.
The project is done as a part of "Software development" course in one semester.

![alt-text](https://raw.githubusercontent.com/AlexBrence/FERIdebugger/main/FeriDebugger.png)

# Help page
```
FERI debugger

usage: fdb <input file>

optional arguments:
    -h                          display help

debugger commands:

    help                        print help for all commands
    run / r [arg1, arg2...]     run the program with arguments
    continue / c                continue execution
    step / s                    step one instruction
    stepover / so               step over one instruction/function

    d / disas [label]           disassemble function
    lf / list func              list all functions

    b / break [address]         set breakpoint at given address
    list break / lb             list all breakpoints
    del break [n]               delete breakpoint number [n]
    on/off [n]                  enable/disable breakpoint number [n]
                                if no argument given: enable/disable all

    reg                         print values in all registers
    reg [name]                  print value in [name] register
    set reg [name] [value]      set register [name] to value [value]
    set mem [address] [value]   set memory at [address] to [value]
    read <address> <n>          read n bytes from address

    mem [address] [n]           dump memory, [n] bytes starting from [address]
    stack                       dump memory from current stack frame

    info <header, process>      print information

    ! / shell <command>         run external shell command
```
