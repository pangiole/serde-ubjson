
#[cfg(feature = "std")]
pub use std::{vec::Vec, string::String, boxed::Box};

// Define a synonym for the underlying std module (if the std feature is enabled)
#[cfg(feature = "std")]
use std::io;

#[cfg(all(feature = "alloc", not(feature = "std")))]
pub use alloc::{vec::Vec, string::String, boxed::Box};

// As a last resort, include our fake IO module
#[cfg(not(any(feature = "std")))]
mod io;

// Re-export the Error type from either the Rust std::io module (if the std feature is enabled)
// or from the fake crate::io module (if the std feature is disabled).
pub use io::Write as IoWrite;
pub use io::Error as IoError;