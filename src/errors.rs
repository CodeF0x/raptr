// use std::fmt;
use std::io::ErrorKind;

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

pub fn display_io_error(error_kind: ErrorKind, path: &str) {
    match error_kind {
        ErrorKind::NotFound=> eprintln!("File or directory not found: {}", path),
        ErrorKind::PermissionDenied => eprintln!("No suitable permissions to create or to write to: {}", path),
        ErrorKind::AlreadyExists => eprintln!("File or directory already exists: {}", path),
        _ => eprintln!("An unexpected error occurred while processing {}.", path)
    }
    std::process::exit(1);
}