//! Provides a simple mechanism of routing
use http_parser::{request::{Request}, types::Method};
use http_parser::response::{Response, CodeStatus};

/// Represents a target route
#[allow(missing_debug_implementations)]
pub struct Route {
    /// The target resource.
    pub target: String,
    /// The HTTP method.
    pub method: Method,
    /// The handle
    pub handle: Box<dyn Fn(&Request) -> Response>
}

/// The Router
#[allow(missing_debug_implementations)]
pub struct Router {
    routes: Vec<Route>,
}

impl Router {
    /// Creates a new instance of [`Router`].
    pub fn new() -> Self {
        Self { routes: Vec::new() }
    }
    
    /// Registers a new [`Route`] in the [`Router`].
    pub fn register_route<F>(
        &mut self, 
        target: &str, 
        method: &str, 
        handle: F
    ) where 
        F: Fn(&Request) -> Response + 'static
    {
        self.routes.push(Route { 
            target: target.to_string(), 
            method: Method::map_method(method), 
            handle: Box::new(handle)
        });
    }

    /// Match the request againts a route.
    pub fn match_route(&self, request: &Request) -> Response {
        for route in &self.routes {
            let is_method = route.method == *request.start_line().method();
            let is_target = route.target == request.start_line().target();
            
            if is_method && is_target {
                return (route.handle)(request);
            }
        }

        Response::build(
            CodeStatus::NotFound,
            Some(b"Not Found"),
        )
    }
}

