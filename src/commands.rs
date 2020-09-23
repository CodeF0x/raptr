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
            },
            "hatch" => {
                if let Some(arg) = args.next() {
                    port = arg.parse().unwrap();
                }
                web::launch_server(port);
            },
            _ => eprintln!("Command not found!")
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn builds_command() {
        let help_one = vec_of_strings![""];
        let help_two = vec_of_strings!["help"];
        
        let start_one = vec_of_strings!["hatch"];
        let start_two = vec_of_strings!["hatch", "3000"];

        let publish_one = vec_of_strings!["publish"];
        let publish_two = vec_of_strings!["publish", "/output"];
        let publish_three = vec_of_strings!["publish", "web"];

        let config = vec_of_strings!["config", "something=value"];

        // assert_eq!("", build_command(help_one));
        // assert_eq!("help", build_command(help_two));

        // assert_eq!("hatch", build_command(start_one));
        // assert_eq!("hatch 3000", build_command(start_two));

        // assert_eq!("publish", build_command(publish_one));
        // assert_eq!("publish /output", build_command(publish_two));
        // assert_eq!("publish web", build_command(publish_three));

        // assert_eq!("config something=value", build_command(config));
    }

    #[test]
    pub fn handles_command() {
        // assert_eq!(Ok(()), handle_command(String::from("")));
        // assert_eq!(Ok(()), handle_command(String::from("help")));
        // assert_eq!(Err("Command not found"), handle_command(String::from("invalid")));

        // Testing of webserver / webinterface happens in web.rs!!
    }
}