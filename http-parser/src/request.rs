//! Provides tools for client request handling

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
    start_line: RequestStartLine,
    host: String,
    user_agent: String,
    accept: String,
}

#[derive(Debug)]
/// The request start line. See more at
/// [RFC 9112](https://www.rfc-editor.org/rfc/rfc9112.html#name-message)
///
/// The HTTP version isn't necessary because this parser will forever support only http/1.1
pub struct RequestStartLine {
    method: Method,
    target: String,
}

impl RequestStartLine {
    /// Creates a new instance of `RequestStartLine`.
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

impl Request {
    /// Creates a new structured client `Request`.
    pub fn new(
        start_line: RequestStartLine,
        host: String,
        user_agent: String,
        accept: String,
    ) -> Self {
        Self {
            start_line,
            host,
            user_agent,
            accept,
        }
    }

    /// Returns the host of request.
    pub fn host(&self) -> &str {
        &self.host
    }

    /// Returns the user agent of request
    pub fn user_agent(&self) -> &str {
        &self.user_agent
    }

    /// Returns the value of accept header.
    pub fn accept(&self) -> &str {
        &self.accept
    }

    /// Returns the request's  start line. 
    pub fn start_line(&self) -> &RequestStartLine {
        &self.start_line
    }
}
