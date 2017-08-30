#![cfg_attr(feature = "strict", deny(warnings))]
#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]

extern crate arc_io_error;

use std::error::Error;
use std::fmt;
use std::io;

use arc_io_error::{IoError, IoErrorKind};

#[test]
fn test_os() {
    let err1 = io::Error::from_raw_os_error(42);
    let err2 = IoError::from(io::Error::from_raw_os_error(42));
    let err3 = err2.clone();
    let err4 = io::Error::from(err3.clone());

    assert_eq!(err1.description(), err2.description());
    assert_eq!(err1.description(), err3.description());
    assert_eq!(err1.description(), err4.description());

    assert_eq!(err1.to_string(), err2.to_string());
    assert_eq!(err1.to_string(), err3.to_string());
    assert_eq!(err1.to_string(), err4.to_string());

    assert_eq!(err1.raw_os_error().unwrap(), 42);
    assert_eq!(err2.raw_os_error().unwrap(), 42);
    assert_eq!(err3.raw_os_error().unwrap(), 42);
    assert_eq!(err4.raw_os_error().unwrap(), 42);

    assert_eq!(err1.kind(), err2.kind());
    assert_eq!(err1.kind(), err3.kind());
    assert_eq!(err1.kind(), err4.kind());

    assert!(err1.into_inner().is_none());
    assert!(err2.into_inner().is_none());
    assert!(err3.into_inner().is_none());
    assert!(err4.into_inner().is_none());
}

#[test]
fn test_kind() {
    let err1 = io::Error::from(IoErrorKind::ConnectionReset);
    let err2 = IoError::from(io::Error::from(IoErrorKind::ConnectionReset));
    let err3 = err2.clone();
    let err4 = io::Error::from(err3.clone());

    assert_eq!(err1.description(), err2.description());
    assert_eq!(err1.description(), err3.description());
    assert_eq!(err1.description(), err4.description());

    assert_eq!(err1.to_string(), err2.to_string());
    assert_eq!(err1.to_string(), err3.to_string());
    assert_eq!(err1.to_string(), err4.to_string());

    assert!(err1.raw_os_error().is_none());
    assert!(err2.raw_os_error().is_none());
    assert!(err3.raw_os_error().is_none());
    assert!(err4.raw_os_error().is_none());

    assert_eq!(err1.kind(), IoErrorKind::ConnectionReset);
    assert_eq!(err2.kind(), IoErrorKind::ConnectionReset);
    assert_eq!(err3.kind(), IoErrorKind::ConnectionReset);
    assert_eq!(err4.kind(), IoErrorKind::ConnectionReset);

    assert!(err1.into_inner().is_none());
    assert!(err2.into_inner().is_none());
    assert!(err3.into_inner().is_none());
    assert!(err4.into_inner().is_none());
}

#[test]
fn test_custom() {
    #[derive(Debug)]
    struct MyError(u8);

    impl fmt::Display for MyError {
        fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
            write!(f, "{}: {}", self.description(), self.0)
        }
    }

    impl Error for MyError {
        fn description(&self) -> &str {
            "my error"
        }

        fn cause(&self) -> Option<&Error> {
            None
        }
    }

    let err1 = io::Error::new(IoErrorKind::ConnectionReset, MyError(42));
    let err2 = IoError::from(io::Error::new(IoErrorKind::ConnectionReset, MyError(42)));
    let err3 = err2.clone();
    let err4 = io::Error::from(err3.clone());

    assert_eq!(err1.description(), err2.description());
    assert_eq!(err1.description(), err3.description());
    assert_eq!(err1.description(), err4.description());

    assert_eq!(err1.to_string(), err2.to_string());
    assert_eq!(err1.to_string(), err3.to_string());
    assert_eq!(err1.to_string(), err4.to_string());

    assert!(err1.raw_os_error().is_none());
    assert!(err2.raw_os_error().is_none());
    assert!(err3.raw_os_error().is_none());
    assert!(err4.raw_os_error().is_none());

    assert_eq!(err1.kind(), IoErrorKind::ConnectionReset);
    assert_eq!(err2.kind(), IoErrorKind::ConnectionReset);
    assert_eq!(err3.kind(), IoErrorKind::ConnectionReset);
    assert_eq!(err4.kind(), IoErrorKind::ConnectionReset);

    assert_eq!(err1.into_inner().unwrap().to_string(), "my error: 42");
    assert_eq!(err2.into_inner().unwrap().to_string(), "my error: 42");
    assert_eq!(err3.into_inner().unwrap().to_string(), "my error: 42");
    assert_eq!(err4.into_inner().unwrap().to_string(), "my error: 42");
}
