use crate::web;

#[macro_export]
macro_rules! vec_of_strings {
    ($($x:expr),*) => (vec![$($x.to_string()),*]);
}

/// Checks if command exists and launches appropriate action.
pub fn handle_arguments<'a>(args: Vec<String>) {
    let mut port = 3000;

    let mut args = args.iter();
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "" | "help" => {
                print_help();
            }
            "hatch" => {
                if let Some(arg) = args.next() {
                    port = arg.parse().unwrap();
                }
                web::launch_server(port);
            }
            "publish" => {
                if let Some(arg) = args.next() {
                    match arg.as_str() {
                        "web" => {
                            if let Some(arg) = args.next() {
                                let webroot = arg;
                                // TODO: publish with extra webroot given
                            } else {
                                // TODO: publish with no extra webroot given
                            }
                        }
                        _ => {
                            // TODO: publish to specific output directory
                        }
                    }
                } else {

                }
            }
            _ => eprintln!("Command not found!"),
        }
    }
}

/// Prints the help command.
fn print_help() {
    let help_command_string = "\
    Usage:
    
    raptr hatch - Starts the webinterface
    raptr hatch <port> - Starts the webinterface at <port> (port is optional)
    
    raptr publish - Genereates HTML file(s) to standard path
    raptr publish <path> - Generates HTML file(s) to specified path
    raptr publish web - Generates HTML file(s) to standard webroot
    
    raptr config <option>=<value> - Sets <option> to <value> in config file";
    println!("{}", help_command_string);
}
