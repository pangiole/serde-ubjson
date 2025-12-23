use crate::prelude::*;

/// An error that can occur during serializing to (or deserializing from) Universal Binary JSON
#[derive(Debug)]
pub enum UbjError {
    // TODO Review the newtype variants taking a static string slice as we may need to leak memory

    /// A legal value type for which this crate does not implement serialization/deserialization yet
    UnimplementedValueType(&'static str),

    /// An illegal key type which is not allowed in Universal Binary JSON format
    IllegalKeyType(&'static str),

    /// An illegal character which is not allowed in Universal Binary JSON format.
    IllegalChar(char),

    /// Any other error defined by the user of this crate
    Custom(String),

    /// Error occurring IO (Input/Output) against the underlying writer/reader
    IO(IoError),
}


impl core::fmt::Display for UbjError {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            UbjError::UnimplementedValueType(msg) =>  write!(formatter, "Unimplemented value type: {msg}"),
            UbjError::IllegalKeyType(msg) =>  write!(formatter, "Illegal key type: {msg}"),
            UbjError::IllegalChar(c) => { let code = *c as u32; write!(formatter, "Char out of range: {code:#x}") },
            UbjError::Custom(msg) =>  write!(formatter, "{msg}"),
            UbjError::IO(err) =>  write!(formatter, "IO error occurred: {}", err),
        }
    }
}


impl From<IoError> for UbjError {
    fn from(err: IoError) -> Self {
        UbjError::IO(err)
    }
}


#[cfg(all(feature = "alloc", not(feature = "std")))]
use alloc::string::ToString;

impl serde::ser::Error for UbjError {
    fn custom<T: core::fmt::Display>(msg: T) -> Self {
        UbjError::Custom(msg.to_string())
    }
}


#[cfg(feature = "std")]
impl std::error::Error for UbjError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            UbjError::IO(err) => Some(err),
            _ => None
        }
    }
}

