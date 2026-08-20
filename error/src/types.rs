//! Provides error types.
//! 
//! Unrecoverable errors carry a descriptive message, while recoverable errors do not. 
//! The rationale is that recoverable tipically don't need require additional context beyond their variant,
//!  whereas unrecoverable errors need to provide information about the cause of failure.
use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub struct ServerError {
    cause: ErrorCause
}

impl Error for ServerError {}
impl Display for ServerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.cause {
            ErrorCause::Connection(e) => {
                match e {
                    ConnectionError::InvalidEnv(message) => write!(f, "{}", message),
                }
            },
            ErrorCause::Parser(e) => {
                match e {
                    ParserError::UnknownMethod(message) => write!(f, "{}", message)
                }
            },
            ErrorCause::Io(e) => write!(f, "{}", e.to_string()),
        } 
    }
}

impl From<std::io::Error> for ServerError {
    fn from(value: std::io::Error) -> Self {
        Self { cause: ErrorCause::Io(value) }
    }
}

impl From<ParserError> for ServerError {
    fn from(value: ParserError) -> Self {
        Self { cause: ErrorCause::Parser(value) }
    }
}

#[allow(dead_code)]
#[derive(Debug)]
enum ErrorCause {
    Connection(ConnectionError),
    Io(std::io::Error),
    Parser(ParserError)
}
#[allow(dead_code)]
#[derive(Debug)]
enum ConnectionError {
    InvalidEnv(String),
}

#[derive(Debug)]
pub enum ParserError {
    /// It's a temporary measure why the understading of "unknown method" isn't well defined.
    /// It's very common costumized methods or fields in HTTP requests because HTTP is extensible by design. 
    UnknownMethod(String) 
}
