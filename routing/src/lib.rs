//! Provides a simple mechanism of routing
use http_parser::{request::Request, types::Method};

/// Represents a target route
#[derive(Debug)]
pub struct Route {
    /// The target resource.
    pub target: String,
    /// The HTTP method.
    pub method: Method,
}

/// The Router
#[derive(Debug)]
pub struct Router {
    routes: Vec<Route>,
    request: Request,
}

impl Router {
    /// Creates a new instance of [`Router`].
    pub fn new(req: Request) -> Self {
        Self { routes: Vec::new(), request: req}
    }

    /// Registers a new [`Route`] in the [`Router`].
    pub fn register_route(&mut self, route: Route) {
        self.routes.push(route);
    }

    /// Listens to a endpoint
    pub fn listen<T>(&self, uri: &str, method: &str, handle: T) 
    where 
        T: FnOnce() -> ()
    {
        let method = Method::map_method(method);
        let req_method = *self.request.start_line().method();
        let req_target = self.request.start_line().target(); 
        
        if req_method == method && req_target == uri {
            handle()
        }
    }
}

