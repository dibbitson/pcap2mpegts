# pcap2mpegts

pcap2mpegts is a simple CLI tool for extracting MpegTS data from a .pcap file. It only cares about MpegTS delivered over UDP and will ignore everything else.

It exists because traditional methods using WireShark can be quite slow.

One way to generate a .pcap file that is compatible with this tool is:

```
sudo tcpdump -p -i any port <port> -w output.pcap -C 400 -K -n
```

pcap2mpegts has the following options:

```
Usage: pcap2mpegts [OPTIONS]

Options:
  -i, --input <INPUT>    An input .pcap file
  -o, --output <OUTPUT>  An output mpegts (.ts) file
  -a, --all              Dump all streams
  -s, --stream <STREAM>  The stream number to dump
  -q, --query            Query available streams only
  -h, --help             Print help
  -V, --version          Print version
```

Only one of --query, --all, and --stream can be accepted. --output is optional. If not
suppplied it will use the input filename as a base for the generated output files.
You can run --query to only print out all the available streams, and not write out a file.
You can run --stream with an index (gathered from --query) to dump out that stream to OUTPUT.
If not --stream is specified, the tool will dump the first stream found.
Finally, --all with dump all found streams.
