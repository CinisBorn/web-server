//! The HTTP parser.
//!
//! Provides utilities, types, traits, structs, and other useful tools for HTTP parsing.

use std::collections::HashMap;
use std::io::ErrorKind::Interrupted;
use std::io::{Read, Write};

use error::ParserErr;
use helpers::{map_method, find_line};
use request::{Method, Request, RequestLine, HttpVersion};

mod helpers;
pub mod error;
pub mod request;
pub mod response;

/// Parses a incoming request.
///
/// # Errors
///
/// Returns a [`ParserErr`] if it's not possible to parse the request.
pub fn parse_req<T>(src: &mut T, content: &mut [u8]) -> Result<Request, ParserErr> 
where
  T: Read + Write  
{
    read_src(src, content)?;
    let mut cursor = 0;

    let start_line = parse_req_line(src, content, &mut cursor)?;

    if let HttpVersion::Unsupported(version) = start_line.version() {
        return Err(ParserErr::UnsupportedHttpVersion(version.to_owned()))
    }
    
    let headers = parse_headers(src, content, &mut cursor)?;
    let body = match read_body(src, content, &mut cursor, &headers) {
        Ok(body) => body,
        Err(err) => match err {
            ParserErr::Ignored => String::new(),
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
/// Returns [`ParserErr::ConnClosed`] if the client closes the connection.
///
/// Returns [`ParserErr::Io(n)`] if any other I/O error occurs.
///
/// It will retry if an [`ErrorKind::Interrupted`] occurs.
fn read_src<T>(src: &mut T, buf: &mut [u8]) -> Result<usize, ParserErr>
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

fn parse_headers<T>(
    src: &mut T,
    content: &mut [u8],
    cursor: &mut usize,
) -> Result<HashMap<String, String>, ParserErr>
where
    T: Read + Write,
{
    let mut headers = HashMap::new();

    loop {
        if let Some(line) = find_line(&content[*cursor..]) {
            *cursor += line.len() + 2;

            if line.is_empty() {
                break Ok(headers);
            }

            let Some(colon_position) = line.iter().position(|byte| *byte == b':') else {
                return Err(ParserErr::InvalidInput);
            };

            let field = &line[..colon_position];
            let value = &line[colon_position + 2..];

            let Ok(field) = std::str::from_utf8(field) else {
                return Err(ParserErr::InvalidInput);
            };

            let Ok(value) = std::str::from_utf8(value) else {
                return Err(ParserErr::InvalidInput);
            };

            headers.insert(field.to_owned(), value.to_owned());
        } else {
            read_src(src, content)?;
        }
    }
}

fn parse_req_line<T>(
    src: &mut T,
    content: &mut [u8],
    cursor: &mut usize,
) -> Result<RequestLine, ParserErr>
where
    T: Read + Write,
{
    let line = loop {
        if let Some(line) = find_line(&content[*cursor..]) {
            *cursor += line.len() + 2;
            break line;
        }

        read_src(src, content)?;
    };

    let mut items = line.split(|byte| *byte == b' ');

    let method  = items.next().ok_or(ParserErr::InvalidInput)?;
    let target  = items.next().ok_or(ParserErr::InvalidInput)?;
    let version = items.next().ok_or(ParserErr::InvalidInput)?;

    let Ok(method) = std::str::from_utf8(method) else {
        return Err(ParserErr::InvalidInput);
    };

    let Ok(target) = std::str::from_utf8(target) else {
        return Err(ParserErr::InvalidInput);
    };

    let Ok(version) = std::str::from_utf8(version) else {
        return Err(ParserErr::InvalidInput);
    };

    let method = map_method(method)?;
    let version = HttpVersion::map(version);
    
    let req_start_line = RequestLine::new(version, method, target.to_owned());

    Ok(req_start_line)
}

fn read_body<T>(
    src: &mut T,
    content: &mut [u8],
    cursor: &mut usize,
    headers: &HashMap<String, String>,
) -> Result<String, ParserErr>
where
    T: Read + Write,
{
    let Some(content_length) = headers.get("Content-length") else {
        return Err(ParserErr::Ignored);
    };

    let Ok(content_length) = content_length.parse::<usize>() else {
        return Err(ParserErr::InvalidInput);
    };

    let available_data = &mut content[*cursor..];
    let from_buffer = available_data.len().min(content_length);
    let remaining = content_length - from_buffer;
    let mut body = vec![0u8; content_length];

    body[..from_buffer].copy_from_slice(&available_data[..from_buffer]);

    if remaining > 0 {
        src.read_exact(&mut body[from_buffer..]).map_err(ParserErr::Io)?;
    }

    let Ok(body_txt) = String::from_utf8(body) else {
        return Err(ParserErr::InvalidInput);
    };

    Ok(body_txt)
}
