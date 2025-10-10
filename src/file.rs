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

pub async fn sendfile(path: String, keep: bool, bind: String) -> io::Result<()> {
	let listener = TcpListener::bind(bind).await?;
	let mut databuffer: Vec<u8> = Vec::new();    
	let mut file = fs::OpenOptions::new()
		.read(true)
		.open(path).await?;
	if keep{
		println!("press Ctrl + C to stop ...");
	}

	loop{
		let (mut stream, client) = listener.accept().await?;
		println!("from {}",client);

		databuffer.clear();
		file.read_to_end(&mut databuffer).await?;
		
		stream.write_all(&databuffer.clone()).await?;
		
		if !keep{break}
	}

	Ok(())
}

