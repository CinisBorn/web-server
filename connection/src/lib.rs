//! TCP connection handling.
//!
//! This crate provides types and utilities for TCP handling.
pub mod error;

use std::io::Write;
use std::net::{TcpListener, TcpStream};

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
}

impl Stream for ConnectionStream {
    fn send_response(&mut self, content: &[u8]) {
        let _ = self.stream.write(content);
    }

    fn stream_mut(&mut self) -> &mut TcpStream {
        &mut self.stream
    }

    fn stream(&self) -> &TcpStream {
        &self.stream
    }
}

/// Provides behavior for connection stream handling. 
pub trait Stream {
    /// Gets a mutable reference to the inner [`TcpStream`].
    fn stream_mut(&mut self) -> &mut TcpStream;
    /// Gets a reference to the inner [`TcpStream`]. 
    fn stream(&self) -> &TcpStream;
    /// Sends a reponse to client
    fn send_response(&mut self, content: &[u8]);
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

    /// Creates a connection with a given listener
    pub fn with_listener(listener: TcpListener) -> Result<Self, ConnectionError> {
        Ok(Self { socket: listener })
    }

    /// Accepts a new incoming connection, returning the correspondent stream.
    ///
    /// The main thread will block until `TcpStream` is dropped.
    ///
    /// # Errors
    /// An error will occur if it's not possible to accept the incoming connection.
    pub fn connect(&self) -> Result<ConnectionStream, ConnectionError> {
        let (stream, _) = self.socket.accept()?;
        Ok(ConnectionStream { stream })
    }
}
