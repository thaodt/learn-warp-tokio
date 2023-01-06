use tokio::sync::oneshot;

#[derive(Debug)]
pub enum PeerMessage {
    //this could be a struct too!
    Ping {
        message: String,
    },
    Pong,
    Version {
        message: String,
        respond_to: oneshot::Sender<usize>,
    },
}
