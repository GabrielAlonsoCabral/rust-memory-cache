use std::borrow::Cow;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::thread;

fn main() {
    let host: &str = "127.0.0.1:6380";
    let listener: TcpListener = TcpListener::bind(host).unwrap();
    println!("Stupid redis is listening on {}", host);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                thread::spawn(move || handle_client(stream));
            }
            Err(e) => {
                println!("Error: {}", e);
                /* connection failed */
            }
        }
    }
}

fn handle_client(mut stream: TcpStream) {
    let mut data: [u8; 1024] = [0 as u8; 1024]; // using 50 bytes buffer

    stream.read(&mut data).unwrap();
    let client_input: String = get_input_message(&data);
    println!("Received: {:?}", client_input);

    if client_input.to_lowercase() == "ping" {
        write_message(&mut stream, "pong\n");
    }

    shutdown(&mut stream);
}

fn get_input_message(buffer: &[u8; 1024]) -> String {
    let client_input: Cow<str> = String::from_utf8_lossy(buffer);

    return client_input.replace("\0", "").replace("\n", "");
}

fn write_message(stream: &mut TcpStream, message: &str) {
    stream.write(message.as_bytes()).unwrap();
}

fn shutdown(stream: &mut TcpStream) {
    stream.write("Closing connection!".as_bytes()).unwrap();
    //stream.shutdown(Shutdown::Both).unwrap();
}
