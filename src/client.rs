use std::net::SocketAddr;

use tokio::net::TcpStream;

use crate::handle;

pub struct Client {
    addr: String,
}

impl Client {
    /// Create a new client.
    pub async fn new(addr: String) -> Self {
        Self { addr }
    }

    /// Start the client, connect to the server and send some messages.
    pub async fn start(&self) -> Result<(), tokio::io::Error> {
        let addr: SocketAddr = self.addr.parse().unwrap();
        println!("start to {}", addr);
        if let Ok(stream) = TcpStream::connect(addr).await {
            println!("connected to {}", addr);
            let peer_handle = handle::PeerHandle::new(stream);
            println!("peer_handle created");
            peer_handle.ping(addr);
            println!("ping sent");
            // peer_handle.pong("127.0.0.1:8002".parse().unwrap());
            // let _ = peer_handle.send_version("Hello".to_string(), "127.0.0.1:8002".parse().unwrap()).await;
        } else {
            println!("failed to connect to {}", addr);
        }
        Ok(())
    }
}
