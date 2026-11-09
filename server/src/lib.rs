//! This module provides HTTP server functionality, including request routing,
//! connection handling, and server configuration.
//!
//! ```no_run
//! # use http_core::response::{Response, ResponseData, StatusCode};
//! # use server::Router;
//! # use server::Server;
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!    let mut router = Router::new();
//!
//!    router.get("/test", |_| {
//!        let mut response = Response::new();
//!        response.set_status(StatusCode::Ok);
//!        response.set_body(Some(b"Everything is OK"));
//!        response
//!    });
//!
//!    router.head("/test", |_| {
//!        let mut response = Response::new();
//!        response.set_status(StatusCode::Ok);
//!
//!        response
//!    });
//!    
//!    Server::new("127.0.0.1:8080").start(router, None)?;
//!
//!    Ok(())
//!}
//!```
//!
//! # Routing
//! 
//! When a client connects, the server accepts the connection, processes the
//! incoming request, matches it against the registered routes, and generates
//! an HTTP response.
//!
//! # Configuration
//!
//! The server uses a thread pool to process incoming connections. By default,
//! it creates 10 worker threads. Use [Server::set_workers] to configure the
//! number of workers before starting the server.
//! 
//! The server establishes a timeout for read and write operations. By default, both
//! read and write operations can wait for `5` seconds. 
//! 
//! Once the timeout is triggered, the client will receive an HTTP response 
//! containing more information about it. Use [`Server::set_read_timeout`] and 
//! [`Server::set_write_timeout`] to configure the timeout value.

use connection::PersistentConnection;

// re-exports
pub use http_parser::parse_req;
pub use http_parser::response::SerializeResponse;
pub use http_parser::ParserConfig as RequestConfig;

pub use routing::Router;
pub use http_core::{
    Method,
    response::{Response, StatusCode, ResponseData} ,
    request::{Request, RequestLine, HttpVersion}
};

use std::{net::{TcpListener, TcpStream, ToSocketAddrs}, sync::Arc, time::Duration};
use std::io::ErrorKind;
use crate::pool::ThreadPool;

mod pool;

/// The server
#[allow(missing_debug_implementations)]
pub struct Server<A: ToSocketAddrs> {
    workers: usize,
    addr: A,
    read_timeout: Duration,
    write_timeout: Duration,
    listener: Option<TcpListener>,
}

impl<A: ToSocketAddrs> Server<A> {
    /// Creates a new instance of [`Server`].
    pub fn new(addr: A) -> Self {
        Self {
            workers: 10,
            addr: addr,
            read_timeout: Duration::from_secs(5),
            write_timeout: Duration::from_secs(5),
            listener: None,
        }
    }
}
// GENERAL SERVER RELATED IMPLEMENTATIONS
impl<A: ToSocketAddrs> Server<A> {
    /// Starts the HTTP server and begins accepting incoming connections.
    ///
    /// If a listener has already been configured, it will be reused.
    /// Otherwise, a new listener will be bound to the server's configured address.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Binding a new listener to the configured address fails.
    /// - Accepting an incoming connection fails with an error other than
    ///   [`ErrorKind::Interrupted`] or [`ErrorKind::ConnectionAborted`].
    ///
    /// # Behavior
    ///
    /// This method runs indefinitely, continuously accepting incoming
    /// connections. It does not return `Ok(())` during normal operation.
    pub fn start(
        &mut self, 
        router: Router,
        request_config: Option<RequestConfig>
    ) -> Result<(), Box<dyn std::error::Error>> {
        let conn = match self.listener.take() {
            Some(l) => PersistentConnection::new_with_listener(l),
            None =>  PersistentConnection::new(&self.addr)?
        };

        let req_config = Arc::new(request_config);
        let router = Arc::new(router);
        let pool = ThreadPool::new(self.workers, Arc::clone(&router), req_config);
        
        loop {
            match conn.listener().accept() {
                Ok((stream, _)) => {
                    Self::set_stream_timeout(
                        &stream, 
                        self.read_timeout, 
                        self.write_timeout
                    );
                    
                    pool.execute(pool::Message::NewJob(stream));
                }
        
                Err(e) => match e.kind() {
                    ErrorKind::Interrupted => {
                        continue;
                    }
        
                    ErrorKind::ConnectionAborted => {
                        eprintln!("Incoming connection aborted: {e}");
                        continue;
                    }
        
                    _ => {
                        return Err(e.into());
                    }
                },
            }
        }
    }
}
// SERVER'S CONFIGURATIONS RELATED IMPLEMENTATIONS
impl<A: ToSocketAddrs> Server<A> {
    /// Sets a timeout to read and write operations.
    /// 
    /// It handles a possible case where the caller pass `0` as 
    /// value. It's an error. A default value is used instead.  
    fn set_stream_timeout(
        stream: &TcpStream, 
        read_timeout: Duration, 
        write_timeout: Duration
    ) {

        if let Err(_) = stream.set_read_timeout(Some(read_timeout)) {
            let time = Duration::from_secs(5);
            stream.set_read_timeout(Some(time)).unwrap()
        }

        if let Err(_) = stream.set_write_timeout(Some(write_timeout)) {
            let time = Duration::from_secs(5);
            stream.set_write_timeout(Some(time)).unwrap()
        }
    }
    
    /// Returns the configured number of worker threads.
    /// 
    /// By default, the server is configured to spawn `10` workers.
    /// The value can be changed using [`Server::set_workers`].
    pub fn workers(&self) -> usize {
        self.workers
    }

    /// Sets the number of worker threads the server will spawn.
    ///
    /// No explicit upper limit is enforced by the server. However,
    /// spawning too many workers may exhaust system resources.
    pub fn set_workers(&mut self, size: usize) {
        self.workers = size;
    }

    /// Sets the timeout for read operations on client connections.
    ///
    /// If a read operation exceeds this duration, the operation
    /// returns a timeout error. The connection may then be closed.
    /// 
    /// If the provided value is zero, the default value of `5` seconds is 
    /// used.
    pub fn set_read_timeout(&mut self, value: Duration) {
        self.read_timeout = value
    }

    /// Sets the timeout for write operations on client connections.
    ///
    /// If a write operation exceeds this duration, the operation
    /// returns a timeout error. The connection may then be closed.
    /// 
    /// If the provided value is zero, the default value of `5` seconds is 
    /// used.
    pub fn set_write_timeout(&mut self, value: Duration) {
        self.write_timeout = value
    }

    /// Sets the TCP listener used by the server.
    ///
    /// The server takes ownership of the provided listener and uses it
    /// to accept incoming connections. If no listener is provided,
    /// the server creates and binds a new listener when started.
    pub fn set_listener(&mut self, listener: TcpListener) {
        self.listener = Some(listener)
    }
}
