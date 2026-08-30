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
    headers: ReqHeaders,
}

/// The headers of request.
#[derive(Debug)]
pub struct ReqHeaders {
    host: Option<String>,
    user_agent: Option<String>,
    accept: Option<String>
}

impl ReqHeaders {
    /// Creates a new instance of `ReqHeaders`.
    pub fn new() -> Self {
        Self { 
            host: None, 
            user_agent: None, 
            accept: None
        }
    }

    /// Sets the host of request. 
    pub fn set_host(&mut self, value: &str)  {
        self.host = Some(value.to_string())
    }

    /// Returns the host of request.
    pub fn host(&self) -> Option<&str> {
        self.host.as_deref()
    }

    /// Sets the user agent of request. 
    pub fn set_user_agent(&mut self, value: &str) {
        self.user_agent = Some(value.to_string())
    }
    
    /// Returns the user agent of request.
    pub fn user_agent(&self) -> Option<&str> {
        self.user_agent.as_deref()
    }

    /// Sets the value of accept header.
    pub fn set_accept(&mut self, value: &str) {
        self.accept = Some(value.to_string())
    }
    
    /// Returns the value of accept header.
    pub fn accept(&self) -> Option<&str> {
        self.accept.as_deref()
    }
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
    pub fn new(start_line: RequestStartLine, headers: ReqHeaders) -> Self {
        Self { start_line, headers }
    }

    /// Returns the request's  start line. 
    pub fn start_line(&self) -> &RequestStartLine {
        &self.start_line
    }

    /// Returns a `ReqHeaders` containing headers of request
    pub fn headers(&self) -> &ReqHeaders {
        &self.headers
    }
}
