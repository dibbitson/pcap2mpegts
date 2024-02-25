
use clap::{ArgAction,Arg,ArgGroup,Command};
use clap::error::ErrorKind;
use std::error::Error;

#[derive(Debug, Clone)]
pub struct Config {
    // Query the pcap only and list detected streams. No file writing
    pub query: bool,
    // Dump all detected streams to individual files. Ignores the
    // stream and output flags.
    pub all: bool,
    // an optional flag to just dump one stream (by index gathered from query). 
    // This requires the output flag specified a filename. If this is
    // not specified, it defaults to the first stream detected.
    pub stream: Option<usize>,
    // An input .pcap file to parse, required
    pub input: String,
    // An optional filename to use for a custom output filename. If not
    // specified it will default to the .pcap filename with a .ts extension.
    // Ignored if all or query are specified.
    pub output: Option<String>,
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
        .arg(Arg::new("all")
            .short('a')
            .long("all")
            .required(false)
            .num_args(0)
            .value_parser(clap::value_parser!(bool))
            .help("Dump all streams")
            .value_name("ALL"))
        .arg(Arg::new("stream")
            .short('s')
            .long("stream")
            .required(false)
            .value_parser(clap::value_parser!(usize))
            .help("The stream number to dump")
            .action(ArgAction::Set)
            .value_name("STREAM"))
        .arg(Arg::new("query")
            .short('q')
            .long("query")
            .required(false)
            .num_args(0)
            .value_parser(clap::value_parser!(bool))
            .help("Query available streams only"))
        .group(ArgGroup::new("out_opts")
            .args(["all", "stream", "query"])
        );
        let args = cmd.get_matches_mut();

        let input = match args.get_one::<String>("input") {
            None => {
                return Err(cmd.error(ErrorKind::MissingRequiredArgument, "No input provided.").into());
            }
            Some(s) => s
        };

        let output = args.get_one::<String>("output");
        let mut stream_idx = args.get_one::<usize>("stream");
        if stream_idx.is_none() {
            stream_idx = Some(&0);
        }
        let do_all = args.get_flag("all");
        let do_query = args.get_flag("query");
    
        Ok(Config { 
            stream: stream_idx.copied(), 
            all: do_all, 
            query: do_query, 
            input: input.to_string(), 
            output: output.cloned()
        })
    }
}