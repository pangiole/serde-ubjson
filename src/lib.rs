#![doc = include_str!("../README.md")]
#![warn(missing_docs)]
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(any(feature = "std", feature = "embedded-io")))]
compile_error! {
    "serde_ubj requires that either `std` (default) or `embedded-io` features to be enabled"
}

#[cfg(feature = "std")]
extern crate std;

// Our serde_ubj crate requires the alloc build-in crate for either 'std' or 'embedded-io' features.
extern crate alloc;

#[cfg(any(feature = "std", feature = "embedded-io"))]
mod inner {

    #[cfg(feature = "std")]
    pub use std::{io::BufRead as IoBufRead, io::Error as IoError, io::Write as IoWrite};

    #[cfg(all(not(feature = "std"), feature = "embedded-io"))]
    pub use embedded_io::{BufRead as IoBufRead, ErrorKind as IoError, Write as IoWrite};

    pub mod de;
    pub mod err;
    mod markers;
    mod reader;
    pub mod ser;
    mod writer;
}

// Re-exports

pub use inner::de::{from_vec, from_buf_reader};
pub use inner::err::UbjError;
pub use inner::err::UbjResult;
pub use inner::ser::{to_vec, to_writer};
