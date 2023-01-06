use std::net::SocketAddr;

use tokio::{
    net::TcpStream,
    sync::{mpsc, oneshot},
};

use super::{actor::run_actor, actor::PeerActor, message::PeerMessage};

#[derive(Debug, Clone)]
pub struct PeerHandle {
    tx: mpsc::Sender<(SocketAddr, PeerMessage)>,
}

impl PeerHandle {
    pub fn new(socket: TcpStream) -> Self {
        let (tx, rx) = mpsc::channel(16);
        let peer_actor = PeerActor::new(rx, socket);

        std::thread::spawn(move || run_actor(peer_actor));

        Self { tx }
    }

    pub async fn send(&mut self, peer_message: (SocketAddr, PeerMessage)) {
        self.tx.send(peer_message).await.unwrap();
    }

    fn _for_tx(tx: mpsc::Sender<(SocketAddr, PeerMessage)>) -> Self {
        Self { tx }
    }

    pub fn ping(&self, addr: SocketAddr) {
        let _ = self.tx.try_send((
            addr,
            PeerMessage::Ping {
                message: "ping".to_string(),
            },
        ));
    }

    pub fn pong(&self, addr: SocketAddr) {
        let _ = self.tx.try_send((addr, PeerMessage::Pong));
    }

    pub async fn send_version(
        &self,
        message: String,
        addr: SocketAddr,
    ) -> Result<usize, tokio::io::Error> {
        let (tx, rx) = oneshot::channel();
        let _ = self.tx.try_send((
            addr,
            PeerMessage::Version {
                message,
                respond_to: tx,
            },
        ));
        Ok(rx.await.unwrap())
    }
}
