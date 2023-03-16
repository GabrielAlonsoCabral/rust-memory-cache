use std::borrow::Cow;
use std::io::{prelude::*, BufReader};
use std::net::{TcpListener, TcpStream};
use std::thread;

extern crate resp;
use resp::{encode, encode_slice, Decoder, Value};

fn main() {
    let host: &str = "127.0.0.1:6380";
    let listener: TcpListener = TcpListener::bind(host).unwrap();
    println!("Stupid redis is listening on {}\n", host);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}\n", stream.peer_addr().unwrap());
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

    if client_input.to_lowercase() == "ping" {
        write_message(&mut stream, "pong\n");
    }

    shutdown(&mut stream);
}

fn get_input_message(buffer: &[u8; 1024]) -> String {
    let mut decoded_message: Decoder<&[u8]> = Decoder::new(BufReader::new(buffer.as_slice()));

    let legible_message: String = decoded_message.decode().unwrap().to_beautify_string();

    println!("Received: {:?}\n", legible_message);

    return legible_message;
}

fn write_message(stream: &mut TcpStream, message: &str) {
    let encoded_message: String = encode_message(message);

    stream.write(encoded_message.as_bytes()).unwrap();
}

fn encode_message(message: &str) -> String {
    let val: Value = Value::String(message.to_string());
    return val.to_encoded_string().unwrap();
}

fn shutdown(stream: &mut TcpStream) {
    stream.write("Closing connection!\n".as_bytes()).unwrap();
    //stream.shutdown(Shutdown::Both).unwrap();
}
