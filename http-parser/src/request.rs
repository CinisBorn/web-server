//! Provides tools for client request handling

use std::collections::HashMap;

/// The HTTP method of `Request`.
#[derive(Debug, PartialEq, Eq)]
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
}

/// Represents the client request.
#[derive(Debug)]
pub struct Request {
    start_line: ReqStartLine,
    headers: HashMap<String, String>,
    body: String,
}

#[derive(Debug)]
/// The request start line. See more at
/// [RFC 9112](https://www.rfc-editor.org/rfc/rfc9112.html#name-message)
///
/// The HTTP version isn't necessary because this parser will forever support only http/1.1
pub struct ReqStartLine {
    method: Method,
    target: String,
}

impl ReqStartLine {
    /// Creates a new instance of `ReqStartLine`.
    pub fn new(method: Method, target: String) -> Self {
        Self { method, target }
    }

    /// Returns the HTTP method.
    pub fn method(&self) -> &Method {
        &self.method
    }

    /// Returns the target resource.
    pub fn target(&self) -> &str {
        &self.target
    }
}

impl Request
 {
    /// Creates a new structured client `Request`.
    pub fn new(
        start_line: ReqStartLine,
        headers: HashMap<String, String>,
        body: String,
    ) -> Self {
        Self {
            start_line,
            headers,
            body,
        }
    }

    /// Returns the request's  start line.
    pub fn start_line(&self) -> &ReqStartLine {
        &self.start_line
    }

    /// Returns the headers of request
    pub fn headers(&self) -> &HashMap<String, String> {
        &self.headers
    }

    /// Returns the body of request.
    pub fn body(&self) -> &str {
        &self.body
    }
}
