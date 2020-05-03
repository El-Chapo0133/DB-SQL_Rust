use std::net::{TcpStream};
use std::io::{Read,Write};


struct Client {
        stream: TcpStream,
}

impl Client {
        pub fn new() -> Option<Client> {
                let ip_port: String = String::from("172.20.20.100:5000");
                match TcpStream::connect(ip_port) {
                        Ok(stream) => { 
                                Some(Client {stream: stream})
                        },
                        Err(e) => {
                                println!("Failed to connect : {}", e);
                                None
                        }
                }
        }
        pub fn send(&mut self, message: Vec<u8>) -> bool {
                self.stream.write(&message).unwrap();
                let mut data: Vec<u8> = vec![0; message.len()];
        
                match self.stream.read_exact(&mut data) {
                        Ok(_) => {
                                if &message == &data {
                                        println!("Reply is ok!");
                                        true
                                } else {
                                        println!("unexpected reply : {}", std::str::from_utf8(&data).unwrap());
                                        false
                                }
                        },
                        Err(e) => {
                                println!("Failed to receive data : {}", e);
                                false
                        }
                }
        }
}



/*
main :

mod tcp;
fn main() {
        let client: Option<my::Client> = my::Client::new();
        match client {
                Some(mut c) => {
                        let result: bool = c.send("Hey!".as_bytes().to_vec());
                        if result {
                                println!("Message and reply is ok");
                        } else {
                                println!("Error somewhere ¯\\_(°°)_/¯");
                        }
                }
                None => {
                        println!("Error, stream not setted");
                }
        }
}
*/