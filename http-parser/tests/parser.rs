//! It will exhaustively checks for logical error in parsing functions
use http_parser::{parse_req, request::Method};
use http_parser::error::ParserErr;

use std::io::{Cursor, Write};

const BUF_SIZE: usize = 8192;

const BASIC_REQUEST: &[u8; 103] = b"GET / HTTP/1.1\r\n\
    User-agent: mock\r\n\
    Host: 127.0.0.1\r\n\
    Accept: application/json\r\n\
    Content-length: 5\r\n\
    \r\n\
    hello";

const REQUEST_WITH_INCOMPLETE_BODY: &[u8; 100] = b"GET / HTTP/1.1\r\n\
    User-agent: mock\r\n\
    Host: 127.0.0.1\r\n\
    Accept: application/json\r\n\
    Content-length: 5\r\n\
    \r\n\
    he";

fn mock_source(content: &[u8], capacity: usize) -> Cursor<Vec<u8>> {
    let mut source = Cursor::new(Vec::with_capacity(capacity));

    source.write_all(content).unwrap();
    source.set_position(0);

    source
}

#[test]
fn request_is_parsed_correctly() {
    let mut source = mock_source(BASIC_REQUEST, BUF_SIZE);
    let mut buf = [0u8; 103];

    let req = parse_req(&mut source, &mut buf).unwrap();
    let headers = req.headers().unwrap();
    let accept = headers.get("Accept");
    let host = headers.get("Host");

    assert_eq!(accept, Some(&String::from("application/json")));
    assert_eq!(host, Some(&String::from("127.0.0.1")));
}

#[test]
fn start_line_is_parsed_correctly() {
    let mut source = mock_source(BASIC_REQUEST, BUF_SIZE);
    let mut buf = [0u8; 103];

    let req = parse_req(&mut source, &mut buf).unwrap();
    let start_line = req.start_line();

    assert_eq!(start_line.target(), "/");
    assert_eq!(*start_line.method(), Method::GET);
}

#[test]
fn request_with_body_is_parsed_correctly() {
    let mut source = mock_source(BASIC_REQUEST, BUF_SIZE);
    let mut buf = [0u8; 103];

    let req = parse_req(&mut source, &mut buf).unwrap();

    assert_eq!(req.body().unwrap(), "hello");
}

#[test]
#[should_panic]
fn retry_if_body_is_incomplete() {
    let mut source = mock_source(REQUEST_WITH_INCOMPLETE_BODY, BUF_SIZE);
    let mut buf = [0u8; 100];

    parse_req(&mut source, &mut buf).unwrap();
}

#[test]
fn fail_if_http_version_is_unsupported() -> Result<(), ()> {
    let mut buf = [0u8; 18];
    let mut src = mock_source(b"GET / HTTP/2.0\r\n\r\n", BUF_SIZE);
    let req = parse_req(&mut src, &mut buf).err().unwrap();

    match req {
         ParserErr::UnsupportedHttpVersion(_) => return Ok(()),
         _ => return Err(())
    }
}
