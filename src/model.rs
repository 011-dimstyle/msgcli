use serde::{Deserialize, Serialize};

use clap::{Parser,  ArgAction};

#[derive(Parser)]
#[command(disable_help_flag = true)]
pub struct Args{
    #[arg(short, long, action = ArgAction::SetTrue)]
    pub version: bool,

    #[arg(short, long, default_value_t = true ,action = ArgAction::SetTrue)]
    pub tcp : bool,

     #[arg(short, default_value_t = false ,long, action = ArgAction::SetTrue)]
    pub udp: bool,

    #[arg(short, default_value_t = true,long, action = ArgAction::SetTrue)]
    pub listen: bool,
    
    #[arg(short,long, default_value_t = false ,action = ArgAction::SetTrue)]
    pub receive: bool,

    #[arg(short, long, default_value_t = String::from("0.0.0.0"))]
    pub host: String,

    #[arg(short, long, default_value_t = 2000u32)]
    pub port: u32,

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