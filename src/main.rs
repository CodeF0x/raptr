mod project;
mod errors;
mod config;
mod render;

use clap::{Arg, App};
use config::Config;
use render::RenderEngine;


fn main() {
    let matches = App::new("raptr")
        .version("0.1.0")
        .about("An experimental blogging engine")
        .author("Tobias \"CodeF0x\" Oettl <contact@codef0x.dev>")
        .arg(
            Arg::with_name("new")
            .short("n")
            .long("new")
            .value_name("PROJECT_NAME")
            .help("Creates a new project")
            .takes_value(true)
        )
        .arg(
            Arg::with_name("publish")
            .short("p")
            .long("publish")
            .value_name("OUTPUT_DIRECTORY")
            .help("Renders your drafts to web-ready html files")
            .takes_value(true)
            .default_value("output")
        )
        .arg(
            Arg::with_name("verbosity")
            .short("v")
            .long("verbosity")
            .help("Shows detailed errors and logging messages")
            .takes_value(false)
        )
        .get_matches();
        
    let verbosity_active = matches.is_present("verbosity");
    let verbosity_string = if verbosity_active { 
        String::from("") 
    } else { 
        String::from("Try again with -v / --verbosity to display a more detailed error") 
    };

    if let Some(project_name) = matches.value_of("new") {
        match project::create_project(&project_name) {
            Ok(_) => println!("Created your new project {}", project_name),
            Err(err) => {
                eprintln!("Could not create project. {}", verbosity_string);
                if verbosity_active {
                    eprintln!("{}", err);
                }
            }
        };
    }

    if matches.is_present("publish") {
        let config = Config::new();
        let render_engine = RenderEngine::new(&config.theme);

        project::prepate_output_dir(&config.theme);
        render_engine.render_index(&config);
        render_engine.render_blog_posts(&config);
    }
}