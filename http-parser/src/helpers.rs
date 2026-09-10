use crate::{Method, ParserError};

pub(crate) fn map_method(method: &str) -> Result<Method, ParserError> {
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