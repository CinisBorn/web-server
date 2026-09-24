//! The HTTP Response

use std::collections::HashMap;

use crate::request::Method;

/// The response code status.
#[derive(Debug)]
pub enum CodeStatus {
    /// Represents the HTTP status code: `200`.
    Ok,
    /// Represents the HTTP status code: `404`.
    NotFound,
    /// Represents the HTTP status code: `400`.
    BadRequest,
    /// Represents the HTTP status code: `204`.
    NoContent,
    /// Represents the HTTP status code: `304`.
    NotModified,
}

impl CodeStatus {
    /// gets the numeric code from phrasal code.
    fn code(&self) -> String {
        match self {
            CodeStatus::Ok => 200.to_string(),
            CodeStatus::BadRequest => 400.to_string(),
            CodeStatus::NotFound => 404.to_string(),
            CodeStatus::NoContent => 204.to_string(),
            CodeStatus::NotModified => 304.to_string(),
        }
    }
}

#[derive(Debug)]
/// An HTTP request response
pub struct Response {
    status: CodeStatus,
    headers: HashMap<String, String>,
    body: Vec<u8>,
}

impl Response {
    /// Builds an HTTP response.
    pub fn build(status: CodeStatus, body: &[u8]) -> Self {
        Self {
            status,
            headers: HashMap::new(),
            body: body.to_vec(),
        }
    }

    /// Serializes the HTTP response.
    pub fn serialize(&mut self, method: &Method) -> Vec<u8> {
        let mut response: Vec<u8> = Vec::new();

        let headers = &mut self.headers;
        let body = &self.body;
        let status = self.status.code();
        let has_body = check_response(&status, method);

        if has_body {
            headers.insert("Content-Length".to_string(), body.len().to_string());
        }

        sereliaze_status_line("HTTP/1.1", &status, &mut response);
        serialize_headers(&mut self.headers, &mut response);

        response.extend_from_slice(b"\r\n"); // mandatory body break line

        if has_body {
            serialize_body(body, &mut response);
        }

        response
    }

    #[cfg(debug_assertions)]
    #[allow(clippy::missing_panics_doc)]
    /// Serializes a HTTP response and convert into `String`.
    pub fn serialize_to_string(&mut self, method: &Method) -> String {
        String::from_utf8(self.serialize(method)).unwrap()
    }
}

fn sereliaze_status_line(version: &str, status: &str, response: &mut Vec<u8>) {
    response.extend_from_slice(version.as_bytes());
    response.extend_from_slice(b" ");
    response.extend_from_slice(status.as_bytes());
    response.extend_from_slice(b"\r\n");
}

fn serialize_headers(headers: &mut HashMap<String, String>, response: &mut Vec<u8>) {
    for (key, value) in headers {
        response.extend_from_slice(key.as_bytes());
        response.extend_from_slice(b": ");
        response.extend_from_slice(value.as_bytes());
        response.extend_from_slice(b"\r\n");
    }
}

fn serialize_body(body: &[u8], response: &mut Vec<u8>) {
    response.extend_from_slice(body);
}

/// Checks if the response will have a body. 
fn check_response(status: &str, method: &Method) -> bool {
    let method = !matches!(method, Method::HEAD);
    let status = !matches!(status, "304" | "204");

    method && status
}
