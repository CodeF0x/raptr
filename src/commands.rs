use crate::io;
use crate::config;

/// Checks if command exists and launches appropriate action.
pub fn handle_arguments<'a>(args: Vec<String>) -> Result<(), String> {
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
                    println!("Sorry, this is not implemented yet! :( Please use raptr publish instead.");
                    return Ok(());
                } else {
                    let config = config::read_config()?;
                    io::copy_theme_files()?;
                    io::render_blog(&config)?;
                    io::render_index(&config)?;
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
                            println!("Error: Please supply a name for your new site.");
                        }
                    } else if arg == "draft" {
                        if let Some(arg) = args.next() {
                            let draft_name = arg;
                            match io::create_new_draft(draft_name) {
                                Ok(_) => println!("Created new draft {}.md.", draft_name),
                                Err(err) => eprintln!("Could not create new draft {}.md: {:?}", draft_name, err),
                            }
                        } else {
                            println!("Error: Please supply a name for your new draft.");
                        }
                    }
                }
            }
            "theme" => {
                if let Some(arg) = args.next() {
                    let theme_name = arg;
                    config::change_theme(theme_name)?;
                } else {
                    println!("Error: Please provide a theme name.");
                    return Ok(())
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
    
    raptr new site <name>         Generates new project with <name>              Example: raptr new site my-awesome-blog

    raptr publish                 Generates HTML file(s) to standard path        Example: raptr publish
    raptr publish <path>          Generates HTML file(s) to specified path       Exmaple: raptr publish /home/juniper/blog-files
    ";
    println!("{}", help_command_string);
}
