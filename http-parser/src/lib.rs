//! The HTTP parser.
//!
//! Provides utilities, types, traits, structs, and other useful tools for HTTP parsing.

use error::ParserError;
use std::io::{BufRead, BufReader};

pub mod error;

/// The HTTP method of `Request`.
#[derive(Debug)]
pub enum Method {
    /// The GET method.
    GET,
    /// The POST method.
    POST,
    /// The DELETE method.
    DELETE,
    /// The PATCH method.
    PATCH,
    /// The CONNECT method.
    CONNECT,
    /// The OPTIONS method.
    OPTIONS,
    /// The PUT method.
    PUT,
    /// The HEAD method.
    HEAD,
}

/// Represents the client request.
#[derive(Debug)]
pub struct Request {
    method: Method,
    host: String,
    user_agent: String,
    accept: String,
}

impl Request {
    /// Retrieves the HTTP method.
    pub fn method(&self) -> &Method {
        &self.method
    }

    /// Retrieves the host of request.
    pub fn host(&self) -> &String {
        &self.host
    }

    /// Retrieves the agent of request.
    pub fn user_agent(&self) -> &String {
        &self.user_agent
    }

    /// Retrieves the `accept` field from request.
    pub fn accept_field(&self) -> &String {
        &self.accept
    }
}

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

        Ok(Request {
            method,
            host,
            user_agent,
            accept
        })
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
