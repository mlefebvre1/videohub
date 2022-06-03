use serde::{de, ser};
use std::fmt::Display;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Message(String),
    DevicePresentError,
    DeviceInfoError,
    LockStatusError,
    ParseInt(std::num::ParseIntError),
    ParseValueError,
    LabelsLengthError,
}

impl ser::Error for Error {
    fn custom<T: Display>(msg: T) -> Self {
        Error::Message(msg.to_string())
    }
}

impl de::Error for Error {
    fn custom<T: Display>(msg: T) -> Self {
        Error::Message(msg.to_string())
    }
}

impl Display for Error {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::Message(msg) => formatter.write_str(msg),
            _ => formatter.write_str("unexpected end of input"), /* and so forth */
        }
    }
}

impl std::error::Error for Error {}
