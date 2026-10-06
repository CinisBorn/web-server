//!  The web server
//!
//! This is the main binary crate for running the server. It doesn't provide any tool, utilities, or
//! anything else.

use connection::Connection;
use http_parser::types::Method;
use http_parser::response;
use routing::Router;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let conn = Connection::new()?;
    
    loop {
        let mut stream = conn.connect()?;
        let req = http_parser::parse_req(stream.stream_mut()).unwrap();
        let router = Router::new(req);

        router.listen("/", "GET", || {
            let mut res = response::Response::build(
                response::CodeStatus::Ok, 
                Some(b"Hello World")
            );

            stream.send_response(&res.serialize(&Method::GET));
        });

        router.listen("/", "HEAD", || {
            let mut res = response::Response::build(
                response::CodeStatus::Ok, 
                Some(b"Hello World alternative")
            );

            stream.send_response(&res.serialize(&Method::HEAD));
        });
    }
}
