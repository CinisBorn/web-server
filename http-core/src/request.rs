//! Request types, utilities and tools.

use std::collections::HashMap;

use crate::Method;

/// The available supported HTTP methods.
#[derive(Debug)]
pub enum HttpVersion {
    /// Represents the HTTP/1.1
    ///
    /// Older version won't be supported.
    V1, // aka HTTP/1.1
    /// Represents the unsupported http version.
    Unsupported(String),
}

impl HttpVersion {
    /// Maps a string into [`HttpVersion`]
    pub fn map(version: &str) -> Self {
        match version {
            "HTTP/1.1" => HttpVersion::V1,
            v => HttpVersion::Unsupported(v.to_string()), // v = version
        }
    }
}

#[derive(Debug)]
/// The request start line. See more at
/// [RFC 9112](https://www.rfc-editor.org/rfc/rfc9112.html#name-message)
///
/// The HTTP version isn't necessary because this parser will forever support only http/1.1
pub struct RequestLine {
    version: HttpVersion,
    method: Method,
    target: String,
}

impl RequestLine {
    /// Creates a new instance of `RequestLine`.
    pub fn new(version: HttpVersion, method: Method, target: String) -> Self {
        Self {
            version,
            method,
            target,
        }
    }

    /// Returns the HTTP method.
    pub fn method(&self) -> &Method {
        &self.method
    }

    /// Returns the target resource.
    pub fn target(&self) -> &str {
        &self.target
    }

    /// Returns the HTTP version of request
    pub fn version(&self) -> &HttpVersion {
        &self.version
    }
}

/// Represents the client request.
#[derive(Debug)]
pub struct Request {
    start_line: RequestLine,
    headers: Option<HashMap<String, String>>,
    body: Option<String>,
}

impl Request {
    /// Creates a new structured client `Request`.
    pub fn new(
        start_line: RequestLine,
        headers: Option<HashMap<String, String>>,
        body: Option<String>,
    ) -> Self {
        Self {
            start_line,
            headers,
            body,
        }
    }

    /// Returns the request's  start line.
    pub fn start_line(&self) -> &RequestLine {
        &self.start_line
    }

    /// Returns the headers of request
    pub fn headers(&self) -> Option<&HashMap<String, String>> {
        self.headers.as_ref()
    }

    /// Returns the body of request.
    pub fn body(&self) -> Option<&String> {
        self.body.as_ref()
    }

    /// Returns the connection header of request if any.
    pub fn connection(&self) -> Option<&str> {
        if let Some(headers) = self.headers() {
            headers.get("Connection").map(|v| v.as_ref())            
        } else {
            None
        }
    }
}
