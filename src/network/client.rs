use std::{io::{Read, Write}, net::{TcpStream, ToSocketAddrs}, path::{Path, PathBuf}};
use crate::{args::ReceiveArgs, Error, ARGS};
use super::packet::Packet;
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
        // let bytes = self.recv().unwrap();
        if let Packet::Header { filename, total_size, chunk_count, } = self.recv_header().unwrap() {
            log::info!("Receiving file \"{}\"", filename);
            if self.output_path.is_none() {
                self.output_path = Some(PathBuf::from(format!("./{filename}")));
            }
        } else {
            log::warn!("No header packet sent, aborting");
            return;
        }

        if let Packet::Content(bytes) = self.recv_body_chunk().unwrap() {
            // TODO: file splitting and multiple chunks
            let mut file = std::fs::File::create_new(self.output_path.as_ref().unwrap()).unwrap();
            file.write_all(&bytes).unwrap();
        }
    }

    fn recv_header(&mut self) -> Result<Packet, Error> {
        let bytes = self.recv()?;
        let packet = rmp_serde::from_slice(&bytes).unwrap();
        return Ok(packet);
    }

    // TODO: return a packet
    fn recv_body_chunk(&mut self) -> Result<Packet, Error> {
        // TODO: size
        std::thread::sleep(std::time::Duration::from_secs(1));
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
