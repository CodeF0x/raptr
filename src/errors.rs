// use std::fmt;
use std::io::{ErrorKind};
use std::error::Error;

/**
#[derive(Debug)]
pub enum RaptrError {
    IOError,
}

impl std::error::Error for RaptrError {}

impl fmt::Display for RaptrError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RaptrError::IOError => write!(f, "I/O Error")
        }
    }
}
*/

pub fn display_io_error(error: std::io::Error, path: &str, verbose: bool) {
    match error.kind() {
        ErrorKind::NotFound=> eprintln!("File or directory not found: {}", path),
        ErrorKind::PermissionDenied => eprintln!("No suitable permissions to create or to write to: {}", path),
        ErrorKind::AlreadyExists => eprintln!("File or directory already exists: {}", path),
        _ => eprintln!("An unexpected error occurred while processing {}", path)
    }

    if verbose {
        eprintln!("{:?}", error);
    } else {
        println!("Run with -v for a more detailed error message.");
    }

    std::process::exit(1);
}