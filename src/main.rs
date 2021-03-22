//! # main.rs
//!
//! Gathers user input and launches appropriate
//! program behavior.

mod project;
mod config;
mod render;
mod errors;
mod constants;

use clap::{Arg, App};
use config::Config;
use std::env;
use warp;

#[tokio::main]
async fn main() {
    let mut app = App::new("raptr")
        .version("0.1.0")
        .about("An opinionated blogging engine")
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
            .long("verbose")
            .help("Shows detailed errors and logging messages")
            .takes_value(false)
        )
        .arg(
            Arg::with_name("draft")
            .short("d")
            .long("draft")
            .value_name("DRAFT_NAME")
            .help("Creates a new draft")
            .takes_value(true)
        )
        .arg(
            Arg::with_name("serve")
                .short("s")
                .long("serve")
                .value_name("PORT")
                .help("Serves a preview at specified port or standard port if none is set")
                .takes_value(true)
                .default_value("3000")
        );

    let args: Vec<String> = env::args().skip(1).collect();
    if args.is_empty() {
        app.print_help().unwrap();
        return;
    }

    let matches = app.get_matches();
    let verbose = matches.occurrences_of("verbosity") == 1;

    if let Some(project_name) = matches.value_of("new") {
        project::create_project(&project_name, verbose);
    } else {
        if let Some(draft_name) = matches.value_of("draft") {
            let config = Config::new(verbose);
           project::create_new_draft(&config.theme, &draft_name, verbose);
        }
    
        // use occurrences_of because we use default_value above and so is_present
        // will still return true.
        if matches.occurrences_of("publish") == 1 {
            project::build_project(&matches, verbose);
        }

        if matches.occurrences_of("serve") == 1 {
            let port = matches.value_of("serve").unwrap().parse::<u16>().unwrap();
            let mut output_dir = env::temp_dir();
            output_dir.push("raptr");
            let output_dir = String::from(output_dir.to_str().unwrap());

            project::build_project(&matches, verbose);

            println!("Serving on localhost:{}. Press CRTL + C to exit.", port);

            warp::serve(warp::fs::dir(output_dir))
                .run(([127, 0, 0, 1], port))
                .await;
        }
    }
}
