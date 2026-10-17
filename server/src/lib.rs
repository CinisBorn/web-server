//! Provides tools for server handling. 

use http_core::{request::Request, response::Response};
use routing::Router;

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