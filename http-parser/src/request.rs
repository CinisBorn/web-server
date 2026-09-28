//! Provides tools for client request handling

use std::collections::HashMap;
use crate::types::Method;

mod header_section;
mod body;

/// The available supported HTTP methods. 
#[derive(Debug)]
pub enum HttpVersion {
    /// Represents the HTTP/1.1
    /// 
    /// Older version won't be supported.
    V1, // aka HTTP/1.1 
    /// Represents the unsupported http version. 
    Unsupported(String)
}

impl HttpVersion {
    /// Maps a string into [`HttpVersion`]
    pub fn map(version: &str) -> Self {
        match version {
            "HTTP/1.1" => HttpVersion::V1,
            v => HttpVersion::Unsupported(v.to_string()) // v = version
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
        Self { version, method, target }
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

impl Request
 {
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
}

use std::io::{Read, Write};
use std::io::ErrorKind::Interrupted;

use crate::error::ParserErr;
/// Attempts to read bytes from a stream.
///
/// Returns `Ok(&[n])` if one or more bytes are successfully read.
///
/// # Errors
///
/// Returns [`ParserErr::ConnClosed`] if the client closes the connection.
///
/// Returns [`ParserErr::Io(n)`] if any other I/O error occurs.
///
/// It will retry if an [`ErrorKind::Interrupted`] occurs.
pub(crate) fn read_src<T>(src: &mut T, buf: &mut [u8]) -> Result<usize, ParserErr>
where
    T: Read + Write,
{
    loop {
        match src.read(buf) {
            Ok(0) => return Err(ParserErr::ConnClosed),
            Ok(n) => return Ok(n),
            Err(e) if e.kind() == Interrupted => {}
            Err(e) => return Err(ParserErr::Io(e)),
        }
    }
}

use header_section::parse_headers;
use header_section::parse_req_line;
use body::read_body;

/// Parses a incoming request.
///
/// # Errors
///
/// Returns a [`ParserErr`] if it's not possible to parse the request.
pub fn parse_req<T>(src: &mut T, content: &mut [u8]) -> Result<Request, ParserErr>
where
    T: Read + Write,
{
    read_src(src, content)?;
    let mut cursor = 0;

    let start_line = parse_req_line(src, content, &mut cursor)?;

    if let HttpVersion::Unsupported(version) = start_line.version() {
        return Err(ParserErr::UnsupportedHttpVersion(version.to_owned()));
    }

    let headers = parse_headers(src, content, &mut cursor)?;
    let body = read_body(src, content, &mut cursor, headers.as_ref())?;

    Ok(Request::new(start_line, headers, body))
}