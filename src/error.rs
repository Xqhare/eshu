use std::fmt;

pub type EshuResult<T> = Result<T, EshuError>;

#[derive(Debug)]
pub enum EshuError {
    Generic(String),
    Storage(String),
    EmptyString(String),
    InvalidName(String),
    Io(std::io::Error),
}

impl fmt::Display for EshuError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EshuError::Generic(msg) => write!(f, "{}", msg),
            EshuError::InvalidName(msg) => write!(f, "{}", msg),
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
