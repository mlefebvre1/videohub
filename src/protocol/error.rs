use serde::{de, ser};
use std::{fmt, num::ParseIntError};
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("The string {0} can't be converted to a valid DevicePresent variant")]
    DevicePresentErr(String),
    #[error("The string {0} can't be converted to a valid LockStatus variant")]
    LockStatusErr(String),
    // Serializer errors
    #[error("Protocol serialization error")]
    SerializeErr(String),
    #[error("The type {0} you attempt to serialize is not supported")]
    SerializeNotSupportedTypeErr(String),
    #[error("The Block type {0} you attempt to serialize is not supported")]
    SerializeNotSupportedBlockTypeErr(String),
    // Deserializer errors
    #[error("Protocol deserialization error")]
    DeserializeErr(String),
    #[error("Expected semi-colons to delimit key, but found nothing")]
    ExpectedKey,
    #[error("Did not succesfully deserialize, because there are remaining characters")]
    TrailingCharacters,
    // Below is from deserialize json..
    #[error("Attempted to consume a character, but the buffer ios empty")]
    Eof,
    #[error("Expected to deserialize {0} to boolean")]
    ExpectedBoolean(String),
    #[error("Expected to deserialize an integer but found a character different than a digit")]
    ExpectedInteger,
    #[error("Attempted to deserialize an unsupported type.")]
    UnsupportedType,
    #[error("Expected a colon after a key")]
    ExpectedMapColon,
    #[error("Failed parsing an int from a string")]
    ParseInt(#[from] ParseIntError),
}

impl ser::Error for Error {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        Error::SerializeErr(msg.to_string())
    }
}

impl de::Error for Error {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        Error::DeserializeErr(msg.to_string())
    }
}
