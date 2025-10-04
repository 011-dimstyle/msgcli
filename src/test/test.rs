// // use tokio::io::AsyncWriteExt;
// use tokio::io::AsyncWriteExt;


#[tokio::test]
async fn hello(){
    match tokio::fs::OpenOptions::new().create(true).write(true).open("".to_string()).await{
        Err(e) => {eprintln!("error : {}",e)},
        Ok(_) =>{}
    }
    let x: Vec<_> = "hello:hai".split(":").collect();
    // file.write_all("hello world\nhai".as_bytes()).await.unwrap();
    println!("{:?}",x[0]);
}

    // use rand::prelude::*;
    // #[test]
    // fn test(){
    //     let mut rng = rand::rng();
    //     let x: u32 = rng.random();
    //     println!("{}",x);
    // }

    