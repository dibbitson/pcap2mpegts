

use std::process;
use pcap2mpegts::{config::Config,Converter};

fn run() -> i32
{
    if let Err(e) = Config::build().and_then(Converter::run) {
        println!("{e}");
        return 1;
    }
    0
}

fn main()
{    
    process::exit(run());
}