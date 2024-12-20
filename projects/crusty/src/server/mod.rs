// use std::collections::HashMap;
use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::{TcpListener, TcpStream}};

pub struct Server {
    listener: Option<TcpListener>,
    connections: Vec<TcpStream>,
    address: String,
    port: String
}

impl Server {
    pub fn new(port: &str) -> Self {
        Server {
            connections: vec![],
            address: String::from("0.0.0.0"),
            port: String::from(port),
            listener: None
        }
    }

    pub async fn start(&mut self) {
        let full_address = format!("{}:{}", self.address, self.port);

        let listener = TcpListener::bind(&full_address).await.unwrap();
        println!("Server started at {}", full_address);

        loop {
            if let Ok((socket, _)) = listener.accept().await {
                self.handle_connection(socket).await;
            }
        }
    }

    async fn handle_connection(&mut self, mut connection: TcpStream) {
            let mut data = [0u8; 1024];
            let _ = connection.read(&mut data).await;

            println!("{:?}", data);

            let response = "HTTP/1.1 200 OK\r\n\r\nkkkk";

            println!("Response: {}", response);

            let _ = connection.write(response.as_bytes()).await;

            // self.connections.push(connection);
    }
}

