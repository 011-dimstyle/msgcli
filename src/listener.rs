use std::fmt::format;

use tokio::net::{TcpListener};

pub async fn listen_from(
    ip: &str, 
    port: i32, 
    listenOnTcp: bool, 
    listenOnUdp: bool
)-> Result<(), Box<dyn std::error::Error>>{

    let listener = TcpListener::bind(format!("{}:{}",ip, port)).await?;
    println!("listening on {}", listener.local_addr()?.ip());

    Ok(())

}