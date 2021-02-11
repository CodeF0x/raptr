mod project;
mod config;
mod render;
mod errors;

use clap::{Arg, App};
use config::Config;
use render::RenderEngine;
use std::env;

fn main() {
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
            let config = Config::new(verbose);
            let output_dir = matches.value_of("publish").unwrap_or("output");
            let render_engine = RenderEngine::new(&config.theme);
    
            project::prepare_output_dir(&config.theme, output_dir, verbose);
            let links = render_engine.render_blog_posts(output_dir, verbose);
            render_engine.render_index(&config, output_dir, links, verbose);
        }
    }
    
}
