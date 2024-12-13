mod html;

use html::HtmlReader;
use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use sha1::{Sha1, Digest};

type ConnectionMap = Arc<Mutex<HashMap<String, TcpStream>>>;

pub struct Server {
    listener: TcpListener,
    address: String,
    port: u16,
    connections: ConnectionMap
}

impl Default for Server {
    fn default() -> Self {
        Server {
            listener: TcpListener::bind("0.0.0.0:3002").unwrap(),
            address: String::from("0.0.0.0"),
            port: 3002,
            connections: Arc::new(Mutex::new(HashMap::new()))
        }
    }
}

impl Server {
    pub fn start(&mut self) {
        println!("Running server at address: {}:{}", &self.address, &self.port);

        let connections = self.connections.clone();

        let listener = &self.listener.try_clone().unwrap();
        for stream in listener.incoming() {
            let connections = Arc::clone(&connections);

            std::thread::spawn(move || {
                if let Ok(mut stream) = stream {
                    Self::handle_incoming(stream, connections);
                }
            });
        }
    }

    pub fn set_port(&mut self, port: u16) {
        self.port = port;
        self.listener = TcpListener::bind(format!("{}:{}", &self.address, port)).unwrap();
    }

    fn handle_incoming(mut stream: TcpStream, connections: ConnectionMap) {
        let mut buffer = [0; 1024];
        let mut data = String::new();
        
        // Read in chunks until we find the end of headers
        loop {
            match stream.read(&mut buffer) {
                Ok(n) if n == 0 => return, // Connection closed
                Ok(n) => {
                    println!("{}", data);

                    data.push_str(&String::from_utf8_lossy(&buffer[..n]));
                    if data.contains("\r\n\r\n") {
                        break;
                    }
                }
                Err(e) => {
                    println!("Failed to read from stream: {}", e);
                    return;
                }
            }
        }

        let (request_type, path, headers) = Self::get_request_args(&data);
        
        match request_type {
            "GET" => Self::handle_get_request(&mut stream, &path, &headers, connections),
            _ => println!("Not a GET request"),
        }

        // if let Err(e) = stream.shutdown(Shutdown::Both) {
        //     println!("Shutdown error: {}", e);
        // }
    }

    fn handle_get_request(stream: &mut TcpStream, path: &str, headers: &HashMap<String, String>, connections: ConnectionMap) {
        match path {
            // "/" => Routes::handle_index(stream),
            "/connection" => Routes::handle_connection(connections, stream, headers),
            _ => println!("Unknown path")
        }
    }

    fn get_request_args(string: &String) -> (&str, &str, HashMap<String, String>) {
        let mut splits = string.split("\n");
        
        // Get first line like "GET /path HTTP/1.1"
        let first_line = splits.next().unwrap_or("GET / HTTP/1.1");
        
        // Split the first line by spaces
        let mut parts = first_line.split_whitespace();
        let request_type = parts.next().unwrap_or("GET");
        let path = parts.next().unwrap_or("/");

        let mut headers: HashMap<String, String> = HashMap::new();

        while let Some(line) = splits.next() {
            let mut sub_split = line.split(": ");
            
            // Get the key, break if None
            let key = match sub_split.next() {
                Some(k) => k.trim(),
                None => break,
            };
            
            // Get the value, break if None
            let value = match sub_split.next() {
                Some(v) => v.trim(),
                None => break,
            };

            headers.insert(key.to_string(), value.to_string());
        }

        (request_type, path, headers)
    }
}

struct Routes {}

impl Routes {
    // fn handle_index(stream: &mut TcpStream) {
    //     let html = HtmlReader::read_file("", 0).unwrap();

    //     let headers = format!(
    //         "HTTP/1.1 200 OK\r\n\
    //       Content-Type: text/html\r\n\
    //       Content-Length: {}\r\n\
    //       Connection: close\r\n\
    //       \r\n",
    //       html.len(),
    //     );

    //     let mut response = headers.into_bytes();
    //     response.extend(html);

    //     match stream.write(&response) {
    //         Ok(_) => println!("Write successful"),
    //         Err(e) => println!("Write failed: {}", e),
    //     }
    // }

    fn handle_connection(connections: ConnectionMap, stream: &mut TcpStream, headers: &HashMap<String, String>) {
        let websocket_secret = headers.get("Sec-WebSocket-Key").expect("This must be defined");

        let response_headers = format!(
            "HTTP/1.1 101 Switching Protocols\r\n\
            Upgrade: websocket\r\n\
            Connection: Upgrade\r\n\
            Sec-Websocket-Accept: {}\r\n\
            \r\n",
            Self::get_websocket_hash(websocket_secret)
        );

        let _ = stream.write(&response_headers.into_bytes());

        {
            let mut connections = connections.lock().unwrap();
            connections.insert(websocket_secret.to_owned(), stream.try_clone().unwrap());
        }

        Self::spawn_websocket_listener(stream, &connections);
    }

    fn get_websocket_hash(original: &String) -> String {
        let full = original.to_string() + "258EAFA5-E914-47DA-95CA-C5AB0DC85B11";
        let mut hasher = Sha1::new();
        hasher.update(full);
        
        base64.encode(hasher.finalize())
    }

    fn spawn_websocket_listener(stream: &mut TcpStream, connections: &ConnectionMap) {
        let mut buffer = [0; 1024];
        loop {
            match stream.read(&mut buffer) {
                Ok(n) if n == 0 => break,
                Ok(n) => {
                    // Parse WebSocket frame
                    let fin = (buffer[0] & 0b10000000) != 0;
                    let opcode = buffer[0] & 0b00001111;
                    let masked = (buffer[1] & 0b10000000) != 0;
                    let payload_len = (buffer[1] & 0b01111111) as usize;
                    
                    // Handle different opcodes
                    match opcode {
                        8 => {
                            println!("Client sent close frame");
                            break;
                        },
                        1 | 2 => { // Text or Binary frame
                            if masked {
                                let mask_start = if payload_len <= 125 { 2 } else { 4 };
                                let mask = &buffer[mask_start..mask_start + 4];
                                let payload_start = mask_start + 4;
                                let payload = &buffer[payload_start..payload_start + payload_len];
                                
                                // Unmask the payload
                                let unmasked: Vec<u8> = payload
                                    .iter()
                                    .enumerate()
                                    .map(|(i, &b)| b ^ mask[i % 4])
                                    .collect();

                                let frame = Self::create_websocket_frame(unmasked);
                                for (_, mut client_stream) in connections.lock().unwrap().iter() {
                                    let _= client_stream.write(&frame);
                                }            
                            }
                        },
                        _ => println!("Unsupported WebSocket opcode: {}", opcode),
                    }
                },
                Err(e) => {
                    println!("Error in websocket");
                    break;
                }
            }
        }
    }

    fn create_websocket_frame(message: Vec<u8>) -> Vec<u8> {
        let payload_len = message.len();

        let mut frame = Vec::new();
        frame.push(0x81);

        if payload_len <= 125 {
            frame.push(payload_len as u8);
        } else if (payload_len <= 65535) {
            frame.push(126);
            frame.extend_from_slice(&(payload_len as u16).to_be_bytes());
        } else {
            frame.push(127);
            frame.extend_from_slice(&(payload_len as u64).to_be_bytes());
        }

        frame.extend_from_slice(&message);

        frame
    }
}

use base64::{engine::general_purpose::STANDARD as base64, Engine}; // Add this

fn main() {
    let mut server = Server::default();

    server.start();
}