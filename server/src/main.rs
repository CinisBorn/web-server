use std::io::Read;
use http_parser::HttpParser;
use connection::Connection;

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let conn = Connection::new()?;

    loop {
        let stream = conn.connect()?;
        let mut buf = String::new();

        stream.get_stream().read_to_string(&mut buf)?;
        println!("{}", buf);
        dbg!(stream.parser(buf.as_bytes()).unwrap());
    }
}
