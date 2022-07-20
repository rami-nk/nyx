use std::{io, str, string};

#[derive(Debug)]
pub enum NyxError {
    IoError(io::Error),
    Utf8Error(str::Utf8Error),
    FromUtf8Error(string::FromUtf8Error),
}

impl From<io::Error> for NyxError {
    fn from(err: io::Error) -> Self {
        NyxError::IoError(err)
    }
}

impl From<str::Utf8Error> for NyxError {
    fn from(err: str::Utf8Error) -> Self {
        NyxError::Utf8Error(err)
    }
}

impl From<string::FromUtf8Error> for NyxError {
    fn from(err: string::FromUtf8Error) -> Self {
        NyxError::FromUtf8Error(err)
    }
}
