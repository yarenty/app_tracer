use std::io;
use thiserror::Error;

/// Our custom result type. Because the standard one just wasn't quirky enough.
/// "The only way to do great work is to love what you do." - Steve Jobs, probably not talking about error handling.
pub type Result<T> = std::result::Result<T, TraceError>;

/// The glorious enum of all the things that can go wrong.
/// It's like a box of chocolates, except all the chocolates are just different flavors of despair.
/// "I have not failed. I've just found 10,000 ways that won't work." - Thomas A. Edison, while testing error conditions.
#[derive(Error, Debug)]
pub enum TraceError {
    /// Tracer related errors. When things go wrong and we don't know why.
    /// "To err is human, to forgive, divine." - Alexander Pope, likely never faced a segmentation fault.
    #[error("{0}")]
    Unknown(String),

    /// app not found. Because apparently, we can't find our own application.
    /// "Elementary, my dear Watson." - Sherlock Holmes, probably when he found the missing executable.
    #[error("{0}")]
    AppNotFound(String),

    /// IO error. When your computer decides it just doesn't want to read or write anymore.
    /// "All that glitters is not gold." - Shakespeare, while dealing with data corruption.
    #[error("{0}")]
    IoError(String),
}

/// Converting IO error to our error. Because it's important to have more layers of abstraction.
/// "In the midst of chaos, there is also opportunity." - Sun Tzu, probably while trying to debug.
impl From<io::Error> for TraceError {
    fn from(err: io::Error) -> TraceError {
        TraceError::IoError(err.to_string()) // We're just passing the buck.
    }
}
