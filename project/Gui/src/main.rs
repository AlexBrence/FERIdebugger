use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button, ButtonBuilder, Builder, TextView, Entry, Label};

fn main() {
    let app = Application::builder().application_id("org.gtk-rs.demoApp").build();
    app.connect_activate(build_ui);
    app.run();
}

fn build_ui(app: &Application){
    let builder = gtk::Builder::from_string(include_str!("../gui_glade.glade"));

    let root: ApplicationWindow = builder.object("window1").expect("root not found");
    let go_button : Button = builder.object("go_button").expect("no button");
    let command_input : Entry = builder.object("terminal_input").expect("input not found");
    let status_display : Label = builder.object("status_display").expect("widnow not found");
    go_button.connect_clicked(move |_|{
        let message = command_input.text().to_string();
        status_display.set_text(&format!("{}",message));
    });
    let function_display : TextView = builder.object("function_display").expect("window not found");
    function_display.buffer().expect("window not found").set_text("debugger commands:
    help                    
    run / r [arg1, arg2...] 
    continue / c            
    step / s                

    d / disas [label]       
    lf / list func          

    b / break [address]     
    list break / lb         
    del break [n]           
    [n] on/off              

    reg                     
    reg [name]              
    set reg [name] [value]  

    mem [address] [n]       
    stack                   

    info <header, process> ");
    root.set_application(Some(app));
    root.present();
}


