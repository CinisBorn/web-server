//! It will exhaustively checks for logical error in parsing functions
use http_parser::HttpParser;

const BASIC_REQUEST: &[u8; 79] = b"GET / HTTP/1.1\r\n\
    User-agent: mock\r\n\
    Host: 127.0.0.1\r\n\
    Accept: application/json\r\n\
    \r\n\
    ";

struct MockParser();

impl MockParser {
    fn new() -> Self { Self() }
}

impl HttpParser for MockParser {}

#[test]
fn request_is_parsed_correctly() {
    let mock = MockParser::new();
    let req = mock.parser(BASIC_REQUEST).unwrap();

    assert_eq!(req.accept_field(), "application/json");
    assert_eq!(req.host(), "127.0.0.1");
}