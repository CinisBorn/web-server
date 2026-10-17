//! The web server
//!
//! This is the main binary crate for running the server. It doesn't provide any
//! tool, utilities, or anything else.

use connection::Connection;
use http_core::response::{Response, ResponseData, StatusCode};
use http_parser::response::{SerializeResponse};

use server::Server;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let conn = Connection::new()?;
    let mut server = Server::new();
    
    loop {
        let mut stream = conn.connect()?;
        let req = http_parser::parse_req(stream.stream_mut()).unwrap();
  
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

        let mut res = server.match_routes(&req);

        stream.send_response(&mut res.serialize(&req.start_line().method()));
    }
}
