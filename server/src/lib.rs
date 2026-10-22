//! The binary for running the server.
//!
//! The server provides tools for server handling, including routing and server configurations.
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
//! The `server.get` and `server.head` sets a new endpoint to the server router. Once you settled
//! every necessary route, you starts the server.
//!
//! The server will listen to incoming request, parse it, then generate a new HTTP response for
//! the client.

use connection::Connection;
use http_core::{request::Request, response::Response};
use routing::Router;

use std::{sync::{Arc, Mutex}};

use crate::pool::ThreadPool;

mod pool;

/// The server
#[allow(missing_debug_implementations)]
pub struct Server {
    router: Arc<Mutex<Router>>,
    workers: usize
}

// ROUTING RELATED IMPLEMENTATIONS
impl Server {
    /// Creates a new instance of [`Server`].
    pub fn new() -> Self {
        Self {
            router: Arc::new(Mutex::new(Router::new())),
            workers: 10
        }
    }

    /// Registers a new `GET` endpoint to the routing.
    pub fn get<T>(&mut self, uri: &str, handle: T)
    where
        T: Fn(&Request) -> Response + 'static + Sync + Send,
    {
        let mut router = self.router.lock().unwrap();
        router.register_route(uri, "GET", handle);
    }

    /// Register a new `HEAD` endpoint to the router.
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
    pub fn match_routes(&self, request: &Request) -> Response {
        let router = self.router.lock().unwrap();
        router.match_route(request)
    }

    /// Returns a reference to server router.
    pub fn router(&self) -> Arc<Mutex<Router>> {
        let router = Arc::clone(&self.router);
        router
    }
}
// GENERAL SERVER RELATED IMPLEMENTATIONS
impl Server {
    /// Starts the server.
    ///
    /// Note you must set endpoints before start the server.
    /// Otherwise, the server won't match against any route.
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

    /// Sets the amount of workers will be spawned, overwriting the defaut value (`10`).
    pub fn set_workers(&mut self, size: usize) {
        self.workers = size;
    }
}