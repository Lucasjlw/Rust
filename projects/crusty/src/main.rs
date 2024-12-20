mod server;
use server::Server;

#[tokio::main]
async fn main() {
    let mut server = Server::new("3001");
    server.start().await;
}

