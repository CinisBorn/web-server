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
//!    Server::new().start(router)?;
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

use connection::Connection;
pub use routing::Router;

use std::{net::TcpListener, sync::Arc};

use crate::pool::ThreadPool;

mod pool;

/// The server
#[allow(missing_debug_implementations)]
pub struct Server {
    workers: usize,
}

impl Server {
    /// Creates a new instance of [`Server`].
    pub fn new() -> Self {
        Self {
            workers: 10,
        }
    }
}
// GENERAL SERVER RELATED IMPLEMENTATIONS
impl Server {
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
    ///    Server::new().start(router)?;
    ///
    ///    Ok(())
    ///}
    /// ```
    ///
    /// # Errors
    ///
    /// An error is returned if it's not possible establish a
    /// connection successfully.
    pub fn start(&self, router: Router) -> Result<(), Box<dyn std::error::Error>> {
        let conn = Connection::new()?;
        let router = Arc::new(router);
        let pool = ThreadPool::new(self.workers, Arc::clone(&router));

        loop {
            let stream = conn.connect()?;
            pool.execute(pool::Message::NewJob(stream));
        }
    }

    /// Starts the server with a given [`TcpListener`].
    pub fn start_with_listener(
        &self, 
        router: Router, 
        listener: TcpListener
    ) -> Result<(), Box<dyn std::error::Error>> {
        let conn = Connection::with_listener(listener)?;
        let router = Arc::new(router);
        let pool = ThreadPool::new(self.workers, Arc::clone(&router));

        loop {
            let stream = conn.connect()?;
            pool.execute(pool::Message::NewJob(stream));
        }
    }
}
// SERVER'S CONFIGURATIONS RELATED IMPLEMENTATIONS
impl Server {
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
