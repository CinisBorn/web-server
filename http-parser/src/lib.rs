use std::io::{BufRead, BufReader};
use error::types:: ParserError;

#[derive(Debug)]
pub enum Method {
    GET, 
    POST,
    DELETE,
    PATCH,
    CONNECT, 
    OPTIONS,
    PUT,
    HEAD 
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

pub trait HttpParser {
    /// Parses a HTTP request returning a `Request` struct.  
    fn parser(&self, raw_txt: &[u8]) -> Result<Request, ParserError> {
        let buf = BufReader::new(raw_txt);
        let mut lines = buf.lines().map(|line| line.unwrap()).collect::<Vec<String>>();
        
        let method = lines.swap_remove(0);
        let method = method
            .split_ascii_whitespace()
            .map(|word| word.to_string())
            .collect::<Vec<String>>()
            .remove(0);
        let method =  map_method(&method)?;

        let mut accept = String::new();
        let mut host = String::new();
        let mut user_agent = String::new();
        
        for line in lines {
            if line == "" {
                continue 
            }
            
            let field = line.split(":").map(|f| f.to_string()).collect::<Vec<String>>();
            let value = field[1].trim().to_string();
            
            match field[0].to_lowercase().as_str() {
                "user-agent" => user_agent = value,
                "accept" => accept = value,
                "host" => host = value,
                _ => {
                    // It's a temporary measure
                    panic!("Unknown http field");
                }
            };
        };

        Ok(Request {
            method,
            accept,
            host,
            user_agent
        })
    }
}

// Maps the method from http request to a respective `Enum` variant. 
fn map_method(method: &String) -> Result<Method, ParserError> {
    let method = method.as_str();
    
    match method {
        "GET" => Ok(Method::GET),
        "POST" => Ok(Method::POST),
        "DELETE" => Ok(Method::DELETE),
        "CONNECT" => Ok(Method::CONNECT),
        "PATCH" => Ok(Method::PATCH),
        "PUT" => Ok(Method::PUT),
        "OPTIONS" => Ok(Method::OPTIONS),
        "HEAD" => Ok(Method::HEAD),
        _ => Err(ParserError::UnknownMethod(method.to_string()))
    }
}