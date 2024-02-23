
use std::{fmt, io::BufWriter};
use std::fs::File;
use std::io::{Error, Write};
use std::collections::HashMap;

#[derive(Debug, Copy, Clone)]
pub struct UdpStream {
    pub src_addr: [u8; 4],
    pub src_port: u16,
    pub dst_addr: [u8; 4],
    pub dst_port: u16,
    pub index: usize,
    packet_count: usize
}

impl PartialEq for UdpStream {
    fn eq(&self, other: &Self) -> bool {
        (self.src_addr == other.src_addr) &&
        (self.src_port == other.src_port) &&
        (self.dst_addr == other.dst_addr) &&
        (self.dst_port == other.dst_port)
    }
}

impl fmt::Display for UdpStream {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}.{}.{}.{} -> {}.{}.{}.{}.{} packets: {}", 
            self.src_addr[0], self.src_addr[1], self.src_addr[2], self.src_addr[3], self.src_port,
            self.dst_addr[0], self.dst_addr[1], self.dst_addr[2], self.dst_addr[3], self.dst_port,
            self.packet_count)
    }
}

impl UdpStream {
    pub fn new(src: [u8; 4], sport : u16, dst: [u8; 4], dport : u16) -> UdpStream {
        UdpStream {
            src_addr: src,
            src_port: sport,
            dst_addr: dst,
            dst_port: dport,
            index: 0,
            packet_count: 1
        }
    }
}

#[derive(Debug)]
pub struct StreamTracker {
    streams: Vec<UdpStream>,
    ts_base_filename: String,
    ts_files: HashMap<usize,BufWriter<File>>
}

impl StreamTracker {
    pub fn new(filename: String) -> StreamTracker {
        StreamTracker {
            streams: vec!(),
            ts_base_filename: filename,
            ts_files: HashMap::new()
        }
    }

    pub fn dump_payload(&mut self, index: usize, data: &[u8]) -> Result<(), Error> {

        // Make sure BufWriter exists first for this index
        if !self.ts_files.contains_key(&index) {
            let filename = format!("{}_s{}{}", self.ts_base_filename, index, ".ts");
            let outbuf = BufWriter::new(File::create(filename)?);
            self.ts_files.insert(index,outbuf);

        }

        let writer = self.ts_files.get_mut(&index);
        let _ = match writer {
            Some(buf) => buf.write(data)?,
            None =>  {
                println!("No writer available!");
                return Ok(());
            }
        };
        Ok(())
    }

    pub fn track(&mut self, stream: UdpStream) -> usize {
        let pos = self.streams.iter().position(|&s| s == stream);
        match pos {
            Some(idx) => { 
                self.streams[idx].packet_count += 1;
                return idx;
            },
            None => { 
                self.streams.push(stream);
                return self.streams.len()-1
            }
        }
    }
}

impl fmt::Display for StreamTracker {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (n, s) in self.streams.iter().enumerate() {
            writeln!(f, "\tStream {}: {}", n, s)?
        }
        Ok(())
    }
}
