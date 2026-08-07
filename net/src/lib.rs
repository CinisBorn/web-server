use std::{net::{TcpListener, TcpStream}};
use http_parser::HttpParser;

/// The connection between client and server 
pub struct Connection {
    socket: TcpListener,
}

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
    /// Creates a new connection endpoint at `127.0.0.1:7878` by default.
    /// 
    /// # Errors 
    /// An error will occur if it's not possible create a new socket. It's common the address/port is 
    /// not available, returning an error. 
    pub fn new() -> Result<Self, std::io::Error> {
        let socket = TcpListener::bind("127.0.0.1:7878")?;
        Ok(Self {socket})
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
