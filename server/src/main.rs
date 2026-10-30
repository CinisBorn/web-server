//! The server binary.

use http_core::response::{Response, ResponseData, StatusCode};
use server::Router;
use server::Server;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut router = Router::new();

    router.get("/test", |_| {
        let mut response = Response::new();
        response.set_status(StatusCode::Ok);
        response.set_body(Some(b"Everything is OK"));
        
        response
    });

    router.head("/test", |_| {
        let mut response = Response::new();
        response.set_status(StatusCode::Ok);

        response
    });
    
    Server::new("127.0.0.1:7878").start(router)?;

    Ok(())
}
