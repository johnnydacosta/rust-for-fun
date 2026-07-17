use std::io::{self, Read, Write};
use std::net::TcpStream;

fn main() {
    loop {
        let mut stream = TcpStream::connect("127.0.0.1:7878").expect("Could not connect");
        println!("Please input your message");

        let mut msg = String::new();
        io::stdin()
            .read_line(&mut msg)
            .expect("Failed to read message.");

        let msg = msg.as_bytes();
        stream.write(msg).expect("Failed to send message");
        let mut response = String::new();
        stream
            .read_to_string(&mut response)
            .expect("Failed to read");
        println!("Server says: {}", response);
    }
}
