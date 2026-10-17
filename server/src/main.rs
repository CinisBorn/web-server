//! The web server
//!
//! This is the main binary crate for running the server. It doesn't provide any
//! tool, utilities, or anything else.

use std::collections::HashMap;

use connection::Connection;
use http_core::response::{Response, ResponseData, StatusCode};
use http_parser::response::SerializeResponse;
use routing::Router;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let conn = Connection::new()?;

    loop {
        let mut stream = conn.connect()?;
        let req = http_parser::parse_req(stream.stream_mut()).unwrap();
        let mut router = Router::new();

        router.register_route("/", "GET", |_| {
            let mut response = Response::new();

            response.set_status(StatusCode::Ok);
            response.set_body(Some(b"a says: Welcome"));

            response
        });

        router.register_route("/a", "GET", |_| {
            let mut response = Response::new();

            response.set_status(StatusCode::Ok);
            response.set_body(Some(b"b says: Welcome"));

            response
        });

        router.register_route("/a", "HEAD", |_| {
            let mut response = Response::new();
            let mut costum_headers = HashMap::new();

            costum_headers.insert(
                "Funny-Header".to_string(), 
                "Too much fun!".to_string()
            );
            
            response.set_headers(costum_headers);
            response.set_status(StatusCode::Ok);
            response.set_body(Some(b"a says: Welcome HEAD"));

            response
        });

        router.register_route("/", "HEAD", |_| {
            let mut response = Response::new();

            response.set_status(StatusCode::Ok);
            response.set_body(Some(b"/ says: Welcome HEAD"));

            response
        });

        let mut res = router.match_route(&req);

        stream.send_response(&mut res.serialize(&req.start_line().method()));
    }
}
