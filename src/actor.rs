use std::net::SocketAddr;
use tokio::{io::AsyncWriteExt, net::TcpStream, sync::mpsc};

use super::message::PeerMessage;

pub struct PeerActor {
    peer_message_receiver: mpsc::Receiver<(SocketAddr, PeerMessage)>,
    peer_stream: TcpStream,
}

impl PeerActor {
    pub fn new(
        peer_message_receiver: mpsc::Receiver<(SocketAddr, PeerMessage)>,
        peer_stream: TcpStream,
    ) -> Self {
        Self {
            peer_message_receiver,
            peer_stream,
        }
    }

    async fn handle_message(
        &mut self,
        peer_addr: SocketAddr,
        msg: PeerMessage,
    ) -> std::io::Result<()> {
        match msg {
            PeerMessage::Ping { message } => {
                println!("Ping received from {}: {}", peer_addr, message);
                let _ = self.peer_stream.write_all(b"pong");
            }
            PeerMessage::Pong => {
                println!("Pong received from {}", peer_addr);
                let _ = self.peer_stream.write_all(b"ping");
            }
            PeerMessage::Version {
                message,
                respond_to,
            } => {
                // let _ = self.peer_stream.write_all(message.as_bytes());
                // let _ = respond_to.send(message.len());
                let mut buf = Vec::with_capacity(4096);
                let own_addr = self.peer_stream.peer_addr();
                if peer_addr != own_addr? {
                    self.peer_stream.write_all(message.as_bytes()).await?;
                    let resp = self.peer_stream.try_read(&mut buf)?;
                    let _ = respond_to.send(resp);
                }
            }
        }
        Ok(())
    }
}

pub async fn run_actor(mut actor: PeerActor) {
    while let Some(msg) = actor.peer_message_receiver.recv().await {
        let (peer_addr, msg) = msg;
        println!("Message received from {}: {:?}", peer_addr, msg);
        let res = actor.handle_message(peer_addr, msg).await;
        if let Err(e) = res {
            println!("Error: {}", e);
        }
    }
}
