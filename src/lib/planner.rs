/// Raise an error.
#[macro_export]
macro_rules! raise(
    ($message:expr) => (return Err($crate::Error::new($message)));
    ($($argument:tt)*) => (return Err($crate::Error::new(format!($($argument)*))));
);

/// Unwrap a result or raise an error.
#[macro_export]
macro_rules! ok(
    ($result:expr) => (
        match $result {
            Ok(result) => result,
            Err(error) => raise!(error),
        }
    );
);

mod result;

pub mod format;
pub mod layout;

pub use result::{Error, Result};
