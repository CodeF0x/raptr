use crate::io;
use crate::config;

#[macro_export]
macro_rules! vec_of_strings {
    ($($x:expr),*) => (vec![$($x.to_string()),*]);
}

/// Checks if command exists and launches appropriate action.
pub fn handle_arguments<'a>(args: Vec<String>) {
    let mut args = args.iter();
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "" | "help" => {
                print_help();
            }
            "publish" => {
                if let Some(arg) = args.next() {
                    let output_path = arg;
                    // match io::publish_drafts(Some(output_path)) {
                    //     Ok(_) => println!("Generated all files successfuly!"),
                    //     Err(err) => eprintln!("Error while generating files: {:?}", err),
                    // }
                } else {
                    match config::read_config() {
                        Ok(config) => match io::register_theme(&config) {
                            Ok(_) => println!("Super"),
                            Err(_) => println!("Nicht so super")
                        }
                        Err(_err) => eprintln!("Error while generating files:"),
                    }
                }
            }
            "new" => {
                if let Some(arg) = args.next() {
                    if arg == "site" {
                        if let Some(arg) = args.next() {
                            let site_name = arg;
                            match io::create_new_project(site_name) {
                                Ok(_) => println!("Your new project {} is ready.", site_name),
                                Err(err) => eprintln!("Could not create new project {}: {:?}", site_name, err),
                            }
                        } else {
                            println!("Please supply a name for your new site.");
                        }
                    }
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
    
    raptr new site <name>         Generates new project with <name>

    raptr publish                 Generates HTML file(s) to standard path
    raptr publish <path>          Generates HTML file(s) to specified path

    raptr config <option>=<value> Sets <option> to <value> in config file";
    println!("{}", help_command_string);
}
