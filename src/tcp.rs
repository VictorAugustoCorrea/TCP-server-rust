use std::io::{Write, Read};
use std::net:: TcpStream;

pub fn handle_client(mut stream: TcpStream){
    /** Buffer to read data */
    let mut buffer = [0; 1024];
    /** Read and store data in the buffer */
    stream.read(&mut buffer).expect("Failed to read from client");
    /** Convert data into a UTF-8 encoded string */
    let request = String::from_utf8_lossy(&buffer[..]);
    println!("Received request: {}", request);

    let response = "Hello, Client!".as_bytes();
    stream.write(response).expect("Failed to write response");
}