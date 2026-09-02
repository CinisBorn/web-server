//!  The web server
//!
//! This is the main binary crate for running the server. It doesn't provide any tool, utilities, or
//! anything else.

use connection::Connection;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let conn = Connection::new()?;

    loop {
        let mut stream = conn.connect()?;
        let mut buf = [0u8; 8192];
        let request = http_parser::parse_req(stream.stream_mut(), &mut buf);

        dbg!(request.unwrap());
        stream.send_response();
    }
}
