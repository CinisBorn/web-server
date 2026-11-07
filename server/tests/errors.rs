//! Tests if response errors are correctly received.

use std::{io::{Read, Write}, net::{TcpListener, TcpStream}, sync::OnceLock};

use server::{
    Response, 
    ResponseData, 
    Router, 
    StatusCode,
    Server
};

static SIGNAL: OnceLock<bool> = OnceLock::new();
static ADDR: &'static str = "127.0.0.1:8080";

fn run() {
    SIGNAL.get_or_init(|| {
        let mut router = Router::new();
        let listener = TcpListener::bind(ADDR).unwrap();
        
        router.get("/test", |_| {
            let mut res = Response::new();
            res.set_status(StatusCode::Ok);

            res
        });

        std::thread::spawn(move || {
            Server::new(ADDR)
               .start_with_listener(router, listener)
               .unwrap(); 
        });

        true
    });
}

#[test]
fn conflict_body_framing_fail() {
    run();

    let mut conn = TcpStream::connect(ADDR).unwrap();

    conn.write_all(b"GET /test HTTP/1.1\r\n\
        Connection: close\r\n\
        Transfer-Encoding: chunked\r\n\
        Content-Length: 10\r\n\
        \r\n"
    ).unwrap();

    let mut buf = String::new();
    conn.read_to_string(&mut buf).unwrap();

    assert!(buf.starts_with("HTTP/1.1 400"))   
}

#[test]
fn invalid_input_fail() {
    run();

    let mut conn = TcpStream::connect(ADDR).unwrap();
    
    conn.write_all(
        b"GET /test HTTP/1.1\r\n\
        Connection: close\r\n\
        Host Localhost\r\n\
        \r\n"
    ).unwrap();

    let mut buf = String::new();
    conn.read_to_string(&mut buf).unwrap();
    
    assert!(buf.starts_with("HTTP/1.1 400"))
}

#[test]
fn invalid_coding_fails() {
    run();

    let mut conn = TcpStream::connect(ADDR).unwrap();
    conn.write_all(
        b"GET /test HTTP/1.1\r\n\
        Transfer-Encoding: cyber2077\r\n\
        Connection: close\r\n\
        \r\n"
    ).unwrap();

    let mut buf = String::new();
    conn.read_to_string(&mut buf).unwrap();

    assert!(buf.starts_with("HTTP/1.1 501"));
}

#[test]
fn invalid_version_fail() {
    run();

    let mut conn = TcpStream::connect(ADDR).unwrap();
    conn.write_all(
        b"GET /test HTTP/100.0\r\n\
        Connection: close\r\n\
        \r\n"
    ).unwrap();

    let mut buf = String::new();
    conn.read_to_string(&mut buf).unwrap();

    assert!(buf.starts_with("HTTP/1.1 505"));
}

#[test]
fn request_timeout_works() {
    run();

    let mut conn = TcpStream::connect(ADDR).unwrap();
    conn.write_all(
        b"GET /test HTTP/1.1\r\n"
    ).unwrap();

    let mut buf = String::new();
    conn.read_to_string(&mut buf).unwrap();

    assert!(buf.starts_with("HTTP/1.1 408"));
}