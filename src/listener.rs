use core::net::SocketAddr;
use tokio::sync::Mutex;
use tokio::{
    io::{self, AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::{self, TcpListener},
};

use std::sync::Arc;

use crate::{model};

// read handler

async fn readhandler(
    readermutex: Mutex<net::tcp::OwnedReadHalf>,
    checkconnection: Arc<Mutex<bool>>,
    ipaddrmutex: Mutex<SocketAddr>,
    filecpy: Arc<Option<Mutex<tokio::fs::File>>>,
) -> io::Result<()> {
    let mut readerguard = readermutex.lock().await;
    let localaddr = readerguard.local_addr()?;
    let mut readerbuffer = BufReader::new(&mut *readerguard);
    let mut output = io::stdout();
    let mut readerstring = String::new();
    let ipaddrguard = ipaddrmutex.lock().await;
    loop {
        readerstring.clear();
        readerbuffer.read_line(&mut readerstring).await?;
        let streamdata: model::Streamdata = serde_json::from_str(readerstring.as_str())?;

        //write from client to file copy
        if let Some(filecpymutex) = & *filecpy{
            let mut filecpyguard = filecpymutex.lock().await;
            filecpyguard.write_all(format!("{} : {}",ipaddrguard,streamdata.msg).as_bytes()).await?;
        }

        {
            let mut checkconnguard = checkconnection.lock().await;
            *checkconnguard = streamdata.connect;
            if !*checkconnguard {
                output
                    .write_all("the connection has lost\npress enter to continue ...".as_bytes())
                    .await?;
                output.flush().await?;
                break;
            }
        }
        output
            .write_all(format!("\n{} : {}{} : ", ipaddrguard, streamdata.msg, localaddr).as_bytes())
            .await?;
        output.flush().await?;
    }
    Ok(())
}

//write handler

async fn writehandler(
    writermutex: Mutex<net::tcp::OwnedWriteHalf>,
    checkconnection: Arc<Mutex<bool>>,
    filecpy: Arc<Option<Mutex<tokio::fs::File>>>,
) -> io::Result<()> {
    let mut writerguard = writermutex.lock().await;
    let localaddr = writerguard.local_addr()?;
    let mut output = io::stdout();
    let input = io::stdin();
    let readerbuffer = BufReader::new(input);
    let mut lines = readerbuffer.lines();
    loop {

        // print and input  
        output
            .write_all(format!("{} : ", localaddr).as_bytes())
            .await?;
        output.flush().await?;
        match lines.next_line().await? {
            Some(line) => {
                let msg = format!("{}\n", line);

                // from rust struct to json string
                let mut streamdata = serde_json::to_string(&model::Streamdata::new(true, msg.clone()))?;
                streamdata.push('\n');
                writerguard.write_all(streamdata.as_bytes()).await?;
                
                // write from local to file copy
                if let Some(filecpymutex) = & *filecpy{
                    let mut filecpyguard = filecpymutex.lock().await;
                    filecpyguard.write_all(format!("{} : {}",localaddr,msg.clone()).as_bytes()).await?;
                }
                
            }
            None => {
                let mut streamdata =
                    serde_json::to_string(&model::Streamdata::new(false, String::default()))?;
                {
                    let mut checkconnguard = checkconnection.lock().await;
                    *checkconnguard = false;
                }
                streamdata.push('\n');
                writerguard.write_all(streamdata.as_bytes()).await?;
                println!("");
                break;
            }
        }
    }
    Ok(())
}

// connection checker

async fn controlflowhandler(checkconnection: Arc<Mutex<bool>>) {
    'control: loop {
        {
            let checkconnguard = checkconnection.lock().await;
            if !*checkconnguard {
                break 'control;
            };
        }
    }
}

// streaming mode

pub async fn listening(host: String, port: u32, copy: String) -> io::Result<()> {
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

    let mut filecpy: Arc<Option<Mutex<tokio::fs::File>>> = Arc::new(None);
    if !copy.is_empty() {
        filecpy = Arc::new(Some(Mutex::new(
            tokio::fs::OpenOptions::new()
                .create(true)
                .append(true)
                .write(true)
                .open(copy)
                .await?,
        )))
    }

    let write_filecpy = filecpy.clone();
    let read_filecpy = filecpy.clone();

    let checkconnection = Arc::new(Mutex::new(true));
    let checkconn_read = checkconnection.clone();
    let checkconn_write = checkconnection.clone();
    let checkconn_flow = checkconnection.clone();

    tokio::spawn(writehandler(writermutex, checkconn_write, write_filecpy));
    tokio::spawn(readhandler(
        readermutex,
        checkconn_read,
        ipaddrmutex,
        read_filecpy,
    ));

    controlflowhandler(checkconn_flow).await;

    Ok(())
}


// none streaming

pub async fn sending(host: String, port: u32, message: String, keep: bool) -> io::Result<()> {
    let ipaddr = format!("{}:{}", host, port);
    let listener = TcpListener::bind(ipaddr.clone()).await?;
    if keep {
        println!("listening on {}", ipaddr.clone());
    }
    loop {
        let (mut stream, client) = listener.accept().await?;
        if keep {
            println!("{}", client);
        }
        stream.write_all(message.as_bytes()).await?;
        if !keep {
            break;
        };
    }

    Ok(())
}
