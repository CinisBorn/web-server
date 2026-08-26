//!  The web server
//!
//! This is the main binary crate for running the server. It doesn't provide any tool, utilities, or
//! anything else.

use connection::Connection;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let conn = Connection::new()?;

    loop {
        let mut stream = conn.connect()?;
        let request = stream.handle_connection();
        dbg!(request.unwrap());
        stream.send_response();
    }
}
