//! This module provides HTTP server functionality, including request routing,
//! connection handling, and server configuration.
//!
//! ```no_run
//! use http_core::response::{
//!     Response,
//!     ResponseData,
//!     StatusCode
//! };
//!
//! use server::Server;
//!
//! let mut server = Server::new();
//!
//! server.get("/test", |_| {
//!     let mut response = Response::new();
//!     response.set_status(StatusCode::Ok);
//!     response.set_body(Some(b"Everything is OK"));
//!     response
//! });
//!
//! server.head("/test", |_| {
//!     let mut response = Response::new();
//!     response.set_status(StatusCode::Ok);
//!
//!     response
//! });
//!
//! server.start();
//! ```
//!
//! # Routing
//!
//! Use [`Server::get`] and [`Server::head`] to register handlers for GET and
//! HEAD requests, respectively. Register all required routes before starting
//! the server.
//!
//! When a client connects, the server accepts the connection, processes the
//! incoming request, matches it against the registered routes, and generates
//! an HTTP response.
//!
//! # Configuration
//!
//! The server uses a thread pool to process incoming connections. By default,
//! it creates 10 worker threads. Use [`Server::set_workers`] to configure the
//! number of workers before starting the server.

use connection::Connection;
use http_core::{request::Request, response::Response};
use routing::Router;

use std::sync::{Arc, Mutex};

use crate::pool::ThreadPool;

mod pool;

/// The server
#[allow(missing_debug_implementations)]
pub struct Server {
    router: Arc<Mutex<Router>>,
    workers: usize,
}

impl Default for Server {
    fn default() -> Self {
        Self::new()
    }
}

// ROUTING RELATED IMPLEMENTATIONS
impl Server {
    /// Creates a new instance of [`Server`].
    pub fn new() -> Self {
        Self {
            router: Arc::new(Mutex::new(Router::new())),
            workers: 10,
        }
    }

    /// Registers a new `GET` endpoint to the routing.
    #[allow(clippy::missing_panics_doc)]
    pub fn get<T>(&mut self, uri: &str, handle: T)
    where
        T: Fn(&Request) -> Response + 'static + Sync + Send,
    {
        let mut router = self.router.lock().unwrap();
        router.register_route(uri, "GET", handle);
    }

    /// Register a new `HEAD` endpoint to the router.
    #[allow(clippy::missing_panics_doc)]
    pub fn head<T>(&mut self, target: &str, handle: T)
    where
        T: Fn(&Request) -> Response + 'static + Sync + Send,
    {
        let mut router = self.router.lock().unwrap();
        router.register_route(target, "HEAD", handle);
    }

    /// Matchs the request against registered routes.
    ///
    /// If the caller registers equal routes, the first is found is matched.
    #[allow(clippy::missing_panics_doc)]
    pub fn match_routes(&self, request: &Request) -> Response {
        let router = self.router.lock().unwrap();
        router.match_route(request)
    }

    /// Returns a reference to server router.
    pub fn router(&self) -> Arc<Mutex<Router>> {
        Arc::clone(&self.router)
    }
}
// GENERAL SERVER RELATED IMPLEMENTATIONS
impl Server {
    /// Starts the server.
    ///
    /// Note you must set endpoints before start the server.
    /// Otherwise, the server won't match against any route.
    ///
    /// # Errors
    ///
    /// An error is returned if it's not possible establish a
    /// connection successfully.
    pub fn start(&self) -> Result<(), Box<dyn std::error::Error>> {
        let conn = Connection::new()?;
        let pool = ThreadPool::new(self.workers, self.router());

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
