//! TCP connection handling.
//!
//! This crate provides types and utilities for TCP handling.
pub mod error;

use std::io::ErrorKind::Interrupted;
use std::io::Write;
use std::{
    io::Read,
    net::{TcpListener, TcpStream},
};

use crate::error::ConnectionError;

/// A TCP connection Listener.
#[derive(Debug)]
pub struct Connection {
    socket: TcpListener,
}

/// A TCP connection stream
#[derive(Debug)]
pub struct ConnectionStream {
    stream: TcpStream,
    buf: Vec<u8>,
}

impl ConnectionStream {
    /// Handle a incoming connection.
    ///
    /// Attempts to read data from the stream into the internal buffer.
    ///
    /// Returns `Ok(())` if one or more bytes are successfully read.
    ///
    /// # Errors
    ///
    /// Returns [`ConnectionError::Closed`] if the client closes the connection.
    /// This is not considered a fatal error and should be handled by the caller.
    ///
    /// If an [`ErrorKind::Interrupted`] I/O error occurs, the operation is retried.
    ///
    /// Returns [`ConnectionError::Io`] for any other I/O error.
    pub fn handle_connection(&mut self) -> Result<(), ConnectionError> {
        loop {
            match self.stream.read(&mut self.buf) {
                Ok(0) => return Err(ConnectionError::Closed),
                Ok(_) => return Ok(()),
                Err(e) if e.kind() == Interrupted => {},
                Err(e) => return Err(ConnectionError::Io(e)),
            }
        }
    }

    /// Sends a reponse to client
    pub fn send_response(&mut self) {
        let _ = self.stream.write(
            b"HTTP/1.1 200 OK\r\n\
        Content-Type: text/plain\r\n\
        Content-Length: 5\r\n\
        \r\n\
        Hello
        ",
        );
    }
}

impl Connection {
    /// Creates a new connection endpoint at the specified address from `WS` environment variable.
    /// If `WS` isn't present, the address: `127.0.0.1:7878` is the default.
    ///
    /// # Errors
    ///
    /// An *io* error is returned if a related error is triggered.
    /// 
    /// # Panics 
    /// 
    /// Panics if the `WS` environment variable isn't a valid Unicode
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
    pub fn connect(&self) -> Result<ConnectionStream, ConnectionError> {
        let (stream, _) = self.socket.accept()?;
        Ok(ConnectionStream {
            stream,
            buf: Vec::from([0u8; 8192]), // 8 Kib
        })
    }
}
