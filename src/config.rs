
use clap::{ArgAction,Arg,Command};
use clap::error::ErrorKind;
use std::error::Error;

#[derive(Debug, Clone)]
pub struct Config {
    pub query: bool,
    pub stream: Option<usize>,
    pub input: String,
    pub output: String,
}

impl Config {
    pub fn build() -> Result<Config, Box<dyn Error>> {
        let mut cmd = Command::new("pcap2mpegts")
        .version("0.1")
        .about("Extracts an mpegts file from a pcap")
        .arg(Arg::new("input")
            .short('i')
            .long("input")
            .help("An input .pcap file")
            .action(ArgAction::Set)
            .value_name("INPUT"))
        .arg(Arg::new("output")
            .short('o')
            .long("output")
            .help("An output mpegts (.ts) file")
            .action(ArgAction::Set)
            .value_name("OUTPUT"))
        .arg(Arg::new("stream")
            .short('s')
            .long("stream")
            .required(false)
            .help("The stream number to dump")
            .action(ArgAction::Set)
            .value_name("STREAM"))
        .arg(Arg::new("query")
            .short('q')
            .long("query")
            .required(false)
            .num_args(0)
            .help("Query available streams only"));
        let args = cmd.get_matches_mut();

        let input = match args.get_one::<String>("input") {
            None => {
                let _ = cmd.print_help();
                return Err(cmd.error(ErrorKind::MissingRequiredArgument, "No input").into());
            }
            Some(s) => s
        };

        let output = match args.get_one::<String>("output") {
            None => {
                let _ = cmd.print_help();
                return Err(cmd.error(ErrorKind::MissingRequiredArgument, "No output").into());
            }
            Some(s) => s
        };

        let stream_idx = args.get_one::<usize>("stream");

        let do_query = args.contains_id("query");
    
        Ok(Config { stream: stream_idx.copied(), query: do_query, input: input.to_string(), output: output.to_string() })
    }
}