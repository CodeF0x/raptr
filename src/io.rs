use directories::UserDirs;
use std::fs::File;
use std::io::prelude::*;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};
use std::fs;
use comrak;

/// Writes markdown to markdown file in draft directory
pub fn write_markdown_to_draft(filename: &str, markdown: &str) -> Result<(), ErrorKind> {
    let mut file_path = get_default_draft_directory();
    file_path.push(filename);

    let mut file = match File::create(&file_path) {
        Ok(file) => file,
        Err(err) => {
            eprintln!("Could not create file because: {}", err);
            return Err(err.kind());
        }
    };

    match file.write_all(markdown.as_bytes()) {
        Ok(_) => Ok(()),
        Err(err) => {
            eprintln!("Could not save draft because: {}", err);
            return Err(err.kind());
        }
    }
}

/// Checks if working criteria is met.
pub fn setup() {
    let draft_directory = get_default_draft_directory();
    let output_directory = get_default_output_directory();

    if !draft_directory.is_dir() {
        create_default_draft_directory(draft_directory)
    }

    if !output_directory.is_dir() {
        create_default_output_directory(output_directory);
    }

}

/// Gets path to the default draft directory.
fn get_default_draft_directory() -> PathBuf {
    let mut document_dir = PathBuf::new();
    if let Some(user_dirs) = UserDirs::new() {
        document_dir = PathBuf::from(user_dirs.document_dir().unwrap().to_owned());
        document_dir.push("raptr-drafts");
    }
    document_dir
}

/// Gets the path to the default output directory.
fn get_default_output_directory() -> PathBuf {
    let mut document_dir = PathBuf::new();
    if let Some(user_dirs) = UserDirs::new() {
        document_dir = PathBuf::from(user_dirs.document_dir().unwrap().to_owned());
        document_dir.push("raptr-output");
    }
    document_dir
}

/// Creates draft directory.
///
/// # Panics
/// Panics if draft directory connot be created.
fn create_default_draft_directory(path: PathBuf) {
    match fs::create_dir(Path::new(&path)) {
        Ok(_) => println!("Created default default directory"),
        Err(err) => panic!("Could not create default draft directory: {}", err),
    };
}

/// Creates default output directory.
/// 
/// # Panics
/// Panics if output directory cannot be created.
fn create_default_output_directory(path: PathBuf) {
    match fs::create_dir(Path::new(&path)) {
        Ok(_) => println!("Created default output directory"),
        Err(err) => panic!("Could not create default output directory: {}", err),
    }
}

/// Gets all filenames in draft directory.
pub fn get_files() -> Vec<String> {
    let mut paths = Vec::new();
    
    if let Ok(dirs) = fs::read_dir(get_default_draft_directory()) {
        for path in dirs {
            if let Ok(path) = path {
                let file_name = path.file_name().into_string().unwrap();
                paths.push(file_name);
            }
        }
    }

    paths
}

/// Reads single file by file name.
pub fn read_file(name: &str) -> String {
    let draft_directory = get_default_draft_directory();
    let path = format!("{}/{}", draft_directory.display().to_string(), name);

    println!("Path: {}", &path);
    match fs::read_to_string(Path::new(&path)) {
        Ok(content) => content,
        Err(_err) => String::from("Could not read file! Go back and try again."),
    }
}

/// Iterates through draft directory an generates HTML file from every file inside.
pub fn publish_drafts(path: Option<&String>) {
    let mut output_dir = PathBuf::new();

    match path {
        Some(custom_dir) => {
            output_dir = PathBuf::from(custom_dir)
        },
        None => {
            if let Some(user_dirs) = UserDirs::new() {
                let mut document_dir = user_dirs.document_dir().unwrap().to_owned();
                document_dir.push("raptr-output");
                output_dir = document_dir;
            }
        }
    }
    
    let draft_directory = get_default_draft_directory();
    if let Ok(dir) = fs::read_dir(draft_directory) {
        for path in dir {
            if let Ok(path) = path {
                let file_name = path.file_name().into_string().unwrap();
                
                let html_output = comrak::markdown_to_html(&read_file(&file_name), &comrak::ComrakOptions::default());
                
                let html_path = format!(
                    "{}/{}{}", 
                    output_dir.display(), 
                    &file_name.replace(".md", ""), 
                    ".html");

                let mut file = match File::create(&html_path) {
                    Ok(file) => file,
                    Err(err) => panic!("Could not write file {} because {}", &html_path, err),
                };

                match file.write_all(html_output.as_bytes()) {
                    Ok(_) => println!("Successfully generated file: {}", html_path),
                    Err(err) => eprintln!("Could not generate file: {} because {}", html_path, err),
                }
            }
        }
    }
}