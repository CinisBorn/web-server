//! The HTTP parser.
//!
//! Provides utilities, types, traits, structs, and other useful tools for HTTP parsing.

pub mod request;

use crate::request::{Method, Request, RequestStartLine};
use error::ParserError;
use std::collections::HashMap;
use std::io::ErrorKind::{Interrupted};
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
pub fn parse_req<T>(source: &mut T, buf: &mut [u8]) -> Result<Request, ParserError>
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
                    cursor += 2;
                    break;
                }
            },
            None => {
                read_stream(source, buf)?;
            }
        }
    }

    let mut buf = buf.to_vec();
    buf.reverse();
    buf.truncate(cursor);
    
    let body = match read_body(&buf, &headers)  {
        Ok(body) => body,
        Err(err) => match err {
             ParserError::Ignored => String::new(),
             _ => return Err(err)
        }
    };
    
    Ok(Request::new(req_start_line, headers, body))
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
    }

    let Some(colon_position) = line.iter().position(|byte| *byte == b':') else {
        return Parser::Invalid(ParserError::InvalidInput);
    };

    let field = &line[..colon_position];
    let value = &line[colon_position + 2..];

    let Ok(field) = std::str::from_utf8(field) else {
        return Parser::Invalid(ParserError::InvalidInput);
    };

    let Ok(value) = std::str::from_utf8(value) else {
        return Parser::Invalid(ParserError::InvalidInput);
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

    let Ok(method) = std::str::from_utf8(method) else {
        return Err(ParserError::InvalidInput);
    };

    let Ok(target) = std::str::from_utf8(target) else {
        return Err(ParserError::InvalidInput);
    };

    let method = map_method(method)?;
    let req_start_line = RequestStartLine::new(method, target.to_owned());

    Ok(req_start_line)
}

fn read_body(mut source: &[u8], headers: &HashMap<String, String>) -> Result<String, ParserError> {
    let Some(content_length) = headers.get("Content-length") else {
        return Err(ParserError::Ignored);
    };

    let Ok(content_length) = content_length.parse::<usize>() else {
        return Err(ParserError::InvalidInput);
    };

    let mut buf = vec![0u8; content_length];
    _ = source.read_exact(&mut buf);
    
    buf.reverse();

    let Ok(body_txt) = String::from_utf8(buf)  else {
        return Err(ParserError::InvalidInput)
    };

    Ok(body_txt)
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
