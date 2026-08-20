use connection::Connection;

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let conn = Connection::new()?;

    loop {
        let mut stream = conn.connect()?;
        let request = stream.parse_request();

        dbg!(request.unwrap());
    }
}
