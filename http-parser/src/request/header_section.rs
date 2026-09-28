//! Contains function for parsing headers and request line sections. 

use std::collections::HashMap;
use std::io::{Write, Read};

use crate::error::ParserErr;
use crate::helpers::find_line;
use crate::request::{RequestLine, HttpVersion};
use crate::request::read_src;
use crate::types::Method;

pub(crate) fn parse_headers<T>(
    src: &mut T,
    content: &mut [u8],
    cursor: &mut usize,
) -> Result<Option<HashMap<String, String>>, ParserErr>
where
    T: Read + Write,
{
    let mut headers = HashMap::new();

    loop {
        if let Some(line) = find_line(&content[*cursor..]) {
            *cursor += line.len() + 2;

            if line.is_empty() {
                if headers.is_empty() { 
                    break Ok(None) 
                } else { 
                    break Ok(Some(headers))
                }
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

pub(crate) fn parse_req_line<T>(
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

    let method = items.next().ok_or(ParserErr::InvalidInput)?;
    let target = items.next().ok_or(ParserErr::InvalidInput)?;
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

    let met = Method::map_method(method);
    let ver = HttpVersion::map(version);

    let req_start_line = RequestLine::new(ver, met, target.to_owned());

    Ok(req_start_line)
}