//! The HTTP Response

use std::{collections::HashMap};

use crate::types::Method;

/// The response code status.
#[derive(Debug)]
pub enum CodeStatus {
    /// Represents the HTTP status code: `200`.
    Ok,
    /// Represents the HTTP status code: `404`.
    NotFound,
    /// Represents the HTTP status code: `405`.
    MethodNotAllowed(Vec<Method>),
    /// Represents the HTTP status code: `400`.
    BadRequest,
    /// Represents the HTTP status code: `204`.
    NoContent,
    /// Represents the HTTP status code: `304`.
    NotModified,
    /// Represents the HTTP status code: `505`.
    HttpVersionNotSupported,
}

impl CodeStatus {
    /// gets the numeric code from phrasal code.
    fn code(&self) -> String {
        match self {
            Self::Ok => 200.to_string(),
            Self::BadRequest => 400.to_string(),
            Self::NotFound => 404.to_string(),
            Self::NoContent => 204.to_string(),
            Self::NotModified => 304.to_string(),
            Self::HttpVersionNotSupported => 505.to_string(),
            Self::MethodNotAllowed(_) => 405.to_string()
        }
    }
}

#[derive(Debug)]
/// An HTTP request response
pub struct Response {
    status: CodeStatus,
    headers: HashMap<String, String>,
    body: Option<Vec<u8>>,
}

impl Response {
    /// Builds an HTTP response.
    pub fn build(status: CodeStatus, body: Option<&[u8]>) -> Self {
        Self {
            status,
            headers: HashMap::new(),
            body: body.map(|s| s.to_vec()),
        }
    }

    /// Serializes the HTTP response.
    pub fn serialize(&mut self, method: &Method) -> Vec<u8> {
        let mut response: Vec<u8> = Vec::new();

        let headers = &mut self.headers;
        let body = &self.body;
        let status = self.status.code();
        let serialize_body = check_response(&status, method);

        insert_content_length(headers, serialize_body, body.as_ref());

        match &self.status {
            CodeStatus::MethodNotAllowed(methods) => {
                 insert_allow_header(headers, methods.to_owned());
            },
            _ => {}
        }
       
        sereliaze_status_line("HTTP/1.1", &status, &mut response);
        serialize_headers(&mut self.headers, &mut response);
        
        insert_body(body.as_ref(), &mut response, serialize_body);

        response
    }

    #[cfg(debug_assertions)]
    #[allow(clippy::missing_panics_doc)]
    /// Serializes a HTTP response and convert into `String`.
    pub fn serialize_to_string(&mut self, method: &Method) -> String {
        String::from_utf8(self.serialize(method)).unwrap()
    }

    /// Returns the http status code from response
    pub fn status(&self) -> u16 {
        self.status
            .code()
            .parse::<u16>()
            .expect("parse status code")
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

fn insert_body(
    body: Option<&Vec<u8>>, 
    response: &mut Vec<u8>, 
    serialize_body: bool 
) {
    // mandatory body break line
    response.extend_from_slice(b"\r\n"); 
    
    if let Some(body) = body {    
        if serialize_body {
            response.extend_from_slice(body.as_slice());
        }
    }
}

/// Checks if the response will have a body.
fn check_response(status: &str, method: &Method) -> bool {
    let method = !matches!(method, Method::HEAD);
    let status = !matches!(status, "304" | "204");

    method && status
}

/// Inserts content length header conditionally
fn insert_content_length(
    headers: &mut HashMap<String, String>, 
    serialize_body: bool,
    body: Option<&Vec<u8>>,
) {
    if let Some(body) = body {
        if serialize_body {
            headers.insert(
                "Content-Length".to_string(), 
                body.len().to_string()
            );
        }
    } 
}

fn insert_allow_header(
    headers: &mut HashMap<String, String>, 
    methods: Vec<Method>
) {
    let mut value = String::new();
    
    for method in methods {
        value.push_str(method.to_str());
    }

    headers.insert("Allow".to_string(), value);
}
