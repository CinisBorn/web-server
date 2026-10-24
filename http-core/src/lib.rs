//! Contains core http types and tools.

pub mod request;
pub mod response;

/// The HTTP method of `Request`.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[allow(clippy::upper_case_acronyms)]
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
    /// Represents unsupported HTTP methods
    Unsupported,
}

impl Method {
    /// Maps the method
    pub fn map_method(method: &str) -> Method {
        match method {
            "GET" => Method::GET,
            "HEAD" => Method::HEAD,
            _ => Method::Unsupported,
        }
    }

    /// Converts a [`Method`] into [`&str`]
    pub fn to_str(&self) -> &str {
        match self {
            Method::CONNECT => "CONNECT",
            Method::DELETE => "DELETE",
            Method::GET => "GET",
            Method::HEAD => "HEAD",
            Method::OPTIONS => "OPTIONS",
            Method::PATCH => "PATCH",
            Method::POST => "POST",
            Method::PUT => "PUT",
            Method::Unsupported => "UNSUPPORTED",
        }
    }
}
