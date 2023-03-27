use std::num::ParseIntError;

#[derive(Debug)]
pub enum Error {
    Parse,
    ParseInt,
    TooFarBack,
    Write(std::io::Error),
}

impl From<ParseIntError> for Error {
    fn from(_: ParseIntError) -> Self {
        Self::ParseInt
    }
}