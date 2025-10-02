// use tokio::io::{self, AsyncReadExt,AsyncWriteExt};

// pub async fn openfile(path: String) -> io::Result<()>{
//     let mut file = tokio::fs::OpenOptions::new()
//         .read(true)
//         .open(path).await?;
//     let mut stringbuffer = String::new();
    
//     file.read_to_string(&mut stringbuffer).await?;

//     println!("{}",stringbuffer);
//     Ok(())
// }

// pub async fn writetofile(path: String, data: &[u8]) -> io::Result<()>{
//     let mut file = tokio::fs::OpenOptions::new()
//         .create(true)
//         .write(true)
//         .append(true)
//         .open(path).await?;

//     file.write_all(data).await?;
//     Ok(())
// }
