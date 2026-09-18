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
        let length_header = format!("Content-length: {}\r\n", body.len());

        sereliaze_status_line("HTTP/1.1", &status, &mut response);
        response.extend_from_slice(length_header.as_bytes());
        response.extend_from_slice(b"\r\n");
        response.extend_from_slice(body);

        response
    }
}

fn sereliaze_status_line(version: &str, status: &str, response: &mut Vec<u8>) {
    response.extend_from_slice(version.as_bytes());
    response.extend_from_slice(b" ");
    response.extend_from_slice(status.as_bytes());
    response.extend_from_slice(b"\r\n");
}