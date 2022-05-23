use std::result;
use std::io;

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub enum ErrorKind {
    SerdeJson(serde_json::Error),
    IO(io::Error),
    JsonInvalidType,
    UsageInvalid,
    CommandSetLenUnder2,
    FieldEmpty,
}

impl ErrorKind {
    pub fn msg<S: Into<String>>(self, msg: S) -> Error {
        Error::new(self, msg.into())
    }
}

#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
    msg: String,
}

impl Error {
    pub fn new(kind: ErrorKind, msg: String) -> Self {
        Error { kind, msg }
    }

    pub fn kind(kind: ErrorKind) -> Self {
        Self::new(kind, "".to_string())
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for Error {}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Error::new(ErrorKind::SerdeJson(e), "".to_string())
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error::new(ErrorKind::IO(e), "".to_string())
    }
}
