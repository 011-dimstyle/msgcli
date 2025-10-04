use serde::{Deserialize, Serialize};

use clap::{Parser,  ArgAction, Subcommand};


#[allow(non_camel_case_types)]
#[derive(Subcommand)]
pub enum Commands{
    listen {
        #[arg(short, long, default_value_t = 2000u32)]
        port: u32,

        #[arg(short, long, default_value_t = String::default())]
        copy: String
    },

    connect{
        #[arg(long)]
        host: String,

        #[arg(short, long, default_value_t = 2000u32)]
        port: u32,

        #[arg(short, long, default_value_t = String::default())]
        copy: String
    },

    send{
        #[arg(long, default_value_t = String::from("0.0.0.0"))]
        host: String,

        #[arg(short, long, default_value_t = 2000u32)]
        port: u32,

        #[arg(short, long)]
        message: String,

        #[arg(long, default_value_t = false, action = ArgAction::SetTrue)]
        keep: bool
    },

    receive{
        #[arg(long)]
        host: String,

        #[arg(short, long, default_value_t = 2000u32)]
        port: u32,
        
        #[arg(short, long, default_value_t = String::default())]
        copy: String
    },
    
    file{
        #[arg(short,long, default_value_t = 2000u32)]
        port: u32,

        #[arg(long)]
        path: String,

        #[arg(long, default_value_t = false, action = ArgAction::SetTrue)]
        keep: bool
    }


}

#[derive(Parser)]
#[clap(version =  "1.0", author = "dimasalexander27@gmail.com")]
pub struct Args{

    #[command(subcommand)]
    pub subcommand: Commands,

    #[arg(short, default_value_t = true,long, action = ArgAction::SetTrue)]
    pub listen: bool,
    
    #[arg(short,long, default_value_t = false ,action = ArgAction::SetTrue)]
    pub receive: bool,

    #[arg(short, long, default_value_t = String::default())]
    pub message: String,

    #[arg(short, long, default_value_t = String::default())]
    pub file : String,

    #[arg(short,long, default_value_t = String::default())]
    pub copy: String
}  



#[derive(Serialize, Deserialize, Debug)]
pub struct Streamdata{
    pub connect: bool,
    pub msg: String
}

impl Streamdata{
    pub fn new(connect: bool, msg: String) -> Self{
        Streamdata { connect: connect, msg: msg }
    }
}