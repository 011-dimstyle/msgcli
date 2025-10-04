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
        Commands::listen {bind , copy} => listener::listening(bind, copy).await?,
        Commands::connect { bind , copy} => connector::connect_to(bind, copy).await?,
        Commands::send { bind, message, keep} => listener::sending(bind, message, keep).await?,
        Commands::receive {bind , copy} => connector::receive_to(bind, copy).await?,
        Commands::file { path , keep, bind} => file::sendfile(path, keep, bind).await?
    }

    if !args.file.is_empty() {
        
    }
    
    if !args.copy.is_empty(){
        
    }
    Ok(())

}

