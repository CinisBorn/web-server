use crate::{Method, ParserErr};

pub(crate) fn map_method(method: &str) -> Result<Method, ParserErr> {
    match method {
        "GET" => Ok(Method::GET),
        "HEAD" => Ok(Method::HEAD),
        _ => Ok(Method::Unsupported),
    }
}

/// Finds headers lines.
pub(crate) fn find_line(buf: &[u8]) -> Option<&[u8]> {
    buf.windows(2)
        .position(|w| w == b"\r\n")
        .map(|pos| &buf[..pos])
}
