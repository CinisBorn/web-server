//! Tools for body parsing.

use std::collections::HashMap;
use std::io::{Read, Write};

use crate::error::ParserErr;


pub(crate) fn read_body<T>(
    src: &mut T,
    content: &mut [u8],
    cursor: &mut usize,
    headers: Option<&HashMap<String, String>>,
) -> Result<Option<String>, ParserErr>
where
    T: Read + Write,
{
    let Some(headers) = headers else {
        return Ok(None)
    };
    let Some(content_length) = headers.get("Content-length") else {
        return Ok(None);
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
        src.read_exact(&mut body[from_buffer..])
            .map_err(ParserErr::Io)?;
    }

    let Ok(body_txt) = String::from_utf8(body) else {
        return Err(ParserErr::InvalidInput);
    };

    Ok(Some(body_txt))
}
