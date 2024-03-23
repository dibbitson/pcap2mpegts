# pcap2mpegts

`pcap2mpegts` is a simple CLI tool for extracting `MpegTS` data from a `.pcap`
file. It only cares about `MpegTS` delivered over UDP and will ignore everything
else.

It exists because traditional methods using WireShark can be quite slow.

One way to generate a `.pcap` file that is compatible with this tool is:

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

Only one of `--query`, `--all`, and `--stream` can be accepted. `--output` is
optional. If not supplied it will use the input filename as a base for the
generated output files. You can run `--query` to only print out all the
available streams, and not write out a file. You can run `--stream` with an
index (gathered from `--query`) to dump out that stream to `OUTPUT`. If not
`--stream` is specified, the tool will dump the first stream found. Finally,
`--all` with dump all found streams.

## Examples

To query all the available streams in a file:

```
$ ./pcap2mpegts -i ~/input.pcap --query
Legacy header: LINUX_SLL2
Streams found:
	Stream 0: 10.42.69.201.37360 -> 10.42.69.200.15004 packets: 154471
	Stream 1: 169.254.87.45.43675 -> 169.254.87.100.15004 packets: 111603
	Stream 2: 169.254.87.45.40178 -> 169.254.87.100.15004 packets: 5196
	Stream 3: 169.254.87.45.40909 -> 169.254.87.100.15004 packets: 37673
```

To extract a specific stream:

```
$ ./pcap2mpegts -i ~/input.pcap --stream 1

...

$ ls *.ts
input_s1.ts
```
To extract all streams:
```
$ ./pcap2mpegts -i ~/input.pcap --all

...

$ ls *.ts
input_s0.ts
input_s1.ts
input_s2.ts
input_s3.ts
```

## Things to note

The data is expected to be well formed in the sense that UDP packets should not be fragmented. Typically this is not an issue. UDP packet sizes < 1500 bytes with 1 or more 188 byte MpegTS packet each starting with 0x47 is fairly normal. UDP packet sizes > 1500
can be fragmented.