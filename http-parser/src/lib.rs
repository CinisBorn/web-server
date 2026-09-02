//! The HTTP parser.
//!
//! Provides utilities, types, traits, structs, and other useful tools for HTTP parsing.

pub mod request;

use crate::request::{Method, ReqHeaders, Request, RequestStartLine};
use error::ParserError;
use std::io::ErrorKind::Interrupted;
use std::io::{BufRead, BufReader, Read, Write};

pub mod error;

/// Parses a incoming request.
///
/// # Errors
///
/// Returns a [`ParserError`] if it's not possible to parse the request.
pub fn parse_req<'a, T>(source: &mut T, buf: &'a mut [u8]) -> Result<Request, ParserError> 
where 
    T: Read + Write
{
    let raw_data = read_stream(source, buf)?;
    let buf_read = BufReader::new(raw_data);
    let mut headers = buf_read
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<String>>();

    let frame_idx = headers.iter().position(|l| l == "");
    let _ = headers.split_off(frame_idx.unwrap());

    let req_start_line = parse_req_start_line(&mut headers)?;
    let req_headers = parse_req_headers(&headers);

    Ok(Request::new(req_start_line, req_headers))
}

/// Attempts to read bytes from a stream.
///
/// Returns `Ok(&[n])` if one or more bytes are successfully read.
///
/// # Errors
///
/// Returns [`ParserError::ConnClosed`] if the client closes the connection.
///
/// Returns [`ParserError::Io(n)`] if any other I/O error occurs.
///
/// It will retry if an [`ErrorKind::Interrupted`] occurs.
fn read_stream<'a, T>(stream: &mut T, buf: &'a mut [u8]) -> Result<&'a [u8], ParserError> 
where 
    T: Read + Write
{
    loop {
        match stream.read(buf) {
            Ok(0) => return Err(ParserError::ConnClosed),
            Ok(n) => return Ok(&buf[..n]),
            Err(e) if e.kind() == Interrupted => {}
            Err(e) => return Err(ParserError::Io(e)),
        }
    }
}

fn parse_req_headers(headers: &Vec<String>) -> ReqHeaders {
    let mut req_headers = ReqHeaders::new();

    for header in headers {
        if header.is_empty() {
            continue;
        }

        let field = header.split(':').collect::<Vec<&str>>();
        let key = field[0];
        let value = field[1].trim();

        // match field key againts supported headers.
        //
        // Unsupported headers will be ignored.
        match key.to_lowercase().as_str() {
            "user-agent" => req_headers.set_user_agent(value),
            "accept" => req_headers.set_accept(value),
            "host" => req_headers.set_host(value),
            _ => continue,
        }
    }

    req_headers
}

fn parse_req_start_line(headers: &mut Vec<String>) -> Result<RequestStartLine, ParserError> {
    use std::string::ToString;

    let method = headers.remove(0);
    let mut start_line = method
        .split_ascii_whitespace()
        .map(ToString::to_string)
        .collect::<Vec<String>>();
    let method = map_method(&start_line[0])?;
    let target = start_line[1].clone();
    let req_start_line = RequestStartLine::new(method, target);

    start_line.clear();

    Ok(req_start_line)
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
