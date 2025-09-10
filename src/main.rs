mod listener;
use clap::{ArgAction, Parser};


#[derive(Parser)]
struct Args{
    #[arg(short, long, action = ArgAction::SetTrue)]
    version: bool
}

#[tokio::main]
async fn main(){
    let args = Args::parse();

    if (args.version){
        println!("v0.0.1");
    }
}