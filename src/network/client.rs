use std::{io::{Read, Write}, net::{TcpStream, ToSocketAddrs}, path::{Path, PathBuf}};
use crate::{args::ReceiveArgs, Error, ARGS};
use super::packet::{Packet, MAX_CHUNK_SIZE};
use byteorder::{ReadBytesExt, BigEndian};

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
        let chunks;
        let size;

        // let bytes = self.recv().unwrap();
        if let Packet::Header { filename, total_size, chunk_count, } = self.recv_header().unwrap() {
            log::info!("Receiving file \"{}\"", filename);

            chunks = chunk_count;
            size = total_size;

            if self.output_path.is_none() {
                self.output_path = Some(PathBuf::from(format!("./{filename}")));
            }
        } else {
            log::error!("No header packet sent, aborting");
            return;
        }

        let mut buf = vec![0u8; size];
        for i in 0..chunks {
            let start = i * MAX_CHUNK_SIZE;
            let end = usize::min((i + 1) * MAX_CHUNK_SIZE, size);
            let chunk = self.recv_body_chunk().unwrap();
            if let Packet::Content(bytes) = chunk {
                buf[start..end].copy_from_slice(&bytes);
            } else {
                log::error!("Non-content packet received, aborting");
                return;
            }
        }
        let mut file = std::fs::File::create_new(self.output_path.as_ref().unwrap()).unwrap();
        file.write_all(&buf).unwrap();
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
