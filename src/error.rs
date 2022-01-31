use std::io;

//=================================================================================================
// Enums
//=================================================================================================

/// A Vere-specific error encapsulating an informative error message.
#[derive(Debug)]
pub enum Error {
    StdIo,
}

impl From<io::Error> for Error {
    fn from(_err: io::Error) -> Self {
        Error::StdIo
    }
}
