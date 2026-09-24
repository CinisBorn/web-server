use crate::{Method, ParserErr};

pub(crate) fn map_method(method: &str) -> Result<Method, ParserErr> {
    match method {
        "GET" => Ok(Method::GET),
        "POST" => Ok(Method::POST),
        "DELETE" => Ok(Method::DELETE),
        "CONNECT" => Ok(Method::CONNECT),
        "PATCH" => Ok(Method::PATCH),
        "PUT" => Ok(Method::PUT),
        "OPTIONS" => Ok(Method::OPTIONS),
        "HEAD" => Ok(Method::HEAD),
        _ => Err(ParserErr::UnknownMethod(method.to_string())),
    }
}

/// Finds headers lines.
pub(crate) fn find_line(buf: &[u8]) -> Option<&[u8]> {
    buf.windows(2)
        .position(|w| w == b"\r\n")
        .map(|pos| &buf[..pos])
}
