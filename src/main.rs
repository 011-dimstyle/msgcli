mod listener;
mod connector;
mod model;
mod test;
mod file;

use clap::Parser;
use model::Commands;

#[tokio::main]
async fn main() -> Result<(),Box<dyn std::error::Error>>{
    let args = model::Args::parse();                                   

    match args.subcommand {
        Commands::listen { host, port } => listener::listening(host, port).await?,
        Commands::connect { host, port } => connector::connect_to(host, port).await?,
        Commands::send { host, port, message, keep} => listener::sending(host, port, message, keep).await?,
        Commands::receive { host, port } => connector::receive_to(host, port).await?
    }

    if !args.file.is_empty() {
        
    }
    
    if !args.copy.is_empty(){
        
    }
    Ok(())

}

