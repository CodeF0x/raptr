use std::fmt;

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