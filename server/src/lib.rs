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
//!    Server::new("127.0.0.1:8080").start(router)?;
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

use connection::PersistentConnection;

// re-exports
pub use http_parser::response::SerializeResponse;
pub use routing::Router;
pub use http_core::{
    Method,
    response::{Response, StatusCode, ResponseData} ,
    request::{Request, RequestLine, HttpVersion}
};

use std::{net::{TcpListener, ToSocketAddrs}, sync::Arc};
use std::io::ErrorKind;
use crate::pool::ThreadPool;

mod pool;

/// The server
#[allow(missing_debug_implementations)]
pub struct Server<A: ToSocketAddrs> {
    workers: usize,
    addr: A
}

impl<A: ToSocketAddrs> Server<A> {
    /// Creates a new instance of [`Server`].
    pub fn new(addr: A) -> Self {
        Self {
            workers: 10,
            addr: addr
        }
    }
}
// GENERAL SERVER RELATED IMPLEMENTATIONS
impl<A: ToSocketAddrs> Server<A> {
    /// Starts the server.
    ///
    /// Note you must set endpoints before start the server.
    /// Otherwise, the server won't match against any route.
    /// ```no_run
    /// # use http_core::response::{Response, ResponseData, StatusCode};
    /// # use server::Router;
    /// # use server::Server;
    /// fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///    let mut router = Router::new();
    ///
    ///    router.get("/test", |_| {
    ///        let mut response = Response::new();
    ///        response.set_status(StatusCode::Ok);
    ///        response.set_body(Some(b"Everything is OK"));
    ///        response
    ///    });
    ///
    ///    router.head("/test", |_| {
    ///        let mut response = Response::new();
    ///        response.set_status(StatusCode::Ok);
    ///
    ///        response
    ///    });
    ///    
    ///    Server::new("127.0.0.1:8080").start(router)?;
    ///
    ///    Ok(())
    ///}
    /// ```
    /// If the connection is interrupted, the server will retry to accept incoming 
    /// requests.
    /// 
    /// if the connection is aborted, the server logs an error message on console 
    /// and continues accepting incoming requests.
    /// 
    /// # Errors
    /// 
    /// An error is returned if it's not possible establish a
    /// connection successfully.
    pub fn start(&self, router: Router) -> Result<(), Box<dyn std::error::Error>> {
        let conn = PersistentConnection::new(&self.addr)?;
        let router = Arc::new(router);
        let pool = ThreadPool::new(self.workers, Arc::clone(&router));
        
        loop {
            match conn.listener().accept() {
                Ok((stream, _)) => {
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

    /// Starts the server with a given [`TcpListener`].
    pub fn start_with_listener(
        &self, 
        router: Router, 
        listener: TcpListener
    ) -> Result<(), Box<dyn std::error::Error>> {
        let conn = PersistentConnection::new_with_listener(listener);
        let router = Arc::new(router);
        let pool = ThreadPool::new(self.workers, Arc::clone(&router));

        loop {
            match conn.listener().accept() {
                Ok((stream, _)) => {
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
    /// Returns how much workers will be spawned when the server starts. By default
    /// `10` workers are spawned.
    ///
    /// You overwrite the default value with [`set_workers`].
    pub fn workers(&self) -> usize {
        self.workers
    }

    /// Sets the number of workers, overwriting the defaut value (`10`).
    pub fn set_workers(&mut self, size: usize) {
        self.workers = size;
    }
}
