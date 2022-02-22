use std::io;

#[derive(Debug)]
pub enum Error {
    StdIo,
}

impl From<io::Error> for Error {
    fn from(_err: io::Error) -> Self {
        Error::StdIo
    }
}
