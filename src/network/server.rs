use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::{Path, PathBuf};
use std::thread::JoinHandle;
use serde::Serialize;
use byteorder::{WriteBytesExt, BigEndian};

use crate::ARGS;
use crate::Error;
use crate::network::packet::Packet;

use super::packet::{Content, Header, MAX_CHUNK_SIZE};


pub struct Server {
    listener: TcpListener,
    client: Option<TcpStream>,
    path: PathBuf,
}
impl Server {
    pub fn new(path: PathBuf) -> Result<Self, Error> {
        return Ok(Self {
            listener: listener()?,
            client: None,
            path,
        });
    }
}

impl Server {
    pub fn main(&mut self) {
        let path = self.path.clone();
        let packets = std::thread::spawn(|| serialise_packets(make_packets(path)));

        loop {
            if self.accept() { break; }
        }

        let packets = packets.join().unwrap();

        for packet in &packets {
            self.send(packet);
        }
    }

    fn send(&mut self, bytes: &[u8]) {
        let len = bytes.len() as u32;
        let client = self.client.as_mut().unwrap();
        client.write_u32::<BigEndian>(len).unwrap();
        log::debug!("Sending {len} bytes");

        self.client.as_mut().unwrap().write_all(bytes).unwrap();
    }

    fn accept(&mut self) -> bool {
        if let Ok((sock, addr)) = self.listener.accept() {
            // TODO: confirmation
            log::info!("client {addr:?} connected");
            self.client = Some(sock);
            return true;
        } else {
            return false;
        }
    }
}

fn make_packets(path: impl AsRef<Path>) -> Vec<Packet> {
    let filename = path.as_ref().file_name().unwrap().to_str().unwrap().into();

    let mut file = std::fs::File::open(path).unwrap();
    let mut bytes = vec![];
    file.read_to_end(&mut bytes).unwrap();

    let chunk_count = {
        let r = bytes.len() % MAX_CHUNK_SIZE;
        let c = bytes.len() / MAX_CHUNK_SIZE;

        if r > 0 { c + 1 } else { c }
    };

    let header = Packet::Header(Header {
        chunk_count,
        total_size: bytes.len(),
        filename,
    });

    let mut out = Vec::with_capacity(chunk_count + 1);
    out.push(header);

    let mut bytes = bytes.into_iter();
    
    for i in 0..chunk_count {
        let b = (&mut bytes).take(MAX_CHUNK_SIZE).collect::<Vec<u8>>();
        out.push(Packet::Content(Content {
            index: i,
            bytes: b,
        }));
    }

    return out;
}

fn serialise_packets(packets: Vec<Packet>) -> Vec<Vec<u8>> {
    return packets.into_iter()
        .map(|p| rmp_serde::to_vec(&p).unwrap())
        .collect::<Vec<Vec<u8>>>();
}

pub fn listener() -> Result<TcpListener, std::io::Error> {
    let l = match TcpListener::bind(("0.0.0.0", ARGS.port)) {
        Ok(s) => s,
        Err(e) => {
            log::error!("[server] failed to bind to port {} with error {e:?}", ARGS.port);
            return Err(e);
        }
    };

    return Ok(l);
}
