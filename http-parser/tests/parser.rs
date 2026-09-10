//! It will exhaustively checks for logical error in parsing functions
use http_parser::{parse_req, request::Method};
use std::io::{Cursor, Write};

const BASIC_REQUEST: &[u8; 79] = b"GET / HTTP/1.1\r\n\
    User-agent: mock\r\n\
    Host: 127.0.0.1\r\n\
    Accept: application/json\r\n\
    \r\n\
    ";
const BASIC_REQUEST_WITH_BODY: &[u8; 103] = b"GET / HTTP/1.1\r\n\
    User_agent: mock\r\n\
    Host: 127.0.0.1\r\n\
    Accept: application/json\r\n\
    Content-length: 5\r\n\
    \r\n\
    hello";

fn mock_source(content: &[u8], capacity: usize) -> Cursor<Vec<u8>> {
    let mut source = Cursor::new(Vec::with_capacity(capacity));

    source.write_all(content).unwrap();
    source.set_position(0);

    source
}

#[test]
fn request_is_parsed_correctly() {
    let mut source = mock_source(BASIC_REQUEST, 8192);
    let mut buf = [0u8; 79];

    let req = parse_req(&mut source, &mut buf).unwrap();
    let headers = req.headers();
    let accept = headers.get("Accept");
    let host = headers.get("Host");

    assert_eq!(accept, Some(&String::from("application/json")));
    assert_eq!(host, Some(&String::from("127.0.0.1")));
}

#[test]
fn start_line_is_parsed_correctly() {
    let mut source = mock_source(BASIC_REQUEST, 8192);
    let mut buf = [0u8; 79];

    let req = parse_req(&mut source, &mut buf).unwrap();
    let start_line = req.start_line();

    assert_eq!(start_line.target(), "/");
    assert_eq!(*start_line.method(), Method::GET);
}

#[test]
fn request_with_body_is_parsed_correctly() {
    let mut source = mock_source(BASIC_REQUEST_WITH_BODY, 8192);
    let mut buf = [0u8; 103];

    let req = parse_req(&mut source, &mut buf).unwrap();
    
    assert_eq!(req.body(), "hello");
}
