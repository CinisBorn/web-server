use std::io::Read;

use net::Connection;

fn main() {
    let conn = Connection::new().unwrap();

    loop {
        let stream = conn.connect().unwrap();
        let mut buf = String::new();

        stream.get_stream().read_to_string(&mut buf).unwrap();
        println!("{}", buf);
    }
}
