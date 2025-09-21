mod listener;
mod test;

use std::{fs::read, io::Read};

use clap::{ArgAction, Parser, command};

#[derive(Parser)]
#[command(disable_help_flag = true)]
struct Args{
    #[arg(short, long, action = ArgAction::SetTrue)]
    version: bool,
     
     #[arg(short, long, action = ArgAction::SetTrue)]
    tcp: bool,

     #[arg(short, long, action = ArgAction::SetTrue)]
    udp: bool,

    #[arg(short, long, action = ArgAction::SetTrue)]
    listen: bool,

    #[arg(short, long, default_value_t = String::default())]
    host: String,

    #[arg(short, long, default_value_t = 0i32)]
    port: i32,

    #[arg(short, long, default_value_t = String::default())]
    message: String
}   

#[tokio::main]
async fn main() -> Result<(),Box<dyn std::error::Error>>{
    let args = Args::parse();                                   

    if args.version{
        println!("v0.0.1");
    }

    if args.listen{
        listener::listening(args.host,args.port).await?;
    }

    Ok(())

}

