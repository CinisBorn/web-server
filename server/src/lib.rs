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

use http_core::{request::Request, response::Response};
use routing::Router;
use http_parser::response::SerializeResponse;
use connection::Connection;

/// The server
#[allow(missing_debug_implementations)]
pub struct Server {
    router: Router
}

impl Server {
    /// Creates a new instance of [`Server`].
    pub fn new() -> Self {
        Self {
            router: Router::new()
        }
    }

    /// Registers a new `GET` endpoint to the routing. 
    pub fn get<T>(&mut self, uri: &str, handle: T) where 
        T: Fn(&Request) -> Response + 'static
    {
        self.router.register_route(uri, "GET", handle);
    }

    /// Register a new `HEAD` endpoint to the router.
    pub fn head<T>(&mut self, target: &str, handle: T) 
    where  
        T: Fn(&Request) -> Response + 'static
    {
        self.router.register_route(target, "HEAD", handle);
    }

    /// Matchs the request against registered routes. 
    /// 
    /// If the caller registers equal routes, the first is found is matched. 
    pub fn match_routes(&self, request: &Request) -> Response {
        self.router.match_route(request)
    }
}

impl Server {
    /// Starts the server. 
    /// 
    /// Note you must set endpoints before start the server. 
    /// Otherwise, the server won't match against any route.
    pub fn start(&self) -> Result<(), Box<dyn std::error::Error>> {
        let conn = Connection::new()?;

        loop {
            let mut stream = conn.connect()?;    
            let req = http_parser::parse_req(stream.stream_mut())?;

            let mut res = self.match_routes(&req);
    
            stream.send_response(&mut res.serialize(&req.start_line().method()));
        }
    }
}