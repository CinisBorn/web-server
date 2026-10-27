//! Tests Server concurrency 
use server::{Router, Server};
use std::{io::{Read, Write}, net::{TcpListener, TcpStream}, sync::OnceLock};
use http_core::response::{Response, StatusCode, ResponseData};

static SIGNAL: OnceLock<bool> = OnceLock::new();

const DEFAULT_ADDR: &'static str = "127.0.0.1:7878";

fn run() {
    SIGNAL.get_or_init(|| { 
        let mut router = Router::new();
        let listener = TcpListener::bind(DEFAULT_ADDR).unwrap();
    
        router.get("/test", |_| {
            let mut response = Response::new();
            response.set_status(StatusCode::Ok);
    
            response
        });
    
        router.head("/test", |_| {
            let mut response = Response::new();
            response.set_status(StatusCode::Ok);
    
            response
        });
        
        std::thread::spawn(move || {
            Server::new().start_with_listener(router, listener).unwrap();
        });

        true
    });
}

#[test]
fn server_is_running() {
    run();
    
    let mut stream = TcpStream::connect(DEFAULT_ADDR).unwrap();
    stream.write_all(b"GET /test HTTP/1.1\r\n\
        Connection: close\r\n\
        \r\n").unwrap();

    let mut content = String::new();
    stream.read_to_string(&mut content).unwrap(); 

    assert!(content.starts_with("HTTP/1.1 200"));
}

#[test]
fn methods_are_working() {
    run();

    // GET method 
    let mut stream = TcpStream::connect(DEFAULT_ADDR).unwrap();
    stream.write_all(b"GET /test HTTP/1.1\r\n\
        Connection: close\r\n\
        \r\n").unwrap();

    let mut content = String::new();
    stream.read_to_string(&mut content).unwrap(); 

    assert!(content.starts_with("HTTP/1.1 200"));

    // HEAD method 
    let mut stream = TcpStream::connect(DEFAULT_ADDR).unwrap();
    stream.write_all(b"HEAD /test HTTP/1.1\r\n\
        Connection: close\r\n\
        \r\n").unwrap();

    let mut content = String::new();
    stream.read_to_string(&mut content).unwrap(); 

    assert!(content.starts_with("HTTP/1.1 200"));
}