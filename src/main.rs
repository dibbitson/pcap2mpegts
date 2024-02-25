use pcap2mpegts::{config::Config, Converter};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    Config::build().and_then(Converter::run)
}
