use serde_ubj::UbjError;

#[test]
fn display_ser_custom_error() {
    let msg = "An error occurred";
    let err = <UbjError as serde::ser::Error>::custom(msg);
    assert_eq!(err.to_string().as_str(), msg);
}

#[cfg(feature = "std")]
#[test]
fn display_io_error() {
    let io_err = std::io::Error::new(std::io::ErrorKind::Other, "Disk failure");
    let err = UbjError::IO(io_err);
    assert_eq!(err.to_string().as_str(), "IO error occurred: Disk failure");
}

#[cfg(feature = "std")]
#[test]
fn source_io_error() {
    use std::error::Error;
    let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "File not found");
    let err = UbjError::IO(io_err);
    assert!(err.source().is_some());
    assert_eq!(err.source().unwrap().to_string().as_str(), "File not found");
}