//! TCP connection handling.
//! 
//! This crate provides types and utilities for TCP handling. 

use std::{net::{IpAddr, Ipv4Addr, TcpListener, TcpStream}, os::unix::net::SocketAddr};
use http_parser::HttpParser;

/// A TCP connection Listener.  
pub struct Connection {
    socket: TcpListener,
}

/// A TCP connection stream
pub struct ConnectionStream {
    stream: TcpStream,
}

impl HttpParser for ConnectionStream {}

impl ConnectionStream {
    pub fn get_stream(&self) -> &TcpStream {
        &self.stream
    }
}

impl Connection {
    /// Creates a new connection endpoint at the specified address from `WS` environment variable. If `WS` isn't present,
    /// the address: `127.0.0.1:7878` is the default.
    /// 
    /// # Errors 
    /// 
    /// An *io* error is returned if a related error is triggered. 
    pub fn new() -> Result<Self, std::io::Error> {
        let env = std::env::var_os("WS"); 

        if let Some(env) = env {        
            let address = env.to_str().expect("environment variable should be valid Unicode");
            let socket = TcpListener::bind(address)?;

            Ok(Self {socket})
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
        Ok(ConnectionStream { stream })
    }
}
