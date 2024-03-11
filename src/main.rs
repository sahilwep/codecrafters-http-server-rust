use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    thread,
};
use std::{fs::File, path::Path};

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
    let mut buffer = [0; 1600];
    stream.read(&mut buffer).unwrap();
    let request = String::from_utf8_lossy(&buffer[..]);
    println!("request: {}", request);
    let lines: Vec<&str> = request.lines().collect();
    let request_line_parts: Vec<&str> = lines[0].split_whitespace().collect();
    let mut method = String::new();
    let mut path = String::new();
    let mut http_version = String::new();
    if request_line_parts.len() == 3 {
        method = request_line_parts[0].to_string();
        path = request_line_parts[1].to_string();
        http_version = request_line_parts[2].to_string();
    }
    let method = method.as_str();
    let path = path.as_str();
    let http_version = http_version.as_str();
    println!("Method: {}, path: {},version:{}",method, path, http_version);

    // matching pattern:
    match path {
        "/" => stream
            .write_all("HTTP/1.1 200 OK\r\n\r\n".as_bytes())
            .unwrap(),
        "/user-agent" => {
            let user_agent_line_parts: Vec<&str> = lines[2].splitn(2, ": ").collect();
            let user_agent = user_agent_line_parts[1].to_string();
            let res: String = format!(
                "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
                user_agent.len(),
                user_agent
            );
            stream.write_all(res.as_bytes()).unwrap();
        }
        _ if path.starts_with("/files/") => {
            let args = parse_args();
            let file = path.replace("/files/", "");
            let directory = args.directory.clone().unwrap();
            match method {
                "GET" => {
                    if args.directory.is_none() {
                        stream
                            .write_all("HTTP/1.1 404 Not Found\r\n\r\n".as_bytes())
                            .unwrap();
                    } else if !dir_exists(directory.as_str()) {
                        stream
                            .write_all("HTTP/1.1 404 Not Found\r\n\r\n".as_bytes())
                            .unwrap();
                    } else if !file_exists(
                        format!("{}{}", directory.as_str(), file.as_str()).as_str(),
                    ) {
                        stream
                            .write_all("HTTP/1.1 404 Not Found\r\n\r\n".as_bytes())
                            .unwrap();
                    } else {
                        let contents =
                            read_file(format!("{}{}", directory, file.as_str()).as_str());
                        let res = format!("HTTP/1.1 200 OK\r\nContent-Type: application/octet-stream\r\nContent-Length: {}\r\n\r\n{}", contents.len(), contents);
                        stream.write_all(res.as_bytes()).unwrap();
                    }
                }
                "POST" => {
                    let mut body = lines[6].to_string();
                    body = body.replace("\x00", "");
                    println!("body: {}", &body);
                    let file_path = format!("{}{}", directory, file);
                    save_file(&file_path, &body);
                    stream
                        .write_all("HTTP/1.1 201 OK\r\n\r\n".as_bytes())
                        .unwrap();
                }
                _ => stream
                    .write_all("HTTP/1.1 404 Not Found\r\n\r\n".as_bytes())
                    .unwrap(),
            }
        }
        _ if path.starts_with("/echo/") => {
            let text = path.split("/echo/").skip(1).collect::<String>();
            let res = format!(
                "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
                text.len(),
                text
            );
            stream.write_all(res.as_bytes()).unwrap();
        }
        _ => stream
            .write_all("HTTP/1.1 404 Not Found\r\n\r\n".as_bytes())
            .unwrap(),
    }
}


pub fn save_file(path: &str, content: &str) {
    let path = Path::new(path);
    let display = path.display();
    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file) => file,
    };
    match file.write_all(content.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", display, why),
        Ok(_) => println!("successfully wrote to {}", display),
    }
}

fn dir_exists(path: &str) -> bool {
    let path = Path::new(path);
    path.exists() && path.is_dir()
}


fn file_exists(path: &str) -> bool {
    let path = Path::new(path);
    path.exists() && path.is_file()
}


fn read_file(path: &str) -> String {
    let path = Path::new(path);
    let display = path.display();
    println!("Reading file: {}", display);
    let mut file = match File::open(path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };
    let mut buffer = [0 as u8; std::u8::MAX as usize].to_vec();
    match file.read_to_end(&mut buffer) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => {
            let content = String::from_utf8(buffer).unwrap();
            content.replace("\0", "")
        }
    }
}


#[derive(Clone, Debug)]
struct Args {
    directory: Option<String>,
}


fn parse_args() -> Args {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        Args { directory: None }
    } else {
        Args {
            directory: Some(args[2].clone()),
        }
    }
}



fn main() {

    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                // using threads to handel multiple concurrent connections.
                thread::spawn(  || {
                    handel_stream(stream);
                });
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
