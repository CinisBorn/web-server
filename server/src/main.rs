//! The server binary.

use http_core::response::{
    Response, 
    ResponseData, 
    StatusCode
};

use server::Server;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut server = Server::new();

    server.get("/test", |_| {
        let mut response = Response::new();
        response.set_status(StatusCode::Ok);
        response.set_body(Some(b"Everything is OK"));
        response
    });
    
    server.head("/test", |_| {
        let mut response = Response::new();
        response.set_status(StatusCode::Ok);
        
        response
    });

    server.start()?;

    Ok(())
}
