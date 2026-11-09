//! Tests if response errors are correctly received.

use std::{io::{Read, Write}, net::{TcpListener, TcpStream}, sync::OnceLock};

use server::{
    RequestConfig, Response, ResponseData, Router, Server, StatusCode
};

static SIGNAL: OnceLock<bool> = OnceLock::new();
static ADDR: &'static str = "127.0.0.1:8080";

fn run() {
    SIGNAL.get_or_init(|| {
        let mut router = Router::new();
        let listener = TcpListener::bind(ADDR).unwrap();
        let mut req_config = RequestConfig::default();

        req_config.set_request_line_max_size(50); // bytes
        req_config.set_header_max_size(128); // 128 bytes
        req_config.set_max_body_size(1024); // 1 kib
        req_config.set_max_num_headers(5);
        
        router.get("/test", |_| {
            let mut res = Response::new();
            res.set_status(StatusCode::Ok);

            res
        });

        std::thread::spawn(move || {
            let mut server = Server::new(ADDR);

            server.set_listener(listener);
            server.start(router, Some(req_config)).unwrap();
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

#[test]
fn exceed_max_num_of_header_fail() {
    run();

    let mut conn = TcpStream::connect(ADDR).unwrap();
    conn.write_all(b"GET /test HTTP/1.1\r\n\
        Connection: close\r\n\
        Custom: header\r\n\
        Custom2: header2\r\n\
        Custom3: header3\r\n\
        Custom4: header4\r\n\
        \r\n").unwrap();

    let mut buf = String::new();
    conn.read_to_string(&mut buf).unwrap();

    assert!(buf.starts_with("HTTP/1.1 431"));
}

#[test]
fn exceed_max_header_size_fail() {
    run();

    let mut conn = TcpStream::connect(ADDR).unwrap();
    conn.write_all(b"GET /test HTTP/1.1\r\n\
        Connection: close\r\n\
        Custom: a big headeeeeeeeeeeeeeeeeeeeeeeeer\r\n\
        Custom2: it should not happennnnnnnnnnnnnnnnn\r\n\
        Custom3: NOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOO\r\n\
        Custom4: PLACEHOLDER FOR A BIGGER INPUUUUUUUUUUUUUUUUUT\r\n\
        \r\n").unwrap();

    let mut buf = String::new();
    conn.read_to_string(&mut buf).unwrap();

    assert!(buf.starts_with("HTTP/1.1 431"));
}

#[test]
fn exceed_max_uri_size_fail() {
    run();

    let mut conn = TcpStream::connect(ADDR).unwrap();
    conn.write_all(b"GET /testing\
        aaaaaaaaaaaaaaaaaaaaaaaaaa\
        aaaaaaaaaaaaggggggggggggggggggggggggg\
        HTTP/1.1\r\n\
        Connection: close\r\n\
        \r\n").unwrap();

    let mut buf = String::new();
    conn.read_to_string(&mut buf).unwrap();

    assert!(buf.starts_with("HTTP/1.1 414"));
}

#[test]
fn exceed_max_req_line_size_fail() {
    run();

    let mut conn = TcpStream::connect(ADDR).unwrap();
    conn.write_all(b"GETTTTTTTTTTTTTTTTTTTTTTTTTTTTT\
        TTTTTTTTTTTTTTTTTTTTTTTT /test\
        HTTP/1.1302489333033209348930200023948\
        2939839332412312312\r\n\
        Connection: close\r\n\
        \r\n").unwrap();

    let mut buf = String::new();
    conn.read_to_string(&mut buf).unwrap();

    assert!(buf.starts_with("HTTP/1.1 414"));
}