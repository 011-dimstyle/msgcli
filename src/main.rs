mod listener;
mod receiver;
mod model;
mod test;
mod file;

use clap::Parser;

#[tokio::main]
async fn main() -> Result<(),Box<dyn std::error::Error>>{
    let mut args = model::Args::parse();                                   

    if args.version{
        println!("v0.0.1");
    }

    if args.receive{
        receiver::receive_to(args.host.clone(), args.port.clone()).await.unwrap();
        args.listen = false;
    }

    if args.listen{
        listener::listening(args.host.clone(),args.port.clone()).await?;
    }

    if !args.file.is_empty() {
        
    }
    
    if !args.copy.is_empty(){
        
    }
    Ok(())

}

