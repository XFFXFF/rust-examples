use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    let size = stream.read(&mut buffer).unwrap();
    let request = String::from_utf8_lossy(&buffer[..size]);
    let request = request.replace("\r\n", "");
    if request.starts_with("PING") {
        let argument = request.trim_start_matches("PING").trim();
        if argument.len() == 0 {
            stream.write(b"+PONG\r\n").unwrap();
        } else {
            let mut response = String::from("$");
            response.push_str(&argument.len().to_string());
            response.push_str("\r\n");
            response.push_str(argument);
            response.push_str("\r\n");
            stream.write(response.as_bytes()).unwrap();
        }
    }
}
