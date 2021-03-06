extern crate memchr;
extern crate regex;
extern crate regex_syntax as syntax;

use std::error;
use std::fmt;
use std::io;
use std::result;

pub use search::{Grep, GrepBuilder};

mod literals;
mod nonl;
mod search;

/// Result is a convenient type alias that fixes the type of the error to
/// the `Error` type defined in this crate.
pub type Result<T> = result::Result<T, Error>;

/// Error enumerates the list of possible error conditions when building or
/// using a `Grep` line searcher.
#[derive(Debug)]
pub enum Error {
    /// An error from parsing or compiling a regex.
    Regex(regex::Error),
    /// This error occurs when an illegal literal was found in the regex
    /// pattern. For example, if the line terminator is `\n` and the regex
    /// pattern is `\w+\n\w+`, then the presence of `\n` will cause this error.
    LiteralNotAllowed(char),
    /// This errors occurs when a line exceeds the buffer size. The buffer
    /// size is given.
    LineTooLong(usize),
    /// An IO error occurred while searching.
    Io(io::Error),
    /// An unused enum variant that indicates this enum may be expanded in
    /// the future and therefore should not be exhaustively matched.
    #[doc(hidden)]
    __Nonexhaustive,
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Regex(ref err) => err.description(),
            Error::LiteralNotAllowed(_) => "use of forbidden literal",
            Error::LineTooLong(_) => "line exceeds buffer size",
            Error::Io(ref err) => err.description(),
            Error::__Nonexhaustive => unreachable!(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            Error::Regex(ref err) => err.cause(),
            Error::Io(ref err) => err.cause(),
            _ => None,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Regex(ref err) => err.fmt(f),
            Error::LiteralNotAllowed(chr) => {
                write!(f, "Literal '{}' not allowed.", chr)
            }
            Error::LineTooLong(limit) => {
                write!(f, "Line exceeded buffer size of {} bytes, try \
                           searching with memory maps instead.", limit)
            }
            Error::Io(ref err) => err.fmt(f),
            Error::__Nonexhaustive => unreachable!(),
        }
    }
}

impl From<regex::Error> for Error {
    fn from(err: regex::Error) -> Error {
        Error::Regex(err)
    }
}

impl From<syntax::Error> for Error {
    fn from(err: syntax::Error) -> Error {
        Error::Regex(regex::Error::Syntax(err))
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::Io(err)
    }
}
