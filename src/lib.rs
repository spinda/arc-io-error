//! See the type-level documentation for [`IoError`](struct.IoError.html).

#![deny(missing_docs, missing_debug_implementations)]
#![doc(html_root_url = "https://docs.rs/arc-io-error/0.1.0")]

use std::error::Error;
use std::fmt;
use std::io;
use std::mem;
use std::sync::Arc;

/// A version of
/// [`std::io::Error`](https://doc.rust-lang.org/std/io/struct.Error.html)
/// implemented on top of
/// [`Arc`](https://doc.rust-lang.org/std/sync/struct.Arc.html)
/// instead of
/// [`Box`](https://doc.rust-lang.org/std/boxed/struct.Box.html),
/// making it cloneable.
///
/// The API of this type has been designed to match
/// [`io::Error`](https://doc.rust-lang.org/std/io/struct.Error.html), with
/// two exceptions:
///
/// - [`IoError::new`](struct.IoError.html#method.new) and
///   [`IoError::into_inner`](struct.IoError.html#method.into_inner) substitute
///   [`Arc`](https://doc.rust-lang.org/std/sync/struct.Arc.html) for
///   [`Box`](https://doc.rust-lang.org/std/boxed/struct.Box.html), and
/// - [`IoError`](struct.IoError.html) has no equivalent to
///   [`io::Error::get_mut`](https://doc.rust-lang.org/std/io/struct.Error.html#method.get_mut),
///   as the inner error instance is shared.
///
/// See the standard library documentation for more detailed API-level
/// descriptions than are given here.
///
/// [`IoError`](struct.IoError.html) implements
/// [`From`](https://doc.rust-lang.org/std/convert/trait.From.html)
/// for [`io::Error`](https://doc.rust-lang.org/std/io/struct.Error.html)
/// and vice-versa, so the two types can easily be converted between each other.
/// A type containing
/// [`io::Error`](https://doc.rust-lang.org/std/io/struct.Error.html) can
/// be made
/// [`Clone`](https://doc.rust-lang.org/std/clone/trait.Clone.html)-compatible
/// by instead storing [`IoError`](struct.IoError.html) internally and
/// converting from/to
/// [`io::Error`](https://doc.rust-lang.org/std/io/struct.Error.html) on
/// API boundaries.
///
/// Clones derived from the same original [`IoError`](struct.IoError.html)
/// instance will share a single heap-allocated error instance (if one is
/// present) using
/// [`Arc`](https://doc.rust-lang.org/std/sync/struct.Arc.html).
/// [`io::Error`](https://doc.rust-lang.org/std/io/struct.Error.html)
/// instances produced by converting those clones back with the
/// [`From`](https://doc.rust-lang.org/std/convert/trait.From.html)
/// implementation will also share the same single error instance.
#[derive(Clone)]
pub struct IoError(IoErrorRepr);

/// See [`std::io::ErrorKind`](https://doc.rust-lang.org/std/io/enum.ErrorKind.html).
pub use io::ErrorKind as IoErrorKind;

/// See [`std::io::Result`](https://doc.rust-lang.org/std/io/type.Result.html).
pub type IoResult<T> = Result<T, IoError>;

#[derive(Clone)]
enum IoErrorRepr {
    Os(i32),
    Kind(IoErrorKind),
    Custom(IoErrorKind, Arc<Error + Send + Sync>),
}

impl IoError {
    /// See
    /// [`io::Error::new`](https://doc.rust-lang.org/std/io/struct.Error.html#method.new),
    /// with
    /// [`Arc`](https://doc.rust-lang.org/std/sync/struct.Arc.html)
    /// substitued for
    /// [`Box`](https://doc.rust-lang.org/std/boxed/struct.Box.html).
    pub fn new<E>(kind: IoErrorKind, error: E) -> Self
        where E: Into<Arc<Error + Send + Sync>>
    {
        IoError(IoErrorRepr::Custom(kind, error.into()))
    }

    /// See
    /// [`io::Error::last_os_error`](https://doc.rust-lang.org/std/io/struct.Error.html#method.last_os_error).
    pub fn last_os_error() -> Self {
        io::Error::last_os_error().into()
    }

    /// See
    /// [`io::Error::from_raw_os_error`](https://doc.rust-lang.org/std/io/struct.Error.html#method.from_raw_os_error).
    pub fn from_raw_os_error(code: i32) -> Self {
        IoError(IoErrorRepr::Os(code))
    }

    /// See
    /// [`io::Error::raw_os_error`](https://doc.rust-lang.org/std/io/struct.Error.html#method.raw_os_error).
    pub fn raw_os_error(&self) -> Option<i32> {
        match self.0 {
            IoErrorRepr::Os(code) => Some(code),
            _ => None,
        }
    }

    /// See
    /// [`io::Error::get_ref`](https://doc.rust-lang.org/std/io/struct.Error.html#method.get_ref).
    pub fn get_ref(&self) -> Option<&('static + Error + Send + Sync)> {
        match self.0 {
            IoErrorRepr::Custom(_, ref inner) => Some(inner.as_ref()),
            _ => None,
        }
    }

    /// See
    /// [`io::Error::into_inner`](https://doc.rust-lang.org/std/io/struct.Error.html#method.into_inner).
    pub fn into_inner(self) -> Option<Arc<Error + Send + Sync>> {
        match self.0 {
            IoErrorRepr::Custom(_, inner) => Some(inner),
            _ => None,
        }
    }

    /// See
    /// [`io::Error::kind`](https://doc.rust-lang.org/std/io/struct.Error.html#method.kind)
    /// with
    /// [`Arc`](https://doc.rust-lang.org/std/sync/struct.Arc.html)
    /// substitued for
    /// [`Box`](https://doc.rust-lang.org/std/boxed/struct.Box.html).
    pub fn kind(&self) -> IoErrorKind {
        match self.0 {
            IoErrorRepr::Os(code) => io::Error::from_raw_os_error(code).kind(),
            IoErrorRepr::Kind(kind) => kind,
            IoErrorRepr::Custom(kind, _) => kind,
        }
    }
}

impl From<io::Error> for IoError {
    fn from(src: io::Error) -> Self {
        if let Some(code) = src.raw_os_error() {
            return IoError(IoErrorRepr::Os(code));
        }

        let kind = src.kind();
        match src.into_inner() {
            None => IoError(IoErrorRepr::Kind(kind)),
            Some(inner) => {
                let shared = Arc::new(BoxError(inner));
                IoError(IoErrorRepr::Custom(kind, shared))
            }
        }
    }
}

impl From<IoError> for io::Error {
    fn from(src: IoError) -> Self {
        match src.0 {
            IoErrorRepr::Os(code) => io::Error::from_raw_os_error(code),
            IoErrorRepr::Kind(kind) => kind.into(),
            IoErrorRepr::Custom(kind, inner) => io::Error::new(kind, ArcError(inner)),
        }
    }
}

impl From<IoErrorKind> for IoError {
    fn from(src: IoErrorKind) -> IoError {
        IoError(IoErrorRepr::Kind(src))
    }
}

impl fmt::Debug for IoError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self.0 {
            IoErrorRepr::Os(code) => fmt::Debug::fmt(&io::Error::from_raw_os_error(code), fmt),
            IoErrorRepr::Kind(kind) => fmt::Debug::fmt(&io::Error::from(kind), fmt),
            IoErrorRepr::Custom(ref kind, ref inner) => {
                fmt.debug_struct("Error").field("repr", &(kind, inner)).finish()
            }
        }
    }
}

impl fmt::Display for IoError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self.0 {
            IoErrorRepr::Os(code) => fmt::Display::fmt(&io::Error::from_raw_os_error(code), fmt),
            IoErrorRepr::Kind(kind) => fmt::Display::fmt(&io::Error::from(kind), fmt),
            IoErrorRepr::Custom(_, ref inner) => fmt::Display::fmt(inner, fmt),
        }
    }
}

impl Error for IoError {
    fn description(&self) -> &str {
        match self.0 {
            IoErrorRepr::Os(code) => {
                unsafe { mem::transmute(io::Error::from_raw_os_error(code).description()) }
            }
            IoErrorRepr::Kind(kind) => {
                unsafe { mem::transmute(io::Error::from(kind).description()) }
            }
            IoErrorRepr::Custom(_, ref inner) => inner.description(),
        }
    }

    fn cause(&self) -> Option<&Error> {
        match self.0 {
            IoErrorRepr::Custom(_, ref inner) => inner.cause(),
            _ => None,
        }
    }
}

struct ArcError(Arc<Error + Send + Sync>);

impl fmt::Debug for ArcError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&self.0, fmt)
    }
}

impl fmt::Display for ArcError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.0, fmt)
    }
}

impl Error for ArcError {
    fn description(&self) -> &str {
        self.0.description()
    }

    fn cause(&self) -> Option<&Error> {
        self.0.cause()
    }
}

struct BoxError(Box<Error + Send + Sync>);

impl fmt::Debug for BoxError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&self.0, fmt)
    }
}

impl fmt::Display for BoxError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.0, fmt)
    }
}

impl Error for BoxError {
    fn description(&self) -> &str {
        self.0.description()
    }

    fn cause(&self) -> Option<&Error> {
        self.0.cause()
    }
}
