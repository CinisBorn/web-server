//! Parser errors
//!
//! This module provides error types and utilities for parser error handling.

/// The parser errors
#[derive(Debug)]
pub enum ParserError {
    ///  A wrapper around of `std::io::Error`.
    Io(std::io::Error),
    /// The connection was closed.
    ///
    /// This error occurs when the client close the connection when the server is waiting
    /// for new incoming data.
    ConnClosed,
    /// It's a temporary measure why the understading of "unknown method" isn't well defined.
    /// It's very common costumized methods or fields in HTTP requests because HTTP is extensible by design.
    UnknownMethod(String),
    /// Unexpected input was found. 
    InvalidInput,
}

impl std::error::Error for ParserError {}
impl std::fmt::Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParserError::UnknownMethod(m) => write!(f, "{m}"),
            ParserError::ConnClosed => write!(f, "Connection was closed"),
            ParserError::Io(m) => write!(f, "{m}"),
            ParserError::InvalidInput => write!(f, "Unexpected input")
        }
    }
}

impl From<std::io::Error> for ParserError {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}
