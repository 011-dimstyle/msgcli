mod listener;
mod receiver;
mod model;
mod test;

use clap::Parser;

#[tokio::main]
async fn main() -> Result<(),Box<dyn std::error::Error>>{
    let args = model::Args::parse();                                   

    if args.version{
        println!("v0.0.1");
    }

    if args.receive{
        receiver::receive_to(args.host.clone(), args.port.clone()).await?;
    }

    if args.listen{
        listener::listening(args.host.clone(),args.port.clone()).await?;
    }
    
    Ok(())

}

