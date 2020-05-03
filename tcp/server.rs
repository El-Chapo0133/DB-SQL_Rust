use std::thread;
use std::net::{TcpListener,TcpStream,Shutdown};
use std::io::{Read, Write};


pub struct Server {
        listener: TcpListener,
}

impl Server {
        pub fn new() -> Server {
                let ip_port = String::from("172.20.20.100:5000");
                Server {listener: TcpListener::bind(ip_port).unwrap()}
        }
        pub fn listen(&self) {
                for stream in self.listener.incoming() {
                        match stream {
                                Ok(stream) => {
                                        println!("New connection: {}", stream.peer_addr().unwrap());
                                        thread::spawn(move|| {
                                                handle_thread(stream);
                                        });
                                },
                                Err(e) => {
                                        println!("Argg.. {}", e);
                                }
                        }
                }
                drop(&self.listener);
        }
}




fn handle_thread(mut stream: TcpStream) {
        let mut buffer: [u8; 255] = [0; 255];
        while match stream.read(&mut buffer) {
                Ok(size) => {
                        if size != 0 {
                                // TODO : spawn thread, export with the self.data value, then reset self.data
                                export_thread(buffer[0..size].to_vec());
                        }
                        write_on_stream(&stream, &buffer);
                        true
                },
                Err(e) => {
                        stream_read_error(&stream, e)
                }
        } {};
}

fn write_on_stream(mut stream: &TcpStream, data: &[u8; 255]) {
        stream.write(data).unwrap();
}

fn stream_shutdown(stream: &TcpStream) {
        stream.shutdown(Shutdown::Both).unwrap();
}


fn stream_read_error(stream: &TcpStream, e: std::io::Error) -> bool {
        println!("An error occurred, terminating connection with {0}: {1}", stream.peer_addr().unwrap(), e);
        stream_shutdown(&stream);
        false
}

fn export_thread(data: Vec<u8>) {
        match std::str::from_utf8(&data) {
                Ok(s) => println!("{}", s),
                Err(e) => println!("Cannot convert [u8] to str: {}", e)
        }
}