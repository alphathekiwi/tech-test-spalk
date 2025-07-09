#![allow(unused_mut)]
use anyhow::anyhow;
use clap::Parser;
use std::io::{self, IsTerminal};
use std::io::{Cursor, Read};

#[derive(Parser)]
#[command(name = "spalk-tech-test")]
#[command(about = "A simple program that can validate MPEG-TS packets")]
struct Args {
    /// Enables verbose logging
    #[arg(short, long, default_value = "false")]
    verbose: bool,
}

#[derive(Debug)]
struct SyncIdPacket {
    sync: u8,
    packet_id: u16,
}

impl SyncIdPacket {
    fn new(bytes: &[u8]) -> Self {
        SyncIdPacket {
            sync: bytes[0],
            packet_id: u16::from_be_bytes([bytes[1] & 0x1F, bytes[2]]),
        } // 0x1F is used to mask the flags
    }

    fn is_valid(&self) -> bool {
        self.sync == 0x47
    }
}

#[cfg(test)]
mod tests;

fn parse_packets(mut reader: Cursor<Vec<u8>>, verbose: bool) -> anyhow::Result<Vec<String>> {
    let mut buffer = vec![0u8; 188];
    let mut total_bytes = 0;
    let mut packet_num = 0;
    let mut output = Vec::new();

    loop {
        match reader.read(&mut buffer) {
            // all packets should fill the 188 buffer
            // last packet will be discarded if it's shorter than expected
            // this can occur for a number of reasons primarily incomplete data
            Ok(br) if br < 188 => break,
            Ok(bytes_read) => {
                let chunk_data = &buffer[..bytes_read];
                let control = SyncIdPacket::new(&chunk_data[0..3]);
                if control.is_valid() {
                    let output_line = match verbose {
                        true => format!("{packet_num}: 0x{:x}", control.packet_id),
                        false => format!("0x{:x}", control.packet_id),
                    };
                    output.push(output_line);
                    packet_num += 1;
                } else if packet_num == 0 {
                    if verbose {
                        println!("Attempting to correct for first packet being incomplete");
                    }
                    // Packet is not valid and it's the first packet
                    // This implementation is rudimentary and I recognize that it would not work for all cases
                    // It has been left as is due to time restrictions
                    match chunk_data.iter().enumerate().find(|(_, v)| **v == 0x47) {
                        Some((packet_start, _sync)) => {
                            // Adding reader position is not necessary here except for debugging purposes
                            let packet_start = reader.position() - 188 + packet_start as u64;
                            reader.set_position(packet_start);
                        }
                        None => {
                            return Err(anyhow!(
                                "No sync byte present in packet {packet_num}, offset {total_bytes}"
                            ));
                        }
                    }
                } else {
                    return Err(anyhow!(
                        "No sync byte present in packet {packet_num}, offset {total_bytes}"
                    ));
                }
                total_bytes += bytes_read;
            }
            Err(e) => return Err(e.into()),
        }
    }

    Ok(output)
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let mut stdin = io::stdin();
    if stdin.is_terminal() {
        println!("No file was passed as input, try the following command:");
        if cfg!(target_os = "windows") {
            println!("type test_failure.ts | target/debug/spalk-tech-test.exe\n")
        } else {
            println!("cat test_failure.ts | ./target/debug/spalk-tech-test\n")
        }
        return Err(anyhow!("Terminated with error"));
    }
    let mut all_data = Vec::new();
    stdin.read_to_end(&mut all_data)?;

    let mut cursor = std::io::Cursor::new(all_data);
    // Uncomment the following line to simulate an incomplete first packet
    // cursor.set_position(5);
    let output = parse_packets(cursor, args.verbose)?;
    println!("{}", output.join("\n"));
    Ok(())
}
