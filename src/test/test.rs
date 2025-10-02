// // use tokio::io::AsyncWriteExt;
// use tokio::io::AsyncWriteExt;


#[tokio::test]
async fn hello(){
    match tokio::fs::OpenOptions::new().create(true).write(true).open("".to_string()).await{
        Err(e) => {eprintln!("error : {}",e)},
        Ok(_) =>{}
    }

    // file.write_all("hello world\nhai".as_bytes()).await.unwrap();
}

    // use rand::prelude::*;
    // #[test]
    // fn test(){
    //     let mut rng = rand::rng();
    //     let x: u32 = rng.random();
    //     println!("{}",x);
    // }

    