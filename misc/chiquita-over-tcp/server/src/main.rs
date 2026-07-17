use std::io::{Read, Write};
use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    println!("Server listening on port 7878");

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let mut buffer = [0; 1024];
                stream.read(&mut buffer).unwrap();
                println!("Received: {:?}", String::from_utf8_lossy(&buffer));
                stream.write(b"Message received!").unwrap();
            }
            Err(e) => println!("Connection failed: {}", e),
        }
    }
}
