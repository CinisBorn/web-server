//! TCP connection handling.
//!
//! This crate provides types and utilities for TCP handling.
mod error;

use http_parser::error::ParserError;
use http_parser::{HttpParser, Request};
use std::{
    io::Read,
    net::{TcpListener, TcpStream},
};

use crate::error::ConnectionError;

/// A TCP connection Listener.
pub struct Connection {
    socket: TcpListener,
}

/// A TCP connection stream
pub struct ConnectionStream {
    stream: TcpStream,
    buf: Vec<u8>,
}

impl ConnectionStream {
    /// Parses the raw incoming request into HTTP request.
    ///
    /// # Errors
    /// It will return an error if a parser error is triggered.
    ///
    /// # Note
    /// Because of bad design choices, it will panic instead of return a `Result` containing an error.
    pub fn parse_request(&mut self) -> Result<Request, ParserError> {
        let bytes = self.stream.read(&mut self.buf);

        match bytes {
            Ok(0) => unimplemented!(),
            Err(_) => unimplemented!(),
            Ok(n) => {
                println!("{}", String::from_utf8_lossy(&self.buf[..n])); // for debugging purposes
                Ok(self.parser(&self.buf[..n])?)
            },
        }
    }
}
impl HttpParser for ConnectionStream {}

impl Connection {
    /// Creates a new connection endpoint at the specified address from `WS` environment variable.
    /// If `WS` isn't present, the address: `127.0.0.1:7878` is the default.
    ///
    /// # Errors
    ///
    /// An *io* error is returned if a related error is triggered.
    pub fn new() -> Result<Self, ConnectionError> {
        let env = std::env::var_os("WS");

        if let Some(env) = env {
            let address = env
                .to_str()
                .expect("environment variable should be valid Unicode");
            let socket = TcpListener::bind(address)?;

            Ok(Self { socket })
        } else {
            println!("WS environment variable was not found... Binding to 127.0.0.1:7878");
            let socket = TcpListener::bind("127.0.0.1:7878")?;
            Ok(Self { socket })
        }
    }

    /// Accepts a new incoming connection, returning the correspondent stream.
    ///
    /// The main thread will block until `TcpStream` is dropped.
    ///
    /// # Errors
    /// An error will occur if it's not possible to accept the incoming connection.
    pub fn connect(&self) -> Result<ConnectionStream, std::io::Error> {
        let (stream, _) = self.socket.accept()?;
        Ok(ConnectionStream {
            stream,
            buf: Vec::from([0u8; 8192]), // 8 Kib
        })
    }
}
