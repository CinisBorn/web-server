//! Connection error types and utilities
use std::fmt::Debug;

/// The connection error type
#[derive(Debug)]
pub enum ConnectionError {
    /// A wrap around of `std::io::Error`.
    Io(std::io::Error)
}

impl std::error::Error for  ConnectionError {} 
impl std::fmt::Display for ConnectionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConnectionError::Io(message) => write!(f, "{}", message)
        }
    }
}

impl From<std::io::Error> for ConnectionError {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}