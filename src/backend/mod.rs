use std::fmt::Formatter;

#[cfg(not(target_arch = "wasm32"))]
mod native;
#[cfg(target_arch = "wasm32")]
mod web;

#[cfg(not(target_arch = "wasm32"))]
pub use native::{Database, Error as NativeError};
#[cfg(target_arch = "wasm32")]
pub use web::{Database, Error as WebError};

#[derive(Debug)]
pub struct Error {
    #[cfg(target_arch = "wasm32")]
    inner: WebError,
    #[cfg(not(target_arch = "wasm32"))]
    inner: NativeError,
}

pub type Result<T> = std::result::Result<T, Error>;

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Database error: {}", self.inner)
    }
}

#[cfg(target_arch = "wasm32")]
impl From<WebError> for Error {
    fn from(err: WebError) -> Self {
        Error {
            inner: err,
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error {
            inner: err,
        }
    }
}
