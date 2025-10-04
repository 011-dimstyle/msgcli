use tokio::{
    fs,
    io::{self,AsyncWriteExt,AsyncReadExt},
    sync::Mutex,
    net::TcpListener
};


pub async fn write_to_file_stream(filecpy: &Option<Mutex<fs::File>>, ipaddr: String, msg: &String) -> io::Result<()>{
    if let Some(filecpymutex) = filecpy{
        let mut filecpyguard = filecpymutex.lock().await;
        filecpyguard.write_all(format!("{} : {}",ipaddr,msg).as_bytes()).await?;
    }
    Ok(())
}

pub async fn sendfile(path: String, keep: bool, port: u32) -> io::Result<()> {
    let listener = TcpListener::bind(format!("0.0.0.0:{}",port)).await?;
    let mut stringbuffer = String::new();    
    let mut file = fs::OpenOptions::new()
        .read(true)
        .open(path).await?;
    if keep{
        println!("press Ctrl + C to stop ...");
    }

    loop{
        let (mut stream, client) = listener.accept().await?;
        println!("from {}",client);

        stringbuffer.clear();
        file.read_to_string(&mut stringbuffer).await?;
        
        stream.write_all(stringbuffer.clone().as_bytes()).await?;
        
        if !keep{break}
    }

    Ok(())
}

