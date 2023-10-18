//! Print panics in a sensible fashion.
//!
//! First, place the following before your `main` function:
//! ```rust
//! use anerror::FallibleExt;
//!
//! #[anerror::catch]
//! fn main() {
//!     // ...
//! }
//! ```
//!
//! This unlocks pretty-printing for your error messages. For `Option` and
//! `Result` (as long as the `Err` variant implements `Display`), you can use
//! [`fail`](FallibleExt::fail) or [`fail_color`](FallibleExt::fail_color) as
//! drop-in replacements for `expect`. You can continue to use `unwrap` and
//! `expect` where you don't expect any errors, and you will continue to get
//! useful debug information just like normal.
//!
//! If you wish to throw your own errors, see [`error`] and [`error_color`].

use std::fmt::Display;

pub use anerror_error::AnerrorPanic;
pub use anerror_macros::catch;

/// Exits the program cleanly, calling destructors and printing an error message.
/// Uses the same syntax as `format`.
#[macro_export]
macro_rules! error {
    ($($arg:tt),*) => {
        std::panic::panic_any($crate::AnerrorPanic(format!($($arg),*)));
    }
}

/// Exits the program cleanly, calling destructors and printing an error message
/// in bold red. Uses the same syntax as `format`.
#[macro_export]
macro_rules! error_color {
    ($($arg:tt),*) => {
        std::panic::panic_any($crate::AnerrorPanic(format!("\x1b[38;5;1m\x1b[1m{}\x1b[0m", format!($($arg),*))));
    }
}

/// The trait providing [`fail`](FallibleExt::fail) and
/// [`fail_color`](FallibleExt::fail_color). Implemented for `Option<T>` and
/// `Result<T, E: Display>`.
pub trait FallibleExt<T> {
    /// Exits the program cleanly, calling destructors and printing an error message.
    ///
    /// Usage:
    /// ```no_run
    /// # use anerror::FallibleExt;
    /// let bad: Option<i32> = None;
    ///
    /// // Prints the text verbatim to stderr, then exits with code 1.
    /// bad.fail("Expected bad to contain a value");
    /// ```
    fn fail(self, msg: impl Display) -> T;
    /// Exits the program cleanly, calling destructors and printing an error message
    /// in bold red.
    ///
    /// Usage:
    /// ```no_run
    /// # use anerror::FallibleExt;
    /// let bad: Option<i32> = None;
    ///
    /// // Prints the text in bold red to stderr, then exits with code 1.
    /// bad.fail_color("Expected bad to contain a value");
    /// ```
    fn fail_color(self, msg: impl Display) -> T;
}

impl<T> FallibleExt<T> for Option<T> {
    fn fail(self, msg: impl Display) -> T {
        match self {
            Some(t) => t,
            None => std::panic::panic_any(AnerrorPanic(format!("{msg}"))),
        }
    }

    fn fail_color(self, msg: impl Display) -> T {
        match self {
            Some(t) => t,
            None => std::panic::panic_any(AnerrorPanic(format!("\x1b[38;5;1m\x1b[1m{msg}\x1b[0m"))),
        }
    }
}

// TODO: should there also be impl for E: !Display?
impl<T, E: Display> FallibleExt<T> for Result<T, E> {
    fn fail(self, msg: impl Display) -> T {
        match self {
            Ok(t) => t,
            Err(e) => std::panic::panic_any(AnerrorPanic(format!("{msg}: {e}"))),
        }
    }

    fn fail_color(self, msg: impl Display) -> T {
        match self {
            Ok(t) => t,
            Err(e) => std::panic::panic_any(AnerrorPanic(format!(
                "\x1b[38;5;1m\x1b[1m{msg}: {e}\x1b[0m"
            ))),
        }
    }
}
