use core::net::SocketAddr;
use tokio::sync::Mutex;
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::{self, TcpListener},
};

use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};

use crate::model;

async fn readhandler(
    readermutex: Mutex<net::tcp::OwnedReadHalf>,
    checkconnection: Arc<AtomicBool>,
    ipaddrmutex: Mutex<SocketAddr>,
) -> tokio::io::Result<()> {
    let mut readerguard = readermutex.lock().await;
    let localaddr = readerguard.local_addr()?;
    let mut readerbuffer = BufReader::new(&mut *readerguard);
    let mut output = tokio::io::stdout();
    let mut readerstring = String::new();
    let ipaddrguard = ipaddrmutex.lock().await;

    loop {
        readerstring.clear();
        readerbuffer.read_line(&mut readerstring).await?;
        let streamdata: model::Streamdata = serde_json::from_str(readerstring.as_str())?;
        checkconnection.store(streamdata.connect, Ordering::SeqCst);
        output
            .write_all(format!("\n{} : {}{} : ", ipaddrguard, streamdata.msg, localaddr).as_bytes())
            .await?;
        output.flush().await?;
    }
}

async fn writehandler(writermutex: Mutex<net::tcp::OwnedWriteHalf>, checkconnection: Arc<AtomicBool>) -> tokio::io::Result<()> {
    let mut writerguard = writermutex.lock().await;
    let localaddr = writerguard.local_addr()?;
    let mut output = tokio::io::stdout();
    let input = tokio::io::stdin();
    let readerbuffer = BufReader::new(input);
    let mut lines = readerbuffer.lines();
    loop {
        output
            .write_all(format!("{} : ", localaddr).as_bytes())
            .await?;
        output.flush().await?;
        match lines.next_line().await? {
            Some(line) => {
                let msg = format!("{}\n", line);
                let mut streamdata = serde_json::to_string(&model::Streamdata::new(true, msg))?;
                streamdata.push('\n');
                writerguard.write_all(streamdata.as_bytes()).await?;
            }
            None => {
                let mut streamdata = serde_json::to_string(&model::Streamdata::new(false,String::default()))?;
                checkconnection.store(false, Ordering::SeqCst);
                streamdata.push('\n');
                writerguard.write_all(streamdata.as_bytes()).await?;
                break;
            }
        }
    }
    Ok(())
}

async fn controlflowhandler(checkconnection: Arc<AtomicBool>) {
    loop {
        if !checkconnection.load(Ordering::SeqCst) {
            break;
        };
    }
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

    let checkconnection = Arc::new(AtomicBool::new(true));
    let checkconn_read = checkconnection.clone();
    let checkconn_write = checkconnection.clone();
    let checkconn_flow = checkconnection.clone();

    tokio::spawn(writehandler(writermutex, checkconn_write));
    tokio::spawn(readhandler(readermutex, checkconn_read, ipaddrmutex));

    controlflowhandler(checkconn_flow).await;

    Ok(())
}
