//! The HTTP parser.
//!
//! Provides utilities, types, traits, structs, and other useful tools for HTTP parsing.

pub mod request;

use crate::request::{Method, Request, RequestStartLine};
use error::ParserError;
use std::collections::HashMap;
use std::io::ErrorKind::Interrupted;
use std::io::{Read, Write};

pub mod error;

enum Parser {
    Complet,
    Incomplet,
    Invalid(ParserError),
}

/// Parses a incoming request.
///
/// # Errors
///
/// Returns a [`ParserError`] if it's not possible to parse the request.
pub fn parse_req<'a, T>(source: &mut T, buf: &'a mut [u8]) -> Result<Request, ParserError>
where
    T: Read + Write,
{
    let _ = read_stream(source, buf)?;
    let req_start_line;
    let mut cursor = 0;

    loop {
        let line = find_line(&buf[cursor..]);

        match line {
            Some(v) => {
                let res = parse_req_start_line(v)?;
                let read = v.len();
                req_start_line = res;
                cursor += read + 2;

                break;
            }
            None => {
                let _ = read_stream(source, buf)?;
            }
        }
    }

    let mut headers: HashMap<String, String> = HashMap::new();

    loop {
        let line = find_line(&buf[cursor..]);

        match line {
            Some(line) => match parse_header_line(line, &mut headers) {
                Parser::Incomplet => {
                    let read = line.len();
                    cursor += read + 2;
                }
                Parser::Invalid(err) => return Err(err),
                Parser::Complet => {
                    println!("FINISHING");
                    break;
                }
            },
            None => {
                read_stream(source, buf)?;
            }
        };
    }

    Ok(Request::new(req_start_line, headers))
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
    T: Read + Write,
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

fn parse_header_line(line: &[u8], headers: &mut HashMap<String, String>) -> Parser {
    if line.is_empty() {
        return Parser::Complet;
    };

    let colon_position = match line.iter().position(|byte| *byte == b':') {
        Some(position) => position,
        None => return Parser::Invalid(ParserError::InvalidInput),
    };

    let field = &line[..colon_position];
    let value = &line[colon_position + 1..];

    let field = match std::str::from_utf8(field) {
        Ok(value) => value,
        Err(_) => return Parser::Invalid(ParserError::InvalidInput),
    };

    let value = match std::str::from_utf8(value) {
        Ok(value) => value.trim_start(),
        Err(_) => return Parser::Invalid(ParserError::InvalidInput),
    };

    headers.insert(field.to_owned(), value.to_owned());

    Parser::Incomplet
}

fn find_line(buf: &[u8]) -> Option<&[u8]> {
    buf.windows(2)
        .position(|w| w == b"\r\n")
        .map(|pos| &buf[..pos])
}

fn parse_req_start_line(line: &[u8]) -> Result<RequestStartLine, ParserError> {
    let mut items = line.split(|byte| *byte == b' ');

    let method = items.next().ok_or(ParserError::InvalidInput)?;
    let target = items.next().ok_or(ParserError::InvalidInput)?;

    let method = match std::str::from_utf8(method) {
        Ok(method) => method,
        Err(_) => return Err(ParserError::InvalidInput),
    };

    let target = match std::str::from_utf8(target) {
        Ok(target) => target,
        Err(_) => return Err(ParserError::InvalidInput),
    };

    let method = map_method(&method)?;
    let target = target;
    let req_start_line = RequestStartLine::new(method, target.to_owned());

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
