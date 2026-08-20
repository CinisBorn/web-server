use std::io::Read;
use http_parser::HttpParser;
use connection::Connection;

fn main() {
    let conn = match Connection::new() {
        Ok(conn) => conn,
        Err(e) => panic!("{}", e.to_string()),
    };

    loop {
        let stream = conn.connect().unwrap();
        let mut buf = String::new();

        stream.get_stream().read_to_string(&mut buf).unwrap();
        println!("{}", buf);
        dbg!(stream.parser(buf.as_bytes()).unwrap());
    }
}
