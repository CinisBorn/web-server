//! The HTTP parser.
//!
//! Provides utilities, types, traits, structs, and other useful tools for HTTP parsing.

mod request;

use crate::request::{Method, Request};
use error::ParserError;
use std::io::{BufRead, BufReader};

pub mod error;

/// The `HTTPParser` trait allows a type parser a HTTP message.
pub trait HttpParser {
    /// Parses a HTTP request returning a `Request` struct.
    /// 
    /// # Errors 
    /// 
    /// Returns a `ParserError` if the method isn't known. See [`Method`]
    fn parser(&self, raw_txt: &[u8]) -> Result<Request, ParserError> {
        let buf = BufReader::new(raw_txt);
        let mut lines = buf
            .lines()
            .map(|line| line.unwrap())
            .collect::<Vec<String>>();

        let method = lines.swap_remove(0);
        let method = method
            .split_ascii_whitespace()
            .map(std::string::ToString::to_string)
            .collect::<Vec<String>>()
            .remove(0);
        let method = map_method(&method)?;

        let mut accept = String::new();
        let mut host = String::new();
        let mut user_agent = String::new();

        for line in lines {
            if line.is_empty() {
                continue;
            }

            let field = line
                .split(':')
                .map(std::string::ToString::to_string)
                .collect::<Vec<String>>();
            let value = field[1].trim().to_string();

            match field[0].to_lowercase().as_str() {
                "user-agent" => user_agent = value,
                "accept" => accept = value,
                "host" => host = value,
                _ => {
                    // It's a temporary measure
                    panic!("Unknown http field");
                }
            }
        }

        Ok(Request::new(method, host, user_agent, accept))
    }
}

// Maps the method from http request to a respective `Enum` variant.
fn map_method(method: &str) -> Result<Method, ParserError> {
    match method {
        "GET" => Ok(Method::GET),
        "POST" => Ok(Method::POST),
        "DELETE" => Ok(Method::DELETE),
        "CONNECT" => Ok(Method::CONNECT),
        "PATCH" => Ok(Method::PATCH),
        "PUT" => Ok(Method::PUT),
        "OPTIONS" => Ok(Method::OPTIONS),
        "HEAD" => Ok(Method::HEAD),
        _ => Err(ParserError::UnknownMethod(method.to_string())),
    }
}
