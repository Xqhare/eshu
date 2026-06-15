use std::fmt;
use nemesis::NemesisError;

/// Crate-level Result type using NemesisError
pub type EshuResult<T> = Result<T, NemesisError>;

/// The structured leaf error type for eshu parser/builder operations.
#[derive(Debug)]
pub enum EshuErrorKind {
    /// Generic or internal development error
    Generic(String),
    /// Configuration or storage syntax validation error
    Storage(String),
    /// An empty string was provided where a value was required
    EmptyString(String),
    /// Invalid character or format in program, flag, or subcommand name
    InvalidName(String),
    /// Developer configuration error: no flags or subcommands registered on a non-basic CLI
    NoFlagsOrCommands,
    /// One or more unknown arguments were passed on the command line
    UnknownArgument(String),
    /// A flag requiring an argument was passed without one
    MissingArgument {
        flag: String,
        expected_syntax: String,
    },
    /// Wrapper for standard I/O errors
    Io(std::io::Error),
}

impl fmt::Display for EshuErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Generic(msg) | Self::Storage(msg) | Self::EmptyString(msg) | Self::InvalidName(msg) => {
                write!(f, "{}", msg)
            }
            Self::NoFlagsOrCommands => {
                write!(f, "No flags or commands set, add at least one.")
            }
            Self::UnknownArgument(args) => {
                write!(f, "Usage error: Unknown argument(s): {}", args)
            }
            Self::MissingArgument { flag, expected_syntax } => {
                write!(
                    f,
                    "Usage error: Flag '{}' requires an argument. Please provide one via the following syntax: '{}'",
                    flag, expected_syntax
                )
            }
            Self::Io(err) => write!(f, "{}", err),
        }
    }
}

impl std::error::Error for EshuErrorKind {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io(err) => Some(err),
            _ => None,
        }
    }
}

impl From<std::io::Error> for EshuErrorKind {
    fn from(err: std::io::Error) -> Self {
        Self::Io(err)
    }
}
