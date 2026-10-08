//!  The web server
//!
//! This is the main binary crate for running the server. It doesn't provide any tool, utilities, or
//! anything else.

use connection::Connection;
use http_parser::response::{CodeStatus, Response};
use routing::Router;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let conn = Connection::new()?;
    
    loop {
        let mut stream = conn.connect()?;
        let req = http_parser::parse_req(stream.stream_mut()).unwrap();
        let mut router = Router::new();

        router.register_route("/", "GET", |_| {
            Response::build(
                CodeStatus::Ok, 
                Some(b"Hello World")
            )
        });

        router.register_route("/", "HEAD", |_| {
            Response::build(
                CodeStatus::Ok, 
                Some(b"Hello World Alternative")
            )
        });

        let mut res = router.match_route(&req);

        stream.send_response(&mut res.serialize(&req.start_line().method()));
    }
}
