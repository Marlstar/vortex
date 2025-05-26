use std::{io::{Read, Write}, net::{TcpStream, ToSocketAddrs}, path::{Path, PathBuf}};
use crate::{args::ReceiveArgs, Error, ARGS, CWD};
use super::packet::{Content, Packet, MAX_CHUNK_SIZE};
use byteorder::{ReadBytesExt, WriteBytesExt, BigEndian};

const BUF_CAPACITY: usize = 1024;

pub struct Client {
    socket: TcpStream,
    output_path: Option<PathBuf>,
}
impl Client {
    pub fn new(args: &ReceiveArgs) -> Result<Self, Error> {
        return Ok(Self {
            socket: connect((args.server_addr, ARGS.port))?,
            output_path: args.output_path.clone(),
        })
    }

    pub fn main(&mut self) {
        let header;
        let write_packet_idx = |i: usize, c: Content, buf: &mut Vec<u8>, total_size: usize| {
            let start = c.index * MAX_CHUNK_SIZE;
            let end = usize::min((c.index + 1) * MAX_CHUNK_SIZE, total_size);
            buf[start..end].copy_from_slice(&c.bytes);
        };

        // let bytes = self.recv().unwrap();
        if let Packet::Header(h) = self.recv_header().unwrap() {
            header = h;

            log::info!("Receiving file \"{}\"", header.filename);

            if self.output_path.is_none() {
                self.output_path = Some(CWD.join(header.filename));
            }
        } else {
            log::error!("No header packet sent, aborting");
            return;
        }

        let mut buf = vec![0u8; header.total_size];
        let mut received = vec![false; header.chunk_count];
        for _ in 0..header.chunk_count {
            let chunk = self.recv_body_chunk().unwrap();
            if let Packet::Content(c) = chunk {
                received[c.index] = true;
                write_packet_idx(c.index, c, &mut buf, header.total_size);
            } else {
                log::error!("Non-content packet received, aborting");
                return;
            }
        }
        loop {
            let not_received = received.iter().enumerate().filter_map(|(i,a)| if !a { Some(i) } else { None }).collect::<Vec<usize>>();
            if not_received.is_empty() { break; }
            log::warn!("Missing packets: {not_received:?}");

            for i in not_received {
                let packet = self.request_packet(i).unwrap();
                if let Packet::Content(c) = packet {
                    write_packet_idx(i, c, &mut buf, header.total_size);
                    received[i] = true;
                }
            }
        }
        self.socket.write_u32::<BigEndian>(super::code::FINISHED).unwrap();
        let mut file = std::fs::File::create_new(self.output_path.as_ref().unwrap()).unwrap();
        file.write_all(&buf).unwrap();
    }

    fn request_packet(&mut self, idx: usize) -> Result<Packet, Error> {
        self.socket.write_u32::<BigEndian>(super::code::REQUEST_PACKET)?;
        self.socket.write_u32::<BigEndian>(idx as u32)?;
        std::thread::sleep(std::time::Duration::from_secs(3));
        return self.recv_body_chunk();
    }

    fn recv_header(&mut self) -> Result<Packet, Error> {
        let bytes = self.recv()?;
        let packet = rmp_serde::from_slice(&bytes).unwrap();
        return Ok(packet);
    }

    // TODO: return a packet
    fn recv_body_chunk(&mut self) -> Result<Packet, Error> {
        let bytes = self.recv()?;
        let packet = rmp_serde::from_slice(&bytes).unwrap();
        return Ok(packet);
    }

    fn recv(&mut self) -> Result<Vec<u8>, Error> {
        let len = self.socket.read_u32::<BigEndian>().unwrap();
        log::debug!("Receiving {len} bytes");
        let mut buf = vec![0u8; len as usize];
        self.socket.read_exact(&mut buf)?;
        Ok(buf)
    }
}

fn connect(addr: impl ToSocketAddrs) -> Result<TcpStream, Error> {
    Ok(TcpStream::connect(addr)?)
}
