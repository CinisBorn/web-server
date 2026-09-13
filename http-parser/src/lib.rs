//! The HTTP parser.
//!
//! Provides utilities, types, traits, structs, and other useful tools for HTTP parsing.

use std::collections::HashMap;
use std::io::ErrorKind::Interrupted;
use std::io::{Read, Write};

use error::ParserError;
use helpers::map_method;
use request::{Method, Request, RequestStartLine};

pub mod error;
mod helpers;
pub mod request;

/// Parses a incoming request.
///
/// # Errors
///
/// Returns a [`ParserError`] if it's not possible to parse the request.
pub fn parse_req<T>(source: &mut T, buf: &mut [u8]) -> Result<Request, ParserError>
where
    T: Read + Write,
{
    read_stream(source, buf)?;
    let mut cursor = 0;

    let start_line = parse_req_start_line(source, buf, &mut cursor)?;
    let headers = parse_header_line(source, buf, &mut cursor)?;

    let body = match read_body(&buf[cursor..], &headers) {
        Ok(body) => body,
        Err(err) => match err {
            ParserError::Ignored => String::new(),
            _ => return Err(err),
        },
    };

    Ok(Request::new(start_line, headers, body))
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

fn parse_header_line<T>(
    source: &mut T,
    buf: &mut [u8],
    cursor: &mut usize,
) -> Result<HashMap<String, String>, ParserError>
where
    T: Read + Write,
{
    let mut headers = HashMap::new();

    loop {
        if let Some(line) = find_line(&buf[*cursor..]) {
            *cursor += line.len() + 2;

            if line.is_empty() {
                dbg!(&headers);
                break Ok(headers);
            };

            let Some(colon_position) = line.iter().position(|byte| *byte == b':') else {
                return Err(ParserError::InvalidInput);
            };

            let field = &line[..colon_position];
            let value = &line[colon_position + 2..];

            let Ok(field) = std::str::from_utf8(field) else {
                return Err(ParserError::InvalidInput);
            };

            let Ok(value) = std::str::from_utf8(value) else {
                return Err(ParserError::InvalidInput);
            };

            headers.insert(field.to_owned(), value.to_owned());
        } else {
            read_stream(source, buf)?;
        }
    }
}

fn find_line(buf: &[u8]) -> Option<&[u8]> {
    buf.windows(2)
        .position(|w| w == b"\r\n")
        .map(|pos| &buf[..pos])
}

fn parse_req_start_line<T>(
    source: &mut T,
    buf: &mut [u8],
    cursor: &mut usize,
) -> Result<RequestStartLine, ParserError>
where
    T: Read + Write,
{
    let line = loop {
        if let Some(b_line) = find_line(&buf[*cursor..]) {
            *cursor += b_line.len() + 2;
            break b_line;
        }

        read_stream(source, buf)?;
    };

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

    let Ok(body_txt) = String::from_utf8(buf) else {
        return Err(ParserError::InvalidInput);
    };

    Ok(body_txt)
}
