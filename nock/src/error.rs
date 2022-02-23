use std::{error, fmt};

/// A Nock-specific error encapsulating an informative error message.
#[derive(Debug)]
pub struct Error {
    pub msg: String,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl error::Error for Error {}
