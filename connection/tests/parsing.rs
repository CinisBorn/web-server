//! Tests integration parsing logic of `connection` crate.
use std::net::Shutdown;
use std::thread;

use connection::Connection;
use http_parser::error::ParserError;
use http_parser::Request;

// server mock
fn server() -> Result<Request, ParserError> {
    let conn = Connection::new().unwrap();
    let mut stream = conn.connect().unwrap();
    let req = stream.parse_request();

    req
}

#[test]
fn parsing_cancelled_when_connection_is_closed() -> Result<(), ()> {
    let handle_server = thread::spawn(|| server());

    thread::spawn(|| {
        let conn = std::net::TcpStream::connect("127.0.0.1:7878").unwrap();
        conn.shutdown(Shutdown::Write).unwrap();
    });

    match handle_server.join().unwrap() {
        Ok(_) => Err(()),
        Err(e) => match e {
            ParserError::ConnClosed => Ok(()),
            _ => Err(()),
        },
    }
}
