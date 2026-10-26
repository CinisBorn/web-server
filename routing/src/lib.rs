//! Provides a simple mechanism of routing

use http_core::Method;
use http_core::request::Request;
use http_core::response::{Response, ResponseData, StatusCode};

use std::sync::Arc;

/// Represents a target route
#[allow(missing_debug_implementations)]
pub struct Route {
    /// The target resource.
    pub target: String,
    /// The HTTP method.
    pub method: Method,
    /// The handle
    pub handle: Arc<dyn Fn(&Request) -> Response + Send + Sync + 'static>,
}

/// The Router
#[allow(missing_debug_implementations)]
pub struct Router {
    routes: Vec<Route>,
}

impl Default for Router {
    fn default() -> Self {
        Self::new()
    }
}

impl Router {
    /// Creates a new instance of [`Router`].
    pub fn new() -> Self {
        Self { routes: Vec::new() }
    }

    /// Registers a new [`Route`] in the [`Router`].
    pub fn register_route<F>(&mut self, target: &str, method: &str, handle: F)
    where
        F: Fn(&Request) -> Response + 'static,
        F: Send + Sync,
    {
        self.routes.push(Route {
            target: target.to_string(),
            method: Method::map_method(method),
            handle: Arc::new(handle),
        });
    }

    /// Match the request againts a route.
    pub fn match_route(&self, request: &Request) -> Response {
        for route in &self.routes {
            let req_target = request.start_line().target();

            if is_target_route(route, request) {
                return check_possible_routes(&self.routes, req_target, request);
            }

            if is_route(route, request) {
                return (route.handle)(request);
            }
        }

        let mut response = Response::new();

        response.set_status(StatusCode::NotFound);
        response.set_body(Some(b"Not Found"));

        response
    }

    /// Registers a new `GET` endpoint to the routing.
    pub fn get<T>(&mut self, uri: &str, handle: T)
    where
        T: Fn(&Request) -> Response + 'static + Sync + Send,
    {
        self.register_route(uri, "GET", handle);
    }

    /// Register a new `HEAD` endpoint to the router.
    pub fn head<T>(&mut self, target: &str, handle: T)
    where
        T: Fn(&Request) -> Response + 'static + Sync + Send,
    {
        self.register_route(target, "HEAD", handle);
    }
}

/// Checks if the route match testing `route` against a `request`.
///
/// The `switch` parameter is used for flow control where I can test:
///
/// - If both route and method match
/// - If route match, but the method not
///
/// Set `true` for the first case; Set `false` for the second case.
fn is_route(route: &Route, request: &Request) -> bool {
    let is_method = route.method == *request.start_line().method();
    let is_target = route.target == request.start_line().target();

    is_method && is_target
}

fn is_target_route(route: &Route, request: &Request) -> bool {
    let is_method = route.method == *request.start_line().method();
    let is_target = route.target == request.start_line().target();

    is_target && !is_method 
}

fn check_possible_routes(routes: &[Route], req_target: &str, request: &Request) -> Response {
    let possible_routes = routes
        .iter()
        .filter(|e| e.target == req_target)
        .collect::<Vec<&Route>>();

    for route in &possible_routes {
        if is_route(route, request) {
            return (route.handle)(request);
        }
    }

    let allowed_methods = possible_routes
        .iter()
        .map(|e| e.method)
        .collect::<Vec<Method>>();

    let mut response = Response::new();

    response.set_status(StatusCode::MethodNotAllowed(allowed_methods));
    response.set_body(Some(b"Method Not Allowed"));

    response
}
