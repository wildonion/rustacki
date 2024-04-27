

use crate::*;
use clap::Parser;



#[derive(Parser, Debug)]
#[command(about = "rustacki servers", long_about = None)]
#[command(version)]
pub struct ServerKind {

    /// Type of the server you wish to run
    #[arg(short, long, default_value_t = String::from("http"))]
    pub server: String,

}