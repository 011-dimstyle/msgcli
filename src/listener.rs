use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::{
    io::{AsyncWriteExt, BufReader, AsyncBufReadExt},
    net::{TcpListener},
};

pub async fn listening(host: String, port: u32) -> Result<(), Box<dyn std::error::Error>> {

    let listen = TcpListener::bind(format!("{}:{}", host, port)).await?;
    println!("listen on {}", listen.local_addr()?);
    
    let (streammutex, ipaddr) = match listen.accept().await{
        Ok((stream,sock)) => (Arc::new(Mutex::new(stream)), Arc::new(Mutex::new(sock))),
        Err(e) => {panic!("{}", e);}
    };
    
    let streamread = Arc::clone(&streammutex);
    let streamwrite = Arc::clone(&streammutex);
    let ipaddrmutex = Arc::clone(&ipaddr);
    
    let writerhandler = tokio::spawn(async move {

        let mut streamguard = streamwrite.lock().await;
        let input = tokio::io::stdin();
        let mut output = tokio::io::stdout();
        let readerbuffer = BufReader::new(input);
        let mut lines = readerbuffer.lines();
        loop{
            output.write_all(format!("{} : ", streamguard.local_addr().unwrap()).as_bytes()).await.unwrap();
            output.flush().await.unwrap();
            match lines.next_line().await.unwrap() {
                Some(mut line) => {
                    line.push('\n');
                    streamguard.write_all(line.as_bytes()).await.unwrap();
                },
                None => {
                    println!("");
                    break;
                }
            }
        }   
                                                                    
    });
        
    tokio::spawn(async move {
        println!("hello");
        let mut streamguard = streamread.lock().await;
        let mut readstring = String::new();
        let mut readerbuffer = BufReader::new(&mut *streamguard);
        let mut output = tokio::io::stdout();
        let ipaddrguard = ipaddrmutex.lock().await;
        loop{
            // streamguard.read_to_string(&mut readbuffer).await.unwrap();
            readerbuffer.read_line(&mut readstring).await.unwrap();
            output.write_all(format!("{} : {} ", ipaddrguard ,readstring).as_bytes()).await.unwrap();
            output.flush().await.unwrap();
        }
    });

    writerhandler.await?;

    Ok(())

}