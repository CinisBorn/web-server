//! The HTTP Response

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
    fn code(&self) -> u16 {
        match self {
            CodeStatus::Ok => 200,
            CodeStatus::BadRequest => 400,
            CodeStatus::NotFound => 404,
        }
    }
}

#[derive(Debug)]
/// An HTTP request response 
pub struct Response {
    status: CodeStatus,
    body: Vec<u8>,    
}

impl Response {
    /// Builds an HTTP response. 
    pub fn build(status: CodeStatus, body: &[u8]) -> Self {
        Self {
            status,    
            body: body.to_vec(),
        }
    }

    /// Serializes the HTTP response. 
    pub fn serialize(&self) -> Vec<u8> {
        let mut response: Vec<u8> = Vec::new();
        let body = &self.body;
        
        let status = self.status.code();
        let status_line = format!("HTTP/1.1 {} \r\n", status);
        let length_header = format!("Content-length: {}\r\n", body.len());
        
        response.extend_from_slice(status_line.as_bytes());
        response.extend_from_slice(length_header.as_bytes());
        response.extend_from_slice(b"\r\n");
        response.extend_from_slice(body);

        response
    }
}