//! Tests related to response parsing.

use http_parser::request::{Method, Request};
use http_parser::{
    parse_req,
    response::{CodeStatus, Response},
};

use std::io::{Cursor, Write};

const CAPACITY: usize = 8192;

const BASIC_REQUEST: &[u8; 103] = b"GET / HTTP/1.1\r\n\
    User-agent: mock\r\n\
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

fn mock_basic_request() -> Request {
    let mut src = mock_source(BASIC_REQUEST, CAPACITY);
    let mut buf = [0u8; 103];
    let req = parse_req(&mut src, &mut buf).unwrap();

    req
}

#[test]
fn parse_response() {
    let req = mock_basic_request();

    let mut response = Response::build(CodeStatus::Ok, req.body().as_bytes());
    let response = response.serialize_to_string(&Method::GET);
    let expected: &[u8] = b"HTTP/1.1 200\r\n\
        Content-Length: 5\r\n\
        \r\n\
        hello";
    let expected = expected.to_vec();

    assert_eq!(response, String::from_utf8(expected).unwrap());
}

#[test]
fn do_not_include_body() {
    let req = mock_basic_request();

    let mut response = Response::build(CodeStatus::Ok, req.body().as_bytes());
    let response = response.serialize_to_string(&Method::HEAD);
    let expected = String::from_utf8(b"HTTP/1.1 200\r\n\r\n".to_vec()).unwrap();

    assert_eq!(response, expected);
}
