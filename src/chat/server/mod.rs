mod html;

use html::HtmlReader;
use std::error::Error;
use std::io::{Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream};

pub struct Server {
    listener: TcpListener,
    address: String,
    port: u16,
}

impl Default for Server {
    fn default() -> Self {
        Server {
            listener: TcpListener::bind("0.0.0.0:3000").unwrap(),
            address: String::from("0.0.0.0"),
            port: 3001,
        }
    }
}

impl Server {
    pub fn start(&mut self) {
        self.listener = TcpListener::bind(format!("{}:{}", &self.address, &self.port)).unwrap();

        println!("Running server at address: {}:{}", "0.0.0.0", &self.port);

        for stream in self.listener.incoming() {
            self.handle_incoming(stream);
        }
    }

    pub fn set_port(&mut self, port: u16) {
        self.port = port;
        self.listener = TcpListener::bind(format!("{}:{}", &self.address, port)).unwrap();
    }

    fn handle_incoming(&self, streamOption: Result<TcpStream, impl Error>) {
        let mut stream = streamOption.expect("Error with incoming connection");
        let mut data = [0u8; 3];

        if let Err(e) = stream.read(&mut data) {
            println!("Failed to read from stream: {}", e);
            return;
        }

        match String::from_utf8_lossy(&data).to_string().as_str() {
            "GET" => self.handle_get_request(&mut stream),
            _ => println!("Not a GET request"),
        }

        if let Err(e) = stream.shutdown(Shutdown::Both) {
            println!("Shutdown error: {}", e);
        }
    }

    fn handle_get_request(&self, stream: &mut TcpStream) {
        let message = String::from("Hello, World!");
        let response = format!(
            "HTTP/1.1 200 OK\r\n\
          Content-Type: text/plain\r\n\
          Content-Length: {}\r\n\
          Connection: close\r\n\
          \r\n\
          {}",
            message.len(),
            message
        );

        match stream.write(response.as_bytes()) {
            Ok(_) => println!("Write successful"),
            Err(e) => println!("Write failed: {}", e),
        }
    }
}
