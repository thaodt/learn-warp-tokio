mod actor;
pub mod client;
mod handle;
mod message;
use std::net::SocketAddr;
use warp::Filter;

use crate::client::Client;

#[tokio::main]
async fn main() -> Result<(), tokio::io::Error> {
    let addr: SocketAddr = SocketAddr::new(
        std::net::IpAddr::V4(std::net::Ipv4Addr::new(127, 0, 0, 1)),
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

    let server = warp::serve(hello).run(addr);
    println!("Listening to {}", addr);
    // tokio::spawn(async move {server.await; });

    // lets test with 1 client first, then test with the list of peers in a loop
    // tokio::spawn(async move {
    //     let client = Client::new("127.0.0.1:8002".to_owned()).await;
    //     println!("Connecting to {}", 8002);
    //     let res = client.start().await;
    //     if let Err(e) = res {
    //         println!("Error: {}", e);
    //     }
    // });
    let client = Client::new("127.0.0.1:8002".to_owned()).await;
    println!("Connecting to {}", 8002);
    tokio::spawn(async move {
        let res = client.start().await;
        if let Err(e) = res {
            println!("Error: {}", e);
        }
    });

    server.await;

    Ok(())
}
