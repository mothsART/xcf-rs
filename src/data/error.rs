use std::io;
use std::string;

#[derive(Debug, Error)]
pub enum Error {
    Io(io::Error),
    Utf8(string::FromUtf8Error),
    InvalidFormat,
    UnknownVersion,
    InvalidPrecision,
    NotSupported,
}
