use std::fs;
use std::path::Path;
use std::fs::File;
use fs_extra::dir::{copy, CopyOptions};
use crate::errors;
use std::process::exit;
use std::io::Write;
use chrono::prelude::*;

pub fn create_project(project_name: &str, verbose: bool) {
    let root_dir = Path::new(&project_name);
    match fs::create_dir(&root_dir) {
        Ok(_) => {},
        Err(err) => {
            errors::display_io_error(err, project_name, verbose);
            exit(1);
        }
    }

    let sub_dirs = vec!["drafts", "output", "themes", "output/posts"];

    for dir in sub_dirs {
        let sub_dir = root_dir.join(dir);
        match fs::create_dir(sub_dir) {
            Ok(_) => {},
            Err(err) => {
                errors::display_io_error(err, project_name,  verbose);
                exit(1);
            }
        }
    }

    println!("Created new project {}", root_dir.to_str().unwrap_or(project_name));
}

pub fn prepare_output_dir(theme_name: &str, output_dir: &str, verbose: bool) {
    if let Ok(mut entries) = fs::read_dir("templates") {
        if entries.next().is_none() {
            eprintln!("You don't have any themes installed. Please add a theme to the themes directory in your project root.");
            exit(1);
        }
    }

    if !Path::new(output_dir).exists() {
        match fs::create_dir_all(output_dir) {
            Ok(_) => {},
            Err(err) => {
                errors::display_io_error(err, output_dir, verbose);
                exit(1);
            }
        }
    }

    let mut options = CopyOptions::new();
    options.overwrite = true;
    let _ = copy(
        format!("themes/{}/assets", theme_name), output_dir, &options
    ).unwrap();
}

pub fn create_new_draft(theme_name: &str, draft_name: &str) {
    let mut draft_path = Path::new("drafts").join(draft_name);
    draft_path.set_extension("md");

    if draft_path.exists() {
        eprintln!("A draft with that name already exists.");
        exit(1);
    }

    let mut draft_file = File::create(&draft_path).expect("Could not create new draft.");

    let template_draft_header = fs::read_to_string(
        format!("themes/{}/archetypes/post.md", theme_name)
    ).expect("Could not create a new draft.");
    let date = Local::now();
    let new_date_str = format!("date = \"{}\"", date);
    let new_draft_header = template_draft_header.replace("date = \"\"", &new_date_str);

    draft_file.write_all(new_draft_header.as_bytes()).expect("Could not create a new draft.");
    println!("Created a new draft {}", draft_name);
}