//! Contains necessary types, utilities and tools for HTTP Response handling.

use crate::Method;
use std::collections::HashMap;

/// The response code status.
#[derive(Debug, Clone)]
pub enum StatusCode {
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
    /// Represents the HTTP status code: `500`.
    InternalServerError,
    /// Represents the HTTP status code: `503.
    ServiceUnavailable,
}

impl StatusCode {
    /// gets the numeric code from phrasal code.
    pub fn code(&self) -> String {
        match self {
            Self::Ok => 200.to_string(),
            Self::BadRequest => 400.to_string(),
            Self::NotFound => 404.to_string(),
            Self::NoContent => 204.to_string(),
            Self::NotModified => 304.to_string(),
            Self::HttpVersionNotSupported => 505.to_string(),
            Self::MethodNotAllowed(_) => 405.to_string(),
            Self::InternalServerError => 500.to_string(),
            Self::ServiceUnavailable => 503.to_string(),
        }
    }
}

#[derive(Debug)]
/// Represents a HTTP Response
pub struct Response {
    status: StatusCode,
    headers: HashMap<String, String>,
    body: Option<Vec<u8>>,
}

impl Response {
    /// Builds a new HTTP [`Response`]
    pub fn new() -> Self {
        Self {
            status: StatusCode::Ok, // temporary default value
            headers: HashMap::new(),
            body: None,
        }
    }

    /// Creates a new [`Response`] with a status code: `500`.
    pub fn internal_error(body: Option<&[u8]>) -> Self {
        Self {
            status: StatusCode::InternalServerError,
            headers: HashMap::new(),
            body: body.map(|c| c.to_vec()),
        }
    }
}

impl ResponseData for Response {
    fn set_headers(&mut self, value: HashMap<String, String>) {
        self.headers = value;
    }

    fn set_body(&mut self, value: Option<&[u8]>) {
        self.body = value.map(|l| l.to_vec());
    }

    fn set_status(&mut self, value: StatusCode) {
        self.status = value;
    }

    fn get_headers_mut(&mut self) -> &mut HashMap<String, String> {
        &mut self.headers
    }

    fn get_body(&self) -> Option<&Vec<u8>> {
        self.body.as_ref()
    }

    fn get_status(&self) -> &StatusCode {
        &self.status
    }

    fn get_headers(&self) -> &HashMap<String, String> {
        &self.headers
    }
}

/// Defines fundamental data handling behavior to [`Response`]
pub trait ResponseData {
    /// Sets the headers of HTTP Response.
    ///
    /// ```no_run
    /// # use std::collections::HashMap;
    /// # use http_core::response::Response;
    /// # use crate::http_core::response::ResponseData;
    /// let mut res = Response::new();
    /// res.set_headers(HashMap::new());
    /// ```
    /// You can define a pre-defined set of headers:
    /// ```no_run
    /// # use std::collections::HashMap;
    /// # use http_core::response::Response;
    /// # use crate::http_core::response::ResponseData;
    /// let mut res = Response::new();
    /// let mut headers = HashMap::new();
    ///
    /// headers.insert(
    ///     "Content-Type".to_string(),
    ///     "application/json".to_string()
    /// );
    ///
    /// res.set_headers(headers);
    /// ```
    fn set_headers(&mut self, value: HashMap<String, String>);
    /// Sets the [`Response`] body.
    /// ```no_run
    /// # use http_core::response::Response;
    /// # use crate::http_core::response::ResponseData;
    /// let mut response = Response::new();
    /// response.set_body(Some(b"Hello World!"))
    /// ```
    fn set_body(&mut self, value: Option<&[u8]>);
    /// Sets the [`Response`] status code.
    /// ```no_run
    /// # use http_core::response::StatusCode;
    /// # use http_core::response::Response;
    /// # use crate::http_core::response::ResponseData;
    /// let mut response = Response::new();
    /// response.set_status(StatusCode::Ok);
    /// ```
    fn set_status(&mut self, value: StatusCode);
    /// Gets a mutable reference to the response headers.
    fn get_headers_mut(&mut self) -> &mut HashMap<String, String>;
    /// Gets a referece to the [`Response`] body.
    fn get_body(&self) -> Option<&Vec<u8>>;
    /// Gets a reference to the [`Response`] status code.
    fn get_status(&self) -> &StatusCode;
    /// Gets a reference to the [`Response`] headers.
    fn get_headers(&self) -> &HashMap<String, String>;
}
