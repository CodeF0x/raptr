//! # project.rs
//!
//! Contains all functions required for
//! project organisation.

use std::fs;
use std::path::Path;
use std::fs::File;
use fs_extra::dir::{copy, CopyOptions};
use crate::errors;
use std::process::exit;
use std::io::Write;
use chrono::prelude::*;
use crate::constants::CONFIG_FILE_DEFAULT_VALUE;

/// Creates a new project
///
/// # Arguments
///
/// * `project_name` - string slice of the project name
/// * `verbose` - boolean if verbose mode is on
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

    let mut config_file = match File::create(
        Path::new(root_dir).join("config.toml")
    ) {
        Ok(config_file) => config_file,
        Err(err) => {
            errors::display_io_error(err, project_name, verbose);
            exit(1);
        }
    };
    match config_file.write_all(CONFIG_FILE_DEFAULT_VALUE.as_bytes()) {
        Ok(_) => {},
        Err(err) => {
            errors::display_io_error(err, project_name, verbose);
            exit(1);
        }
    }

    println!("Created new project {}", project_name);
}

/// Copies assets from selected theme to output directory
/// and creates target directory and sub directories
/// in case they do not exist yet
///
/// # Arguments
///
/// * `theme_name` - string slice that holds the name of the used theme
/// * `output_dir` - string slice that holds the target path
/// * `verbose` - boolean if the verbose mode is on
pub fn prepare_output_dir(theme_name: &str, output_dir: &str, verbose: bool) {
    if let Ok(mut entries) = fs::read_dir("themes") {
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
    copy(
        format!("themes/{}/assets", theme_name), output_dir, &options
    ).expect("Could not copy all necessary theme files to the output destination.");
}

/// Creates a new draft file and writes it to ./drafts
///
/// # Arguments
///
/// * `theme_name` - string slice that holds the theme name
/// * `draft_name` - string slice that holds the name of the draft file
/// * `verbose` - boolean if verbose mode is on
pub fn create_new_draft(theme_name: &str, draft_name: &str, verbose: bool) {
    let mut draft_path = Path::new("drafts").join(draft_name);
    draft_path.set_extension("md");

    if draft_path.exists() {
        eprintln!("A draft with that name does already exist.");
        exit(1);
    }

    let mut draft_file = match File::create(&draft_path) {
        Ok(draft_file) => draft_file,
        Err(err) => {
            errors::display_io_error(err, &draft_path.to_str().unwrap_or(draft_name), verbose);
            exit(1);
        }
    };

    let archetype_path = Path::new("themes").join(theme_name).join("archetypes/post.md");
    let template_draft_header = match fs::read_to_string(&archetype_path) {
        Ok(template_draft_header) => template_draft_header,
        Err(err) => {
            errors::display_io_error(err, archetype_path.to_str().unwrap_or("post.md"), verbose);
            exit(1);
        }
    };
    let date = Local::now();
    let new_date_str = format!("date = \"{}\"", date);
    let new_draft_header = template_draft_header.replace("date = \"\"", &new_date_str);

    match draft_file.write_all(new_draft_header.as_bytes()) {
        Ok(_) => {},
        Err(err) => {
            errors::display_io_error(err, draft_path.to_str().unwrap_or(draft_name), verbose);
            exit(1);
        }
    }
    println!("Created a new draft at {}", draft_path.to_str().unwrap_or(draft_name));
}