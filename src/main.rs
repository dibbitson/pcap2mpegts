

use std::process;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>>
{
    let config = match pcap2mpegts::config::Config::build() {
        Err(_e) => {
            process::exit(1);
        }
        Ok(s) => { s }
    };
    
    if let Err(e) = pcap2mpegts::Converter::run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
 
    Ok(())
}