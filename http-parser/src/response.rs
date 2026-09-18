//! The HTTP Response

use std::collections::HashMap;

/// The response code status.
#[derive(Debug)]
pub enum CodeStatus {
    /// Represents the HTTP status code: `200`.
    Ok,
    /// Represents the HTTP status code: `404`.
    NotFound,
    /// Represents the HTTP status code: `400`.
    BadRequest,
}

impl CodeStatus {
    /// gets the numeric code from phrasal code.
    fn code(&self) -> String {
        match self {
            CodeStatus::Ok => 200.to_string(),
            CodeStatus::BadRequest => 400.to_string(),
            CodeStatus::NotFound => 404.to_string(),
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
    pub fn serialize(&mut self) -> Vec<u8> {
        let mut response: Vec<u8> = Vec::new();
        let headers = &mut self.headers;
        
        let body = &self.body;

        let status = self.status.code();

        if !body.is_empty() {
            headers.insert("Content-Length".to_string(), body.len().to_string());
        }
        
        sereliaze_status_line("HTTP/1.1", &status, &mut response);
        serialize_headers(&mut self.headers, &mut response);
    
        response.extend_from_slice(b"\r\n");
        response.extend_from_slice(body);

        response
    }
}

fn sereliaze_status_line(
    version: &str, 
    status: &str, 
    response: &mut Vec<u8>
) {
    response.extend_from_slice(version.as_bytes());
    response.extend_from_slice(b" ");
    response.extend_from_slice(status.as_bytes());
    response.extend_from_slice(b"\r\n");
}

fn serialize_headers(
    headers: &mut HashMap<String, String>, 
    response: &mut Vec<u8>
) {
    for (key, value) in headers {
        response.extend_from_slice(key.as_bytes());
        response.extend_from_slice(b":");
        response.extend_from_slice(value.as_bytes());
        response.extend_from_slice(b"\r\n");
    }
}