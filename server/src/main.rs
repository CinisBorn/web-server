//!  The web server
//!
//! This is the main binary crate for running the server. It doesn't provide any tool, utilities, or
//! anything else.

use connection::Connection;
use http_parser::response::{CodeStatus, Response};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let conn = Connection::new()?;

    loop {
        let mut stream = conn.connect()?;
        let mut buf = [0u8; 8192];
        let req = http_parser::parse_req(stream.stream_mut(), &mut buf).unwrap();
        let mut response = Response::build(CodeStatus::Ok, b"HELLO WORLD FROM SERVER-SIDE");
        let method = req.start_line().method();
        

        dbg!(&response.serialize(method));
        stream.send_response(response.serialize(method).as_slice());
    }
}
