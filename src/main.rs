use std::io::Write;
// Uncomment this block to pass the first stage
use std::net::TcpListener;
use std::{io, string};


fn main() {

    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    
    for stream in listener.incoming() {
        match stream {
            Ok(mut _string) => {
                println!("accepted new connection");
                let data = "HTTP/1.1 200 OK\r\n\r\n";
                _string.write_all(data.as_bytes()).unwrap();
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
