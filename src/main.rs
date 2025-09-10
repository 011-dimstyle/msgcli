mod listener;
use clap::{ArgAction, Parser};


#[derive(Parser)]
struct Args{
    #[arg(short, long, action = ArgAction::SetTrue)]
    version: bool,
     
     #[arg(short, long, action = ArgAction::SetTrue)]
    tcp: bool,

     #[arg(short, long, action = ArgAction::SetTrue)]
    udp: bool,

    #[arg(short, long, action = ArgAction::SetTrue)]
    listen: bool
}   

#[tokio::main]
async fn main() -> Result<(),Box<dyn std::error::Error>>{
    let args = Args::parse();

    if args.version{
        println!("v0.0.1");
    }

    if args.listen{
        listener::listen_from("0.0.0.0",4000, args.tcp, args.udp).await;
    }

    Ok(())
}