// use futures::StreamExt;
use std::net::SocketAddr;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpStream;
use tokio::sync::broadcast;
use warp::Filter;

#[tokio::main]
async fn main() -> Result<(), tokio::io::Error> {
    let (tx, _rx) = broadcast::channel::<(SocketAddr, String)>(100);
    let addr: SocketAddr = SocketAddr::new(
        std::net::IpAddr::V4(std::net::Ipv4Addr::new(0, 0, 0, 0)),
        8001,
    );

    let opt = warp::path::param::<String>()
        .map(Some)
        .or_else(|_| async { Ok::<(Option<String>,), std::convert::Infallible>((None,)) });
    // GET /hello/warp => 200 OK with body "Hello, warp!"
    let hello = warp::path("hello")
        .and(opt)
        .and(warp::path::end())
        .map(|name: Option<String>| {
            format!("Hello, {}!", name.unwrap_or_else(|| "world".to_string()))
        });

    let server = warp::serve(hello).try_bind(addr);
    println!("Listening to {}", addr);
    server.await;

    // Connect to a peer
    let mut stream = TcpStream::connect(addr).await?;

    let tx = tx.clone();
    let mut rx = tx.subscribe();

    tokio::spawn(async move {
        let (reader, mut writer) = stream.split();
        let mut reader = BufReader::new(reader);

        loop {
            let mut buffer = String::new();
            tokio::select! {
                // handle inbound messages
                msg = rx.recv() => {
                    let (other_addr, msg) = msg.unwrap();
                    if other_addr != addr {
                        writer.write_all(format!("{}: {}", other_addr, msg).as_bytes()).await.unwrap();
                    }
                }
                // handle outbound messages,
                result = reader.read_line(&mut buffer) => {
                    if result.is_err() || buffer.trim() == "exit" {
                        println!("Disconnected, {}", addr);
                        break;
                    }
                    tx.send((addr, buffer)).unwrap();
                }
            }
        }
    });
    Ok(())
}
