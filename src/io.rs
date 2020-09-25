use directories::UserDirs;
use std::fs::File;
use std::io::prelude::*;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};

/// Writes markdown to markdown file in draft directory
pub fn write_markdown_to_draft(filename: &str, markdown: &str) -> Result<(), ErrorKind> {
    let mut file_path = get_draft_directory();
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
    let draft_directory = get_draft_directory();

    if !draft_directory.is_dir() {
        create_draft_directory(draft_directory.to_str().unwrap())
    }
}

/// Gets path to draft directory.
fn get_draft_directory() -> PathBuf {
    let mut document_dir = PathBuf::new();
    if let Some(user_dirs) = UserDirs::new() {
        document_dir = PathBuf::from(user_dirs.document_dir().unwrap().to_owned());
        document_dir.push("raptr-drafts");
    }
    document_dir
}

/// Creates draft directory.
///
/// # Panics
/// Panics if draft directory could not get created.
fn create_draft_directory(path: &str) {
    match std::fs::create_dir(Path::new(&path)) {
        Ok(_) => println!("Created directory"),
        Err(err) => panic!("Could not create directory: {}", err),
    };
}
