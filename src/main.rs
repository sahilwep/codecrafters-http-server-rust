use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

use std::io::BufRead;
use std::io::BufReader;



use bytes::Bytes;
use tokio::stream;

#[derive(Debug)]
enum HTTPMethod {
    GET,
    POST,
    PUT,
    DELETE,
}

impl HTTPMethod {
    fn new(content: &str) -> Option<HTTPMethod> {
        match content {
            "GET" => Some(HTTPMethod::GET),
            "POST" => Some(HTTPMethod::POST),
            "PUT" => Some(HTTPMethod::PUT),
            "DELETE" => Some(HTTPMethod::DELETE),
            _ => None,
        }
    }
}

#[derive(Debug)]
struct HTTPRequest {
    method: HTTPMethod,
    path: String,
    http_version: String,
    host: String,
    user_agent: String,
}

impl HTTPRequest {
    fn new(stream: &mut TcpStream) -> Option<HTTPRequest> {
        let buf_reader = BufReader::new(stream);

        let http_request: Vec<String> = buf_reader
            .lines()
            .map(| result| result.unwrap())
            .take_while(|line| !line.is_empty())
            .collect();

        println!("raw request: {:?}", http_request);
        let mut first_line = http_request[0].split(" ");
        let request: HTTPRequest = HTTPRequest {
            method: HTTPMethod::new(first_line.next().unwrap()).unwrap(),
            path: first_line.next().unwrap().into(),
            http_version: first_line.next().unwrap().into(),
            host: "".into(),
            user_agent: "".into(),
        };
        Some(request)
    }
}

fn handel_stream(mut stream: TcpStream) -> () {
    let request = HTTPRequest::new(&mut stream);
    println!("Handling Connection");
    println!("{:?}", request);
    let response = | request: HTTPRequest | -> &'static [u8] {
        match request.path.as_str() {
            "/" => b"HTTP/1.1 200 OK \r\n\r\n",
            _ => b"HTTP/1.1 404 Not Found\r\n\r\n",
        }
    }(request.unwrap());
    let _ = stream.write(response).unwrap();
}



fn main() {

    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => handel_stream(stream),
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
