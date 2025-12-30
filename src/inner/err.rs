use crate::inner::IoError;

/// A convenient type alias for `Result<T, UbjError>`
pub type UbjResult<T> = core::result::Result<T, UbjError>;

/// An error that can occur during serializing to (or deserializing from) Universal Binary JSON
#[derive(Debug)]
pub enum UbjError {
    /// A legal value type for which this crate does not implement serialization/deserialization yet
    Unsupported(&'static str),

    /// An illegal key type which is not allowed in Universal Binary JSON format
    IllegalKeyType(&'static str),

    /// An illegal character which is not allowed in Universal Binary JSON format.
    CharNotAscii(u32),

    /// A marker which was not expected in Universal Binary JSON format.
    UnexpectedMarker(u8),

    /// Unexpected end of file
    UnexpectedEof,

    /// Error involving the conversion of raw bytes to UTF-8 characters
    Utf8Error(core::str::Utf8Error),

    /// Error if the buffer is too small.
    ///
    /// It happens when the buffer does not have enough capacity to hold data that the deserializer
    /// wants to borrow (refer to) instead of copying.
    BufferTooSmall(usize),

    /// An enum variant index is larger than the maximum value allowed by the format.
    EnumVariantIndexTooLarge(u32),

    /// Error occurring IO (Input/Output) against the underlying writer/reader
    IO(IoError),

    #[cfg(feature = "std")]
    /// Any other error defined by the user of this crate
    Other(String),

    #[cfg(all(not(feature = "std"), feature = "embedded-io"))]
    /// Any other error defined by the user of this crate
    Other,
}

impl core::fmt::Display for UbjError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            UbjError::Unsupported(msg) => {
                write!(f, "Unsupported: {msg}")
            }

            UbjError::IllegalKeyType(msg) => {
                write!(f, "Illegal key type: {msg}")
            }

            UbjError::CharNotAscii(c) => {
                let code = *c;
                write!(f, "Char not within ASCII range: {code:#x}")
            }

            UbjError::UnexpectedMarker(m) => {
                write!(f, "Unexpected marker: {m:#x}")
            }

            UbjError::UnexpectedEof => {
                write!(f, "Unexpected end of file")
            }

            UbjError::BufferTooSmall(capacity) => {
                write!(
                    f,
                    "Buffer too small. Consider increasing its capacity to at least {}",
                    capacity
                )
            }

            UbjError::EnumVariantIndexTooLarge(v) => {
                write!(f, "Enum variant index is too large: {}", v)
            }

            UbjError::Utf8Error(err) => {
                write!(f, "UTF-8 error occurred: {}", err)
            }

            UbjError::IO(err) => {
                write!(f, "IO error occurred: {}", err)
            }

            #[cfg(feature = "std")]
            UbjError::Other(msg) => {
                write!(f, "{}", msg)
            }

            #[cfg(all(not(feature = "std"), feature = "embedded-io"))]
            UbjError::Other => {
                write!(f, "Other error occurred")
            }
        }
    }
}

#[cfg(all(not(feature = "std"), feature = "embedded-io"))]
impl UbjError {
    /// Convert an embedded-io write error into a UbjError
    pub fn from_io_error(e: impl embedded_io::Error) -> Self {
        UbjError::IO(e.kind())
    }
}

#[cfg(feature = "std")]
impl UbjError {
    /// Convert a std write error into a UbjError
    pub fn from_io_error(err: IoError) -> Self {
        UbjError::IO(err)
    }
}

impl From<core::str::Utf8Error> for UbjError {
    fn from(err: core::str::Utf8Error) -> Self {
        UbjError::Utf8Error(err)
    }
}

impl serde::ser::Error for UbjError {
    #[cfg(feature = "std")]
    fn custom<T: core::fmt::Display>(msg: T) -> Self {
        UbjError::Other(msg.to_string())
    }

    #[cfg(all(not(feature = "std"), feature = "embedded-io"))]
    fn custom<T: core::fmt::Display>(_msg: T) -> Self {
        UbjError::Other
    }
}

impl serde::de::Error for UbjError {
    #[cfg(feature = "std")]
    fn custom<T: core::fmt::Display>(msg: T) -> Self {
        UbjError::Other(msg.to_string())
    }

    #[cfg(all(not(feature = "std"), feature = "embedded-io"))]
    fn custom<T: core::fmt::Display>(_msg: T) -> Self {
        UbjError::Other
    }
}

// #[cfg(feature = "std")]
impl core::error::Error for UbjError {
    fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
        match self {
            UbjError::IO(err) => Some(err),
            _ => None,
        }
    }
}
