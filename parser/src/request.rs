//! Provides tools for client request handling

mod body;
mod header_section;

use http_core::request::Request;
use std::io::{BufReader, Read, Write};

use crate::error::ParserErr;

use body::read_body;
use header_section::parse_headers;
use header_section::parse_req_line;

/// Parses a incoming request.
///
/// # Errors
///
/// Returns a [`ParserErr`] if it's not possible to parse the request.
pub fn parse_req<T>(src: &mut T) -> Result<Request, ParserErr>
where
    T: Read + Write,
{
    let mut reader = BufReader::new(src);

    let start_line = parse_req_line(&mut reader)?;
    let headers = parse_headers(&mut reader)?;
    let body = read_body(&mut reader, headers.as_ref())?;
    
    Ok(Request::new(start_line, headers, body))
}
