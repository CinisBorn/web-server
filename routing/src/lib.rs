//! Provides a simple mechanism of routing

use http_core::request::Request;
use http_core::response::{StatusCode, Response, ResponseData};
use http_core::Method;

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
        'outer: loop {      
            for route in &self.routes {
                let req_target = request.start_line().target();
 
                if is_target_route(route, request) {
                    break 'outer check_possible_routes(
                        &self.routes, 
                        req_target, 
                        request
                    )
                }
                
                if is_route(route, request) {
                    return (route.handle)(request);
                }
            }

            let mut response = Response::new();

            response.set_status(StatusCode::NotFound);
            response.set_body(Some(b"Not Found"));
            
            break response
        }
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
    
    !is_method || is_target
}

fn check_possible_routes(
    routes: &Vec<Route>, 
    req_target: &str, 
    request: &Request
) -> Response {
    let possible_routes = routes
        .iter()
        .filter(|e| e.target == req_target)
        .collect::<Vec<&Route>>();

    for route in possible_routes.iter() {
        if is_route(route, request) {
            return (route.handle)(request)
        }
    }

    let allowed_methods = possible_routes.iter()
        .map(|e| e.method)
        .collect::<Vec<Method>>();

    let mut response = Response::new();

    response.set_status(StatusCode::MethodNotAllowed(allowed_methods));
    response.set_body(Some(b"Method Not Allowed"));

    return response;
}
