//! Provides tools for client request handling 

/// The HTTP method of `Request`.
#[derive(Debug)]
#[allow(clippy::upper_case_acronyms)]
pub enum Method {
    /// The GET method.
    GET,
    /// The POST method.
    POST,
    /// The DELETE method.
    DELETE,
    /// The PATCH method.
    PATCH,
    /// The CONNECT method.
    CONNECT,
    /// The OPTIONS method.
    OPTIONS,
    /// The PUT method.
    PUT,
    /// The HEAD method.
    HEAD,
}

/// Represents the client request.
#[derive(Debug)]
pub struct Request {
    method: Method,
    host: String,
    user_agent: String,
    accept: String,
}

impl Request {
    pub fn new(
        method: Method,
        host: String,
        user_agent: String,
        accept: String
    ) -> Self {
        Self {
            method, 
            host, 
            user_agent,
            accept
        }
    }
    
    /// Retrieves the HTTP method.
    pub fn method(&self) -> &Method {
        &self.method
    }

    /// Retrieves the host of request.
    pub fn host(&self) -> &String {
        &self.host
    }

    /// Retrieves the agent of request.
    pub fn user_agent(&self) -> &String {
        &self.user_agent
    }

    /// Retrieves the `accept` field from request.
    pub fn accept_field(&self) -> &String {
        &self.accept
    }
}