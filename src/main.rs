use std::io::{prelude::*, BufReader};
use std::net::{TcpListener, TcpStream};
use stupid_redis::{MemCache, find_data_by_key};

extern crate resp;
use resp::{Decoder, Value};

fn main() {
    let host: &str = "127.0.0.1:6379";
    let listener: TcpListener = TcpListener::bind(host).unwrap();
    println!("Stupid redis is listening on {}\n", host);
    let mut mem_cache:Vec<MemCache>=vec![];


    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}\n", stream.peer_addr().unwrap());
                handle_client(stream, &mut mem_cache)
            }
            Err(e) => {
                println!("Error: {}", e);
                /* connection failed */
            }
        }
    }
}


fn handle_client(mut stream: TcpStream, mem_cache:&mut Vec<MemCache>) {
    let mut data: [u8; 1024] = [0 as u8; 1024]; // using 50 bytes buffer

    stream.read(&mut data).unwrap();
    let client_input: String = get_input_message(&data);

    if client_input.to_lowercase() == "ping" {
        write_message(&mut stream, "pong\n");
        return 
    }

    let mut client_input_splited = client_input.split(" ");

    let  method:Option<&str> = client_input_splited.nth(0);
    let  key:Option<&str> = client_input_splited.nth(0);
    let value: Option<&str> = client_input_splited.nth(0);
    let ttl: Option<&str> = client_input_splited.nth(0);
    
    if method.is_none(){
        write_message(&mut stream, &String::from("Method is required"));
        return
    }

    if key.is_none(){
        write_message(&mut stream, &String::from("Key is required"));
        return
    }

    if method.unwrap()=="set" && value.is_none(){
        write_message(&mut stream, &String::from("Value is required"));
        return
    }

    if method.unwrap()=="set" && ttl.is_none(){
        write_message(&mut stream, &String::from("TTL is required"));
        return
    }

    if method.unwrap() == "set"{
        mem_cache.push(
            MemCache::new(&String::from(key.unwrap()), value.unwrap().as_bytes().to_vec(), &ttl.unwrap().parse::<u32>().unwrap())
        );
    }

    if method.unwrap() == "get"{
        let data:Option<&MemCache> = find_data_by_key(mem_cache, String::from(key.unwrap()));
        if data.is_none(){
            write_message(&mut stream, &String::from(""));
            return
        }

        write_message(&mut stream, &String::from_utf8(data.unwrap().data.to_owned()).unwrap());
     
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
