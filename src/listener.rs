use core::net::SocketAddr;
use tokio::sync::Mutex;
use tokio::{
    io::{AsyncWriteExt, BufReader, AsyncBufReadExt},
    net::{self, TcpListener},
};

async fn readhandler(readermutex: Mutex<net::tcp::OwnedReadHalf>, ipaddrmutex: Mutex<SocketAddr>) {
    let mut readerguard = readermutex.lock().await;
    let mut readerbuffer = BufReader::new(&mut *readerguard);
    let mut output = tokio::io::stdout();
    let mut readerstring = String::new();
    let ipaddrguard = ipaddrmutex.lock().await;

    loop {
        println!("");
        readerstring.clear();
        readerbuffer.read_line(&mut readerstring).await.unwrap();

        output
            .write_all(format!("{} : {}", ipaddrguard, readerstring).as_bytes())
            .await
            .unwrap();
        output.flush().await.unwrap();
    }
}

async fn writehandler(writermutex: Mutex<net::tcp::OwnedWriteHalf>) {
    let mut writerguard = writermutex.lock().await;
    let localaddr = writerguard.local_addr().unwrap();
    let mut output = tokio::io::stdout();
    let input = tokio::io::stdin();
    let readerbuffer = BufReader::new(input);
    let mut lines = readerbuffer.lines();
    loop {
        output.flush().await.unwrap();
        output
            .write_all(format!("{} : ", localaddr).as_bytes())
            .await
            .unwrap();
        output.flush().await.unwrap();
        match lines.next_line().await.unwrap() {
            Some(line) => {
                writerguard
                    .write_all(format!("{}\n", line).as_bytes())
                    .await
                    .unwrap();
            }
            None => {
                println!("");
                break;
            }
        }
    }
}

async fn controlflowhandler(){
    //to handle the stdout and stdin 
}


pub async fn listening(host: String, port: u32) -> Result<(), Box<dyn std::error::Error>> {

    let listen = TcpListener::bind(format!("{}:{}", host, port)).await?;
    println!("listen on {}", listen.local_addr()?);
    
    let (writermutex, readermutex, ipaddrmutex) = match listen.accept().await {
        Ok((stream, sock)) => {
            let (reader, writer) = stream.into_split();
            (Mutex::new(writer), Mutex::new(reader), Mutex::new(sock))
        }
        Err(e) => {
            panic!("{}", e);
        }
    };
    
    let writerfuture = tokio::spawn(writehandler(writermutex));
    tokio::spawn(readhandler(readermutex, ipaddrmutex));

    writerfuture.await?;

    Ok(())

}