use crate::io;
use crate::config;

/// Checks if command exists and launches appropriate action.
pub fn handle_arguments<'a>(args: Vec<String>) -> Result<(), std::io::Error>{
    let mut args = args.iter();
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "" | "help" => {
                print_help();
            }
            "publish" => {
                if let Some(_arg) = args.next() {
                    // let output_path = arg;
                    // todo output to specified directory
                } else {
                    let config = config::read_config()?;
                    io::copy_theme_files();
                    io::render_index(&config)?;
                    io::render_blog(&config)?;
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

    Ok(())
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
