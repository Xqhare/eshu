use std::fmt;

pub type EshuResult<T> = Result<T, EshuError>;

/// The error type
#[derive(Debug)]
pub enum EshuError {
    /// Generic error; Only for internal / development use
    Generic(String),
    /// Storage error
    Storage(String),
    /// Empty string
    EmptyString(String),
    /// Invalid name
    InvalidName(String),
    /// No flags or commands during `Cli::parse` validation pass
    NoFlagsOrCommands,
    /// I-O error wrapper
    Io(std::io::Error),
    /// Parsing error wrapper
    Parsing(Parsing),
}

/// Parsing error
#[derive(Debug)]
pub enum Parsing {
    /// Stray dash
    StrayDash,
    /// Unknown argument
    UnknownArg(String),
}

impl fmt::Display for Parsing {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Parsing::StrayDash => write!(f, "Stray dash"),
            Parsing::UnknownArg(arg) => write!(f, "Unknown argument: {}", arg),
        }
    }
}

impl fmt::Display for EshuError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EshuError::Generic(msg) => write!(f, "{}", msg),
            EshuError::NoFlagsOrCommands => {
                write!(f, "No flags or commands set, add at least one.")
            }
            EshuError::InvalidName(msg) => write!(f, "{}", msg),
            EshuError::Parsing(msg) => write!(f, "{}", msg),
            EshuError::Storage(msg) => write!(f, "{}", msg),
            EshuError::EmptyString(msg) => write!(f, "{}", msg),
            EshuError::Io(err) => write!(f, "{}", err),
        }
    }
}

impl std::error::Error for EshuError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            EshuError::Io(err) => Some(err),
            _ => None,
        }
    }
}

impl From<std::io::Error> for EshuError {
    fn from(err: std::io::Error) -> Self {
        EshuError::Io(err)
    }
}
