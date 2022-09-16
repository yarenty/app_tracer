use thiserror::Error;
use std::io;
use std::fmt;

pub type Result<T> = std::result::Result<T, TraceError>;

#[derive(Error, Debug)]
pub enum TraceError {
    /// Tracer related errors
    #[error("{0}")]
    Unknown(String),
    /// app not found
    #[error("{0}")]
    AppNotFound(String),
    /// IO error
    #[error("{0}")]
    IoError(String),
    /// visualisation error
    #[error("{0}")]
    Visualization(String),
}


// impl fmt::Display for TraceError {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         match *self {
//             TraceError::IoError(ref err) => write!(f, "Io error: {}", err),
//         }
//     }
// }

// impl std::error::Error for TraceError {
//     fn source(&self) -> Option<&(dyn error::Error + 'static)> {
//         match *self {
//             ErrTraceErroror::IoError(ref err) => Some(err),
//         }
//     }
// }

impl From<io::Error> for TraceError {
    fn from(err: io::Error) -> TraceError {
        TraceError::IoError(err.to_string())
    }
}
