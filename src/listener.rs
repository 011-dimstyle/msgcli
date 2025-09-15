use std::fmt::format;

use tokio::{io::{AsyncWriteExt}, net::{TcpListener}};

pub async fn listen_from(
    ip: &str, 
    port: i32, 
    listenOnTcp: bool, 
    listenOnUdp: bool,
    message: String
)-> Result<(), Box<dyn std::error::Error>>{

    let listener = TcpListener::bind(format!("{}:{}",ip, port)).await?;
    println!("listening on {}", listener.local_addr()?.ip());
    let (mut stream, socks) = listener.accept().await?;
    println!("accept socket from : {}",socks);
    if message.len() > 0{ 
        stream.write_all(message.as_bytes()).await?;
        tokio::time::sleep(std::time::Duration::from_secs(5)).await;
    }
    Ok(())

}