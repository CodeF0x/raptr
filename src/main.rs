use clap::{Arg, App};

mod project;

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
        .get_matches();

    if let Some(project_name) = matches.value_of("new") {
        match project::create_project(project_name) {
            Ok(()) => {}
            Err(err) => eprintln!("{}", err)
        }
    }
}