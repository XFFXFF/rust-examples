use std::net::TcpStream;
use std::io::prelude::*;

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:7878").unwrap();
    stream.write(b"hello").unwrap();
}
