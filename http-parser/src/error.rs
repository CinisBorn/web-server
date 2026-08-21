//! Parser errors 
//! 
//! This module provides error types and utilities for parser error handling. 

/// The parser errors 
#[derive(Debug)]
pub enum ParserError {
    /// It's a temporary measure why the understading of "unknown method" isn't well defined.
    /// It's very common costumized methods or fields in HTTP requests because HTTP is extensible by design. 
    UnknownMethod(String) 
}

impl std::error::Error for ParserError {}
impl std::fmt::Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParserError::UnknownMethod(m) => write!(f, "{}", m)
        }
    }
}
