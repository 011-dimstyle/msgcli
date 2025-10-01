use std::{net::TcpListener, sync::Arc};

use tokio::{
    io::{self, AsyncBufReadExt, BufReader, AsyncWriteExt, AsyncReadExt},
    net::{
        TcpStream,
        tcp::{OwnedReadHalf, OwnedWriteHalf},
    },
    sync::Mutex
};

use crate::model;

async fn readhandler(readmutex: Mutex<OwnedReadHalf>, checkconnection: Arc<Mutex<bool>> ,ipaddr: String) -> io::Result<()> {
    let mut readguard = readmutex.lock().await;
    let localaddr = readguard.local_addr()?;
    let mut reader = BufReader::new(&mut *readguard);
    let mut readerstringbuffer = String::new();
    let mut output = io::stdout();
  
    loop {
        readerstringbuffer.clear();
        reader.read_line(&mut readerstringbuffer).await?;
        let streamdata: model::Streamdata = serde_json::from_str(readerstringbuffer.as_str())?;
        {
            let mut checkconnguard = checkconnection.lock().await;
            *checkconnguard = streamdata.connect;
            if !*checkconnguard{
                output.write_all("the connection has lost\npress enter to continue ...".as_bytes()).await?;
                output.flush().await?;
                break
            };
        }
        output.write_all(format!("\n{} : {}{} : ", ipaddr, streamdata.msg, localaddr).as_bytes()).await?;
        output.flush().await?;
    }
    Ok(())
}

async fn writehandler(writemutex: Mutex<OwnedWriteHalf>, checkconnection : Arc<Mutex<bool>>) -> io::Result<()> {
    let mut writeguard = writemutex.lock().await;   
    let localaddr = writeguard.local_addr()?;
    let mut input = io::stdin();
    let mut output = io::stdout();
    let inputbuffer = BufReader::new(&mut input);
    let mut lines = inputbuffer.lines();

    loop {
        output.write_all(format!("{} : ", localaddr).as_bytes()).await?;
        output.flush().await?;

        match lines.next_line().await?{
            Some(line) => {
                let msg = format!("{}\n",line);
                let mut streamdata = serde_json::to_string(&model::Streamdata::new(true, msg))?;
                streamdata.push('\n');
                writeguard.write_all(streamdata.as_bytes()).await?;
            },
            None => {
                let mut streamdata = serde_json::to_string(&model::Streamdata::new(false, String::default()))?;
                {
                    let mut checkconnguard = checkconnection.lock().await;
                    *checkconnguard = false;
                }
                streamdata.push('\n');
                writeguard.write_all(streamdata.as_bytes()).await?;
                println!("");
                break;
            }
        } 
    }
    Ok(())
}

async fn flowhandler(checkconnection : Arc<Mutex<bool>>){
    'control : loop{
        {
            let checkconnguard = checkconnection.lock().await;
            if !*checkconnguard{break 'control}
        }
    }
}

pub async fn connect_to(host: String, port: u32) -> io::Result<()> {
    let ipaddr = format!("{}:{}", host, port);
    let (readmutex, writemutex) = match TcpStream::connect(ipaddr.clone()).await {
        Ok(stream) => {
            let (read, write) = stream.into_split();
            (Mutex::new(read), Mutex::new(write))
        }
        Err(e) => panic!("{}", e),
    };

    let check_connection = Arc::new(Mutex::new(true));
    let checkconn_reader = check_connection.clone();
    let checkconn_writer = check_connection.clone();
    let checkconn_flow = check_connection.clone();

    tokio::spawn(readhandler(readmutex, checkconn_reader ,ipaddr.clone()));
    tokio::spawn(writehandler(writemutex, checkconn_writer));
    flowhandler(checkconn_flow).await;

    Ok(())
}

pub async fn receive_to(host: String, port: u32) -> io::Result<()>{
    let mut stream = TcpStream::connect(format!("{}:{}",host,port)).await?;
    let mut stringbuffer =  String::new();
    stream.read_to_string(&mut stringbuffer).await?;
    println!("{}",stringbuffer);
    Ok(())
}
