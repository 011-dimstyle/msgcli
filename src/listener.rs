use std::fmt::format;

use tokio::net::{TcpListener};

use crate::listener;

pub async fn listen_from(ip: &str, port: i32, listenOnTcp: bool, listenOnUdp: bool){

   let listener = TcpListener::bind(format!("{}:{}",ip, port)).await.unwrap();
    println!("listening on {}", listener.local_addr().unwrap().ip());


}