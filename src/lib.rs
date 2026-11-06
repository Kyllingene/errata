//! Print panics in a sensible fashion.
//!
//! First, place the following before your `main` function:
//! ```rust
//! use errata::FallibleExt;
//!
//! #[errata::catch]
//! fn main() {
//!     // ...
//! }
//! ```
//!
//! This unlocks pretty-printing for your error messages. For `Option` and
//! `Result` (as long as the `Err` variant implements `Display`), you can use
//! [`fail`](FallibleExt::fail) as a drop-in replacement for `expect`. You can
//! continue to use `unwrap` and `expect` where you don't expect any errors,
//! and you will continue to get useful debug information just like normal.
//!
//! If you wish to throw your own errors, see [`error`].

use std::fmt::Display;

pub use errata_macros::catch;

#[doc(hidden)]
#[derive(Debug)]
pub struct ErrataPanic(pub String);

impl Display for ErrataPanic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Exits the program cleanly, calling destructors and printing an error message.
/// Uses the same syntax as `format`.
///
/// If you enable feature `color`, prints in bold red.
#[cfg(not(feature = "color"))]
#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
        std::panic::panic_any($crate::ErrataPanic(format!($($arg)*)));
    }
}

/// Exits the program cleanly, calling destructors and printing an error message.
/// Uses the same syntax as `format`.
///
/// If you enable feature `color`, prints in bold red.
#[cfg(feature = "color")]
#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
        std::panic::panic_any($crate::ErrataPanic(format!("\x1b[38;5;1m\x1b[1m{}\x1b[0m", format!($($arg)*))))
    }
}

/// The trait providing [`fail`](FallibleExt::fail).
/// Implemented for `Option<T>` and `Result<T, E: Display>`.
pub trait FallibleExt<T> {
    /// Exits the program cleanly, calling destructors and printing an error message.
    ///
    /// If you enable feature `color`, prints in bold red.
    ///
    /// Usage:
    /// ```no_run
    /// # use errata::FallibleExt;
    /// let bad: Option<i32> = None;
    ///
    /// // Prints the text verbatim to stderr, then exits with code 1.
    /// bad.fail("Expected bad to contain a value");
    /// ```
    fn fail(self, msg: impl Display) -> T;
}

impl<T> FallibleExt<T> for Option<T> {
    #[cfg(not(feature = "color"))]
    fn fail(self, msg: impl Display) -> T {
        match self {
            Some(t) => t,
            None => std::panic::panic_any(ErrataPanic(format!("{msg}"))),
        }
    }

    #[cfg(feature = "color")]
    fn fail(self, msg: impl Display) -> T {
        match self {
            Some(t) => t,
            None => std::panic::panic_any(ErrataPanic(format!("\x1b[38;5;1m\x1b[1m{msg}\x1b[0m"))),
        }
    }
}

// TODO: should there also be impl for E: !Display?
impl<T, E: Display> FallibleExt<T> for Result<T, E> {
    #[cfg(not(feature = "color"))]
    fn fail(self, msg: impl Display) -> T {
        match self {
            Ok(t) => t,
            Err(e) => std::panic::panic_any(ErrataPanic(format!("{msg}: {e}"))),
        }
    }

    #[cfg(feature = "color")]
    fn fail(self, msg: impl Display) -> T {
        match self {
            Ok(t) => t,
            Err(e) => {
                std::panic::panic_any(ErrataPanic(format!("\x1b[38;5;1m\x1b[1m{msg}: {e}\x1b[0m")))
            }
        }
    }
}
